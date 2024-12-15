use nrf52840_hal as hal;
use nrf52840_pac::interrupt;
use nrf52840_hal::gpiote::Gpiote;
use embedded_hal::digital::InputPin;
use ds323x::{DateTimeAccess, Ds323x, Datelike, NaiveDateTime, NaiveDate, Rtcc};
use ds323x::{DayAlarm2, WeekdayAlarm1, Hours, Alarm1Matching, Alarm2Matching};
use ds323x::interface::I2cInterface;
use ds323x::ic::DS3231;
use rtt_target::rprintln;
use ds323x::Timelike;
use heapless::String;

use cortex_m::peripheral::NVIC;
use cortex_m::interrupt::Mutex;
use cortex_m::interrupt::free;
use core::cell::RefCell;
use core::fmt::Write;
use circular_buffer::CircularBuffer;
use core::ops::DerefMut;

use crate::io::{Io, IoPins, IntData, Event};

static INTDATA: Mutex<RefCell<Option<IntData>>> = Mutex::new(RefCell::new(None));
const MONTHS: [&str; 12] = ["jan", "feb", "mar", "apr" , "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];
const DAYS: [&str; 7]    = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

macro_rules! get_intdata {($cs:tt) => (INTDATA.borrow($cs).borrow_mut().deref_mut())}

#[interrupt]
fn GPIOTE() 
{
    free(|cs| {
        if let Some(ref mut int_data) = get_intdata!(cs) 
        {
            if int_data.alarm.is_low().unwrap() {int_data.buffer.push_back(check_rtc(&mut int_data.rtc));}
            if int_data.btn_up.is_low().unwrap() {int_data.buffer.push_back(Event::BtnUp);}
            if int_data.btn_mid.is_low().unwrap() {int_data.buffer.push_back(Event::BtnMid);}
            if int_data.btn_dwn.is_low().unwrap() {int_data.buffer.push_back(Event::BtnDown);}

            // quick & dirty debounce
            cortex_m::asm::delay(100_000);
            int_data.gpiote.reset_events();
        }
    });
}

fn check_rtc(rtc: &mut Ds323x<I2cInterface<hal::Twim<nrf52840_hal::pac::TWIM0>>, DS3231>) -> Event
{
    match rtc.has_alarm1_matched()
    {
        Ok(x) =>
        {
            if x
            {
                let _ = rtc.clear_alarm1_matched_flag();
                return Event::Alarm;
            }
        }
        Err(x) => {rprintln!("Error communicating with rtc: {:?}", x);}
    }

    match rtc.has_alarm2_matched()
    {
        Ok(x) =>
        {
            if x
            {
                let _ = rtc.clear_alarm2_matched_flag();
                return Event::Minute;
            }
        }
        Err(x) => {rprintln!("Error communicating with rtc: {:?}", x);}
    }

    return Event::None;
}

////////////////////////////////////////////

impl Io
{
    pub fn new(twi: nrf52840_hal::pac::TWIM0, gpiote: Gpiote, pins: IoPins) -> Io
    {
        let mut buffer = CircularBuffer::<5, Event>::new();
        let mut twim = hal::Twim::new(
            twi,
            hal::twim::Pins {sda: pins.sda, scl: pins.scl},
            hal::twim::Frequency::K100,
        );
        twim.enable();
    
        let mut rtc = Ds323x::new_ds3231(twim);

        let alarm = DayAlarm2{day: 1, hour: Hours::H24(1), minute: 1};
        rtc.set_alarm2_day(alarm, Alarm2Matching::OncePerMinute).unwrap();
        rtc.use_int_sqw_output_as_interrupt().unwrap();
        rtc.enable_alarm2_interrupts().unwrap();

        NVIC::unpend(interrupt::GPIOTE);
        unsafe { NVIC::unmask(interrupt::GPIOTE) };

        let data = IntData
        {
            rtc: rtc,
            gpiote: gpiote,
            buffer: buffer,
            alarm: pins.alarm,
            btn_up: pins.btn_up,
            btn_mid: pins.btn_mid,
            btn_dwn: pins.btn_dwn, 
        };

        cortex_m::interrupt::free(|cs| {
            INTDATA.borrow(cs).replace(Some(data));
        });


        return Io {};
    }

    ////////////////////////////////////////////

    pub fn wait_for_input(&mut self) -> Event
    {
        let mut ev = None;

        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut int_data) = INTDATA.borrow(cs).borrow_mut( ).deref_mut( ) 
            {
                ev = int_data.buffer.pop_front();
            }
        });
        
        if ev.is_some() {return ev.unwrap();}
        rprintln!("wfi");
        cortex_m::asm::wfi();
        
        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut int_data) = INTDATA.borrow(cs).borrow_mut( ).deref_mut( ) 
            {
                ev = int_data.buffer.pop_front();
            }
        });

        if ev.is_none() {return Event::None;}
        // if ev.unwrap() == Event::Rtc {ev = Some(self.check_rtc());}
        return ev.unwrap();
    }    

    ////////////////////////////////////////////

    fn get_datetime(&mut self) -> NaiveDateTime
    {
        let mut dt = NaiveDateTime::from_timestamp(0, 0);
        free(|cs| {
            if let Some(ref mut int_data) = get_intdata!(cs) 
            {
                dt = int_data.rtc.datetime().unwrap();
            }
        });
        return dt;
    }

    pub fn set_datetime(&mut self, dy: i32, dm: u32, dd: u32, h: u32, m: u32) -> bool
    {
        let datetime = NaiveDate::from_ymd(dy, dm, dd).and_hms(h, m, 0);
        let mut result = false;

        free(|cs| {
            if let Some(ref mut int_data) = get_intdata!(cs) 
            {
                result = int_data.rtc.set_datetime(&datetime).is_ok();
            }
        });

        return result;
    }

    pub fn get_time_str(&mut self) -> String<32>
    {
        let t = self.get_datetime();
        let h = t.hour();
        let m = t.minute();

        let mut out = String::<32>::new();
        let _ = write!(out, "{:02}:{:02}", h, m);
        return out;
    }

    pub fn get_date_str(&mut self) -> String<32>
    {
        let t = self.get_datetime();
        let m = self::MONTHS[t.month0() as usize];
        let dow = self::DAYS[t.weekday().num_days_from_monday() as usize];
        let d = t.day();

        let mut out = String::<32>::new();
        let _ = write!(out, "{} {} {}", dow, d, m);
        return out;
    }

    pub fn set_alarm(&mut self, d: u8, h: u8, m: u8)
    {
        let alarm = WeekdayAlarm1
        {
            weekday:d, 
            hour: Hours::H24(h), 
            minute: m,
            second: 0,
        };
        
        free(|cs| {
            if let Some(ref mut int_data) = get_intdata!(cs) 
            {
                int_data.rtc.set_alarm1_weekday(alarm, Alarm1Matching::AllMatch).unwrap();
                let _ = int_data.rtc.enable_alarm1_interrupts();
            }
        });
    }

    pub fn disable_alarm(&mut self)
    {
        free(|cs| {
            if let Some(ref mut int_data) = get_intdata!(cs) 
            {
                let _ = int_data.rtc.disable_alarm1_interrupts();
            }
        });
    }
}
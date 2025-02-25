use nrf52832_hal as hal;
use nrf52832_pac::interrupt;
use nrf52832_hal::gpiote::Gpiote;
use embedded_hal::digital::InputPin;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use ds323x::{DateTimeAccess, Ds323x, Datelike, Timelike};
use ds323x::{DayAlarm2, WeekdayAlarm1, Hours, Alarm1Matching, Alarm2Matching};
use ds323x::interface::I2cInterface;
use ds323x::ic::DS3231;
use rtt_target::rprintln;
use heapless::String;

use cortex_m::peripheral::NVIC;
use cortex_m::interrupt::{Mutex, free};
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
            // TODO: make this not suck ass
            cortex_m::asm::delay(100_000);
            rprintln!("ev");
            int_data.gpiote.reset_events();
        }
    });
}

fn check_rtc(rtc: &mut Ds323x<I2cInterface<hal::Twim<nrf52832_hal::pac::TWIM0>>, DS3231>) -> Event
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

    return Event::NoEvent;
}

////////////////////////////////////////////

impl Io
{
    pub fn new(twi: nrf52832_hal::pac::TWIM0, gpiote: Gpiote, pins: IoPins) -> Io
    {
        let buffer = CircularBuffer::<5, Event>::new();
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
        let _ = rtc.clear_alarm1_matched_flag();
        let _ = rtc.clear_alarm2_matched_flag();
        rtc.enable_alarm2_interrupts().unwrap();

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

        NVIC::unpend(interrupt::GPIOTE);
        unsafe { NVIC::unmask(interrupt::GPIOTE) };

        return Io {};
    }

    ////////////////////////////////////////////

    pub fn wait_for_input(&mut self) -> Event
    {
        let mut ev = None;
        while ev == None
        {
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
        }

        return ev.unwrap();
    }    

    pub fn get_input_waitms(&mut self, delay_ms: u32) -> Event
    {
         //TODO: less powerhungry/better wait
        cortex_m::asm::delay(64_000 * delay_ms);
        let mut ev = None; 
        
        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut int_data) = INTDATA.borrow(cs).borrow_mut( ).deref_mut( ) 
            {
                ev = int_data.buffer.pop_front();
            }
        });
        
        return ev.unwrap_or(Event::NoEvent);
    }

    pub fn buffer_has_ev(&self) -> bool
    {
        let mut has_ev = false;

        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut int_data) = INTDATA.borrow(cs).borrow_mut( ).deref_mut( ) 
            {
                has_ev = int_data.buffer.front().is_some();
            }
        });

        return has_ev;
    }

    ////////////////////////////////////////////

    pub fn get_datetime(&mut self) -> NaiveDateTime
    {

        let nd = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let nt = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let mut dt = NaiveDateTime::new(nd, nt);

        free(|cs| {
            if let Some(ref mut int_data) = get_intdata!(cs) 
            {
                dt = int_data.rtc.datetime().unwrap();
            }
        });
        return dt;
    }

    pub fn set_datetime(&mut self, dy: u16, dm: u8, dd: u8, h: u8, m: u8) -> bool
    {
        let date_opt = NaiveDate::from_ymd_opt(dy as i32, dm as u32, dd as u32);
        let time_opt = NaiveTime::from_hms_opt(h as u32, m as u32, 0);
        if date_opt.is_none() || time_opt.is_none() {return false;} 

        let dt = NaiveDateTime::new(date_opt.unwrap(), time_opt.unwrap());
        let mut result = false;

        free(|cs| {
            if let Some(ref mut int_data) = get_intdata!(cs) 
            {
                result = int_data.rtc.set_datetime(&dt).is_ok();
                let _ = int_data.rtc.clear_alarm1_matched_flag();
                let _ = int_data.rtc.clear_alarm2_matched_flag();
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

    pub fn get_dow_str(&mut self) -> String<32>
    {
        let t = self.get_datetime();
        let dow = self::DAYS[t.weekday().num_days_from_monday() as usize];

        let mut out = String::<32>::new();
        let _ = write!(out, "{}", dow);
        return out;
    }

    pub fn get_date_str(&mut self) -> String<32>
    {
        let t = self.get_datetime();
        let m = self::MONTHS[t.month0() as usize];
        let d = t.day();

        let mut out = String::<32>::new();
        let _ = write!(out, "{} {}", d, m);
        return out;
    }

    ////////////////////////////////////////////

    pub fn set_alarm(&mut self, d: u8, h: u8, m: u8) -> bool
    {
        let mut result = false;
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
                result = int_data.rtc.set_alarm1_weekday(alarm, Alarm1Matching::AllMatch).is_ok();
                if result {let _ = int_data.rtc.enable_alarm1_interrupts();}
            }
        });
        
        return result;
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

    ////////////////////////////////////////////

    pub fn play_tone(&mut self)
    {
        // TODO: play 1 second tone signal
    }
}
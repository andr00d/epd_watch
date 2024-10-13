use nrf52840_hal as hal;
use nrf52840_pac::interrupt;
use nrf52840_hal::gpiote::Gpiote;
use ds323x::{DateTimeAccess, Ds323x, NaiveDate, Rtcc};
use rtt_target::rprintln;
use ds323x::Timelike;
use heapless::String;

use cortex_m::peripheral::NVIC;
use cortex_m::interrupt::Mutex;
use core::cell::RefCell;
use core::fmt::Write;
// use circular_buffer::CircularBuffer;

use crate::io::{Io, IoPins, IntData, Event};

static INTDATA: Mutex<RefCell<Option<IntData>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn GPIOTE() 
{
    cortex_m::interrupt::free(|cs| {
        let intData = INTDATA.borrow(cs).borrow();
        if let Some(intData) = intData.as_ref() 
        {
            intData.gpiote.reset_events();
        }
    });

    rprintln!("int test");
}

impl Io
{
    pub fn new(twi: nrf52840_hal::pac::TWIM0, gpiote: Gpiote, pins: IoPins) -> Io
    {
        // let buffer: [Event; 10] = [Event::None; 10];
        let mut twim = hal::Twim::new(
            twi,
            hal::twim::Pins {sda: pins.sda, scl: pins.scl},
            hal::twim::Frequency::K100,
        );
        twim.enable();
    
        let mut rtc = Ds323x::new_ds3231(twim);
        let datetime = NaiveDate::from_ymd_opt(2024, 10, 10)
            .unwrap()
            .and_hms_opt(12, 00, 00)
            .unwrap();

        let test = rtc.set_datetime(&datetime);
        match test
        {
            Ok(()) => (),
            Err(x) => rprintln!("err: {:?}", x),
        };

        NVIC::unpend(interrupt::GPIOTE);
        unsafe { NVIC::unmask(interrupt::GPIOTE) };

        let data = IntData
        {
            gpiote: gpiote,
            alarm: pins.alarm,
            btn_up: pins.btn_up,
            btn_mid: pins.btn_mid,
            btn_dwn: pins.btn_dwn, 
        };

        cortex_m::interrupt::free(|cs| {
            INTDATA.borrow(cs).replace(Some(data));
        });

        return Io
        {
            rtc: rtc
        };
    }

    pub fn wait_for_input(&self)
    {
        
    }    

    pub fn get_time_str(&mut self) -> String<32>
    {
        let t = self.rtc.time().unwrap();
        let h = t.hour();
        let m = t.minute();
        let s = t.second();

        let mut out = String::<32>::new();
        let _ = write!(out, "{h}:{m}:{s}");
        return out;
    }
}
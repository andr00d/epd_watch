use nrf52840_hal as hal;
use nrf52840_hal::gpio::p0;
use nrf52840_hal::Twim;

use crate::io::{Io, Io_pins, Event};

impl Io
{
    pub fn new(twi: nrf52840_hal::pac::TWIM0, pins: Io_pins) -> Io
    {
        let buffer: [Event; 10] = [Event::None; 10];
        
        let sda = pins.sda.into_floating_input();
        let scl = pins.scl.into_floating_input();
        
        let mut twim = hal::Twim::new(
            twi,
            hal::twim::Pins {
                sda: sda,
                scl: scl,
            },
            hal::twim::Frequency::K100,
        );
        twim.enable();
    
        return Io
        {
            event_buffer: buffer,
            twim: twim,
        };
    }

    pub fn check_input(&self)
    {

    }    
}
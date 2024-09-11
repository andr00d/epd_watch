pub mod io;

use nrf52840_hal as hal;
use nrf52840_hal::gpio::Pin;
use nrf52840_hal::gpio::Disconnected;
use nrf52840_hal::gpio::{Input, Output, Floating, PushPull};

#[derive(Copy)]
#[derive(Clone)]
pub enum Event
{
    None,
    Alarm,
    Minute,
    BtnUp,
    BtnMid,
    BtnDown
}

pub struct Io_pins
{
    pub sda: Pin<Disconnected>,
    pub scl: Pin<Disconnected>,
}

pub struct Io
{
    event_buffer: [Event; 10],
    twim: hal::Twim<nrf52840_hal::pac::TWIM0>,
}


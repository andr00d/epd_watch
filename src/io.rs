pub mod io;

use nrf52840_hal as hal;
use nrf52840_hal::gpio::Pin;
use nrf52840_hal::gpio::{PullUp, Input, Floating};
use ds323x::{Ds323x};
use nrf52840_hal::gpiote::Gpiote;
use ds323x::ic::DS3231;
use ds323x::interface::I2cInterface;

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

pub struct IoPins
{
    pub scl: Pin<Input<Floating>>,
    pub sda: Pin<Input<Floating>>,
    pub alarm: Pin<Input<PullUp>>,
    pub btn_up: Pin<Input<PullUp>>,
    pub btn_mid: Pin<Input<PullUp>>,
    pub btn_dwn: Pin<Input<PullUp>>,
}

struct IntData
{
    pub gpiote: Gpiote,
    pub alarm: Pin<Input<PullUp>>,
    pub btn_up: Pin<Input<PullUp>>,
    pub btn_mid: Pin<Input<PullUp>>,
    pub btn_dwn: Pin<Input<PullUp>>,
}

pub struct Io
{
    rtc: Ds323x<I2cInterface<hal::Twim<nrf52840_hal::pac::TWIM0>>, DS3231>,
}


pub mod io;
mod sound;

use nrf52832_hal as hal;
use nrf52832_hal::gpio::Pin;
use nrf52832_hal::gpio::{PullUp, Input, Floating};
use nrf52832_hal::gpiote::Gpiote;
use nrf52832_hal::rtc::Rtc;
use nrf52832_hal::pwm::Pwm;
use nrf52832_pac::{PWM0, RTC0, RTC1, RTC2};
use ds323x::ic::DS3231;
use ds323x::Ds323x;
use ds323x::interface::I2cInterface;
use circular_buffer::CircularBuffer;

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Event
{
    NoEvent,
    Minute,
    Alarm,
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
    rtc: Ds323x<I2cInterface<hal::Twim<nrf52832_hal::pac::TWIM0>>, DS3231>,
    pub gpiote: Gpiote,
    pub rtc0: Rtc<RTC0>,
    pub timer_expired: bool,
    pub buffer: CircularBuffer::<5, Event>,
    pub alarm: Pin<Input<PullUp>>,
    pub btn_up: Pin<Input<PullUp>>,
    pub btn_mid: Pin<Input<PullUp>>,
    pub btn_dwn: Pin<Input<PullUp>>,

    pub rtc1: Rtc<RTC1>,
    pub rtc2: Rtc<RTC2>,
    pub timer2_expired: bool,
    pub pwm: Pwm<PWM0>,
    pub curr_note: usize,
}

pub struct Io {}

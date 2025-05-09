mod display;
pub mod shape;
pub mod font;

use nrf52832_hal::spim::Spim;
use nrf52832_hal::gpio::Pin;
use nrf52832_hal::gpio::Disconnected;
use nrf52832_hal::gpio::{Input, Output, Floating, PushPull};


const SIZE: usize = 200;
const BUFFSIZE: usize = SIZE*((SIZE+7)/8);

pub struct DispPins
{
    pub power: Pin<Disconnected>,
    pub clk: Pin<Disconnected>,
    pub miso: Pin<Disconnected>,
    pub mosi: Pin<Disconnected>,
    pub busy: Pin<Disconnected>,
    pub res: Pin<Disconnected>,
    pub cs: Pin<Disconnected>,
    pub dc: Pin<Disconnected>,
}

pub struct Display
{
    buffer_curr: [u8; BUFFSIZE],
    buffer_old: [u8; BUFFSIZE],
    // power: Pin<Output<PushPull>>,
    spi: Spim<nrf52832_hal::pac::SPIM2>,
    busy: Pin<Input<Floating>>,
    res: Pin<Output<PushPull>>,
    cs: Pin<Output<PushPull>>,
    dc: Pin<Output<PushPull>>,
    sleeping: bool,
    clean_update: bool,
}

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use hal::pac;
use nrf52840_hal as hal;
use panic_halt as _;
use rtt_target::{rtt_init_print, rprintln};
use nrf52840_hal::gpiote::Gpiote;

use crate::display::DispPins;
use crate::io::IoPins;
use crate::display::Display;
use crate::io::Io;

mod display;
mod io;
mod pages;

fn connect_parts() -> (Display, Io)
{
    let p = pac::Peripherals::take().unwrap();
    let p0 = hal::gpio::p0::Parts::new(p.P0);
    let gpiote = Gpiote::new(p.GPIOTE);

    let disp_pins = DispPins 
    {
        power: p0.p0_08.degrade(),
        clk: p0.p0_06.degrade(),
        miso:  p0.p0_14.degrade(),
        mosi: p0.p0_07.degrade(),
        busy: p0.p0_26.degrade(),
        res: p0.p0_27.degrade(),
        cs: p0.p0_05.degrade(),
        dc: p0.p0_04.degrade(),
    };

    let io_pins = IoPins
    {
        scl: p0.p0_22.into_floating_input().degrade(),
        sda: p0.p0_23.into_floating_input().degrade(),
        alarm : p0.p0_24.into_pullup_input().degrade(),
        btn_up : p0.p0_17.into_pullup_input().degrade(),
        btn_mid : p0.p0_15.into_pullup_input().degrade(),
        btn_dwn : p0.p0_13.into_pullup_input().degrade(),
    };
    
    gpiote.port().input_pin(&io_pins.alarm).low();
    gpiote.port().input_pin(&io_pins.btn_up).low();
    gpiote.port().input_pin(&io_pins.btn_mid).low();
    gpiote.port().input_pin(&io_pins.btn_dwn).low();
    gpiote.port().enable_interrupt();

    let mut display = Display::new(p.SPIM2, disp_pins);
    let io = Io::new(p.TWIM0, gpiote, io_pins);
    return (display, io);
}

#[entry]
fn main() -> ! 
{
    rtt_init_print!();

    let (mut display, io) = connect_parts();
    // display.init();
    rprintln!("finished booting");

    let mut text_flip = false;

    loop 
    {
        cortex_m::asm::delay(6_400_000);
        // if text_flip
        // {
        //     display.text("test", 64, 10, 5);
        // }
        // else
        // {
        //     display.text("test", 64, 50, 5);
        // }

        // display.update();
        text_flip = !text_flip;
        // pages.update();
    }
}

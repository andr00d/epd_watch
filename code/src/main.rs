#![no_main]
#![no_std]

use cortex_m_rt::entry;
use hal::pac;
use nrf52832_hal as hal;
use rtt_target::{rtt_init_print, rprintln};
use nrf52832_hal::gpiote::Gpiote;

use crate::display::DispPins;
use crate::io::{Io, IoPins, Event};
use crate::shared_data::SharedData;
use crate::display::Display;
use crate::pages::Pages;


mod shared_data;
mod display;
mod io;
mod pages;

pub const NFCPINS: *mut u32 = 0x1000120C as *mut u32;

fn connect_parts() -> (Display, Io)
{
    let p = pac::Peripherals::take().unwrap();

    // i have to add this because im stupid and used NFC pins.
    unsafe
    {
        let curr_val = NFCPINS.read_volatile();
        if curr_val & 1 as u32 != 0x0
        {
            p.NVMC.config.write(|w| w.wen().wen());
            NFCPINS.write_volatile(0 as u32 | !(1 as u32));
            p.NVMC.config.write(|w| w.wen().ren());
            rprintln!("set NFC pins to GPIO");
        }
        else {rprintln!("NFC pins already set to GPIO");}
    }

    let p0 = hal::gpio::p0::Parts::new(p.P0);
    let gpiote = Gpiote::new(p.GPIOTE);

    let disp_pins = DispPins 
    {
        power: p0.p0_25.degrade(),
        clk: p0.p0_30.degrade(),
        miso:  p0.p0_24.degrade(),
        mosi: p0.p0_31.degrade(),
        busy: p0.p0_26.degrade(),
        res: p0.p0_27.degrade(),
        cs: p0.p0_29.degrade(),
        dc: p0.p0_28.degrade(),
    };

    let io_pins = IoPins
    {
        scl: p0.p0_12.into_floating_input().degrade(),
        sda: p0.p0_11.into_floating_input().degrade(),
        alarm : p0.p0_10.into_pullup_input().degrade(),
        btn_up : p0.p0_09.into_pullup_input().degrade(),
        btn_mid : p0.p0_08.into_pullup_input().degrade(),
        btn_dwn : p0.p0_07.into_pullup_input().degrade(),
    };
    
    gpiote.port().input_pin(&io_pins.alarm).low();
    gpiote.port().input_pin(&io_pins.btn_up).low();
    gpiote.port().input_pin(&io_pins.btn_mid).low();
    gpiote.port().input_pin(&io_pins.btn_dwn).low();
    gpiote.port().enable_interrupt();

    let mut display = Display::new(p.SPIM2, disp_pins);
    let io = Io::new(p.TWIM0, gpiote, io_pins);
    display.init();
    return (display, io);
}


#[entry]
fn main() -> ! 
{
    rtt_init_print!();

    let (mut display, mut io) = connect_parts();
    let mut shared = SharedData::new(&mut display, &mut io);
    let mut pages = Pages::new();

    pages.update_page(Event::Minute, &mut shared);
    shared.display.update();
    rprintln!("startup done");

    loop 
    {
        let ev = shared.io.wait_for_input();
        if ev == Event::NoEvent {continue;}

        pages.update_page(ev, &mut shared);
        if shared.update {shared.display.update()};
    }
}

use core::panic::PanicInfo;
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    rprintln!("err!: {}", info);
    loop {}
}

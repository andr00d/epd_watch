#![cfg_attr(target_os = "none", no_main)]
#![cfg_attr(target_os = "none", no_std)]
#[cfg(target_os = "none")]
use 
{
    cortex_m_rt::entry,
    hal::pac,
    nrf52840_hal as hal,
    panic_halt as _,
    rtt_target::rtt_init_print,
    rtt_target::rprintln,
    crate::display::Disp_pins,
    crate::io::Io_pins,
    embedded_hal::digital::OutputPin,
    embedded_hal::digital::PinState,
    cortex_m::asm::nop,
};

#[cfg(target_os = "linux")]
macro_rules! rprintln {($fmt:expr $(, $($arg:tt)*)?) => {println!($fmt, $($($arg)*)?);};}

#[cfg_attr(target_os = "linux", path = "pc_test/display.rs")]
mod display;

#[cfg_attr(target_os = "linux", path = "pc_test/io.rs")]
mod io;

mod pages;

use crate::display::Display;
use crate::io::Io;


#[cfg(target_os = "linux")]
fn connect_parts() -> (Display, Io)
{
    let mut display = Display::new();
    display.init();
    let io = Io::new();
    return (display, io);
}

#[cfg(target_os = "none")]
fn connect_parts() -> (Display, Io)
{
    let p = pac::Peripherals::take().unwrap();
    let p0 = hal::gpio::p0::Parts::new(p.P0);
    
    let disp_pins = Disp_pins 
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

    let io_pins = Io_pins
    {
        sda: p0.p0_21.degrade(),
        scl: p0.p0_22.degrade(),
    };
    
    let mut display = Display::new(p.SPIM2, disp_pins);
    let io = Io::new(p.TWIM0, io_pins);

    display.init();

    return (display, io);
}

#[cfg_attr(target_os = "none", entry)]
fn main() -> ! 
{
    #[cfg(target_os = "none")]
    rtt_init_print!();

    let (mut display, io) = connect_parts();
    
    // let pages = Pages::new();

    // display.square(10, 10, 20, 20);
    display.text("test", 64, 20, 5);
    display.update();

    loop 
    {
        io.check_input();
        // pages.update();
    }
}

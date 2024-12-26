macro_rules! rprintln {($fmt:expr $(, $($arg:tt)*)?) => {println!($fmt, $($($arg)*)?)};}

#[path = "../../src/shared_data.rs"]
mod shared_data;
mod display;
mod io;
mod pages;

use crate::display::Display;
use crate::io::{Io, Event};
use crate::pages::Pages;
use crate::shared_data::SharedData;

fn connect_parts() -> (Display, Io)
{
    let mut display = Display::new();
    display.init();
    let io = Io::new();
    return (display, io);
}

fn main()
{
    let (mut display, mut io) = connect_parts();
    let mut shared = SharedData::new(&mut display, &mut io);
    let mut pages = Pages::new();

    pages.update_page(Event::Minute, &mut shared);
    shared.display.update();

    loop 
    {
        let ev = shared.io.wait_for_input();
        if ev == Event::NoEvent {continue;}

        pages.update_page(ev, &mut shared);
        shared.display.update();
    }
}

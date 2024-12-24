#[path = "../sharedData.rs"]
mod sharedData;
mod display;
mod io;
mod pages;

use crate::display::Display;
use crate::io::{Io, Event};
use crate::pages::Pages;
use crate::sharedData::SharedData;


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
    display.update();

    loop 
    {
        let ev = io.wait_for_input();
        if ev == Event::None {continue;}

        // pages.update_page(ev);
        display.update();
    }
}

// #[path = "../src/pc_test/display.rs"]
mod display;

// #[path = "../src/pc_test/io.rs"]
mod io;

// mod pages;

use crate::display::Display;
use crate::io::Io;


fn connect_parts() -> (Display, Io)
{
    let mut display = Display::new();
    display.init();
    let io = Io::new();
    return (display, io);
}

fn main()
{
    #[cfg(target_os = "none")]
    rtt_init_print!();

    let (mut display, io) = connect_parts();
    
    // let pages = Pages::new();

    // display.square(10, 10, 20, 20);
    display.update();

    println!("finished booting");

    let mut text_flip = false;

    loop 
    {
        io.wait_for_input();

        if text_flip
        {
            display.text("test", 64, 10, 5);
        }
        else
        {
            display.text("test", 64, 50, 5);
        }

        display.update();
        text_flip = !text_flip;
        // pages.update();
    }
}

#[cfg(target_os = "none")]
use rtt_target::rprintln;
#[cfg(target_os = "linux")]
macro_rules! rprintln {($fmt:expr $(, $($arg:tt)*)?) => {println!($fmt, $($($arg)*)?);};}

use crate::pages::{Pages, core_pages};
use crate::io::Event;

impl Pages<'_> 
{
    pub fn new() -> Pages<'static>
    {
        return Pages
        {
            curr_page: &Self::menu_time,
        };
    }

    pub fn update_page(&self, ev: Event)
    {
        // 
        match self.curr_page
        {
            menu_time => {},
            menu_settings => {},
            menu_timer => {},
            menu_stopwatch => {},
            _ => (),
        }
    }
}
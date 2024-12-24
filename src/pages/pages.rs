#[cfg(target_os = "none")]
use rtt_target::rprintln;
#[cfg(target_os = "linux")]
macro_rules! rprintln {($fmt:expr $(, $($arg:tt)*)?) => {println!($fmt, $($($arg)*)?);};}

use crate::pages::{Pages, core_pages};
use crate::sharedData::SharedData;
use crate::io::Event;

impl Pages
{
    pub fn new() -> Pages
    {
        return Pages
        {
            curr_page: Self::menu_time,
        };
    }

    fn sm_step(&mut self, pg_func: fn(&mut SharedData) -> (), trans_func: Option<fn()>)
    {
        // if trans_func.is_some() {(trans_func.unwrap())();}  
        self.curr_page = pg_func;
    }

    pub fn update_page(&mut self, ev: Event, data: &mut SharedData)
    {
        match self.curr_page
        {
            menu_time => { match ev {
                    Event::Minute  => {self.sm_step(menu_time,      None);},
                    // Event::Alarm   => {self.sm_step(pg_alarm,       None);},
                    // Event::BtnUp   => {self.sm_step(menu_stopwatch, None);},
                    // Event::BtnMid  => {self.sm_step(pg_alarm,       None);},
                    // Event::BtnDown => {self.sm_step(menu_settings,  None);},
                    _ => (),
            } },

            // menu_settings => {},
            // menu_timer => {},
            // menu_stopwatch => {},
            _ => (),
        }

        (self.curr_page)(data);
    }
}
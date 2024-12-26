#[cfg(target_os = "none")]
use rtt_target::rprintln;
// #[cfg(target_os = "linux")]

use crate::display::{Display, SIZE};

impl Display 
{
    pub fn square(&mut self, x: u8, y: u8, w: u8, h: u8)
    {
        let size = SIZE as u8;
        if x >= size || y >= size || x+w >= size || y+h >= size
        {
            rprintln!("square is drawn outside of bounds");
            return;
        }

        return;
    }
}
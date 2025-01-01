#[cfg(target_os = "none")]
use rtt_target::rprintln;

use crate::display::{Display, SIZE};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum ArrowDir
{
    Up,
    Right,
    Down,
    Left,
}

impl Display 
{
    pub fn rect(&mut self, x_in: u8, y_in: u8, w: u8, h: u8)
    {
        let size = SIZE as u8;
        if x_in >= size || y_in >= size || x_in+w > size || y_in+h > size
        {
            rprintln!("square is drawn outside of bounds");
            return;
        }

        // ignore optimizations for thin lines
        if w < 8
        {
            for y in y_in..(y_in+h) 
            {
                let offset = y as usize * (SIZE/8);
                for x in x_in..(x_in+w)
                { 
                    let buff_byte = offset + (x / 8) as usize;
                    let buff_bit = x % 8;
                    self.set_bit(buff_byte, buff_bit, false);
                } 
            }
            return;
        }

        for y in y_in..(y_in+h) 
        {
            let offset = y as usize * (SIZE/8);
            for i in ((x_in+7)/8)..((x_in+w)/8)
            {
                self.buffer_curr[offset + i as usize] = 0x0;
            }
            
            
            if x_in % 8 > 0 
            {
                let byte = (0xff << (8-(x_in % 8))) as u8;
                self.buffer_curr[offset + (x_in / 8) as usize] &= byte;
            }

            if (x_in+w) % 8 > 0 
            {
                let byte = (0xff >> (x_in+w) % 8) as u8;
                self.buffer_curr[offset + ((x_in+w)/8) as usize] &= byte ;
            }
        }

        return;
    }

    pub fn arrow(&mut self, x_in: u8, y_in: u8, s: u8, dir: ArrowDir)
    {
        // TODO: improve bounds check
        if x_in as usize >= SIZE || y_in as usize >= SIZE
        {
            rprintln!("arrow is drawn outside of bounds");
            return;
        }

        let height = if dir == ArrowDir::Up || dir == ArrowDir::Down {s} else {(s*2)-1};
    
        for y in 0..height 
        {
            let offset = (y+y_in) as usize * (SIZE/8);
            let (row_start, row_end) = match dir
            {
                ArrowDir::Up => 
                {
                    (s-y, s+1+y)
                },
                ArrowDir::Down =>
                {
                    (y, 2*s-1-y)
                },
                ArrowDir::Left =>
                {
                    let mut tmp = (s-1) as i16 - y as i16;
                    tmp = if tmp < 0 {-tmp} else {tmp};
                    (tmp as u8, s)
                },
                ArrowDir::Right =>
                {
                    let tmp = if y < s {y+1} else {2*s-1-y};
                    (0, tmp)
                }
            };

            for x in x_in+row_start..x_in+row_end
            {
                let buff_byte = offset + (x / 8) as usize;
                let buff_bit = x % 8;
                self.set_bit(buff_byte, buff_bit, false); 
            }
        }
 
    }
}
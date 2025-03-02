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

    // code based on https://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    pub fn triangle(&mut self, a: (u8, u8), b: (u8, u8), c: (u8, u8))
    {
        if a == b && b == c {return;}
        // v1 to v3 are sorted by height
        let mut v1;
        let mut v2;
        let mut v3;
        let mut v4;

        if a.1 <= b.1 && a.1 <= c.1 
        {
            v1 = (a.0 as f32, a.1 as f32);
            if b.1 < c.1 {v2 = (b.0 as f32, b.1 as f32); v3 = (c.0 as f32, c.1 as f32);}
            else {v2 = (c.0 as f32, c.1 as f32); v3 = (b.0 as f32, b.1 as f32);}
        }
        else if b.1 <= a.1 && b.1 <= c.1 
        {
            v1 = (b.0 as f32, b.1 as f32);
            if a.1 < c.1 {v2 = (a.0 as f32, a.1 as f32); v3 = (c.0 as f32, c.1 as f32);}
            else {v2 = (c.0 as f32, c.1 as f32); v3 = (a.0 as f32, a.1 as f32);}
        }
        else 
        {
            v1 = (c.0 as f32, c.1 as f32);
            if a.1 < b.1 {v2 = (a.0 as f32, a.1 as f32); v3 = (b.0 as f32, b.1 as f32);}
            else {v2 = (b.0 as f32, b.1 as f32); v3 = (a.0 as f32, a.1 as f32);}
        }

        v4 = (v1.0 + (((v2.1- v1.1) / (v3.1 - v1.1)) * (v3.0 - v1.0)), v2.1);
        if v4.0 < v2.0 {(v4, v2) = (v2, v4);}

        // dirty fix to prevent seeing the edges
        // its a 200X200 screen, accuraccy doesnt matter :)
        v1.1 -= 3.0;
        v2.0 -= 3.0;
        v3.1 += 3.0; 
        v4.0 += 3.0;


        // top tri
        let top_slope1 = (v2.0 - v1.0) / (v2.1 - v1.1);
        let top_slope2 = (v4.0 - v1.0) / (v4.1 - v1.1);

        let mut top_x1 = v1.0;
        let mut top_x2 = v1.0;

        for i in v1.1 as u8..v2.1 as u8
        {
            self.line(top_x1, top_x2, i);
            top_x1 += top_slope1;
            top_x2 += top_slope2;
        }

        //bottom tri
        let bottom_slope1 = (v3.0 - v2.0) / (v3.1 - v2.1);
        let bottom_slope2 = (v3.0 - v4.0) / (v3.1 - v4.1);

        let mut bottom_x1 = v3.0;
        let mut bottom_x2 = v3.0;

        for i in (v2.1 as u8..v3.1 as u8).rev()
        {
            self.line(bottom_x1, bottom_x2, i);
            bottom_x1 -= bottom_slope1;
            bottom_x2 -= bottom_slope2;
        }
    }

    // adapted version of rect to speed up video drawing a teensy bit.
    // also behaves better for animation than rect, didnt wanna find out why
    fn line(&mut self, x1_in: f32, x2_in: f32, y: u8)
    {
        if y > 199 {return;}
        let x1 = (x1_in as u8).min(200);
        let x2 = (x2_in as u8).min(200);
        let offset = y as usize * (SIZE/8);
        
        if (x2-x1) < 8
        {
            for x in x1..x2
            { 
                self.set_bit(offset + (x / 8) as usize, x % 8, false);
            } 
            return;
        }

        for i in ((x1+7)/8)..((x2)/8)
        {
            self.buffer_curr[offset + i as usize] = 0x0;
        }
        
        if x1 % 8 > 0 
        {
            let byte = (0xff << (8-(x1 % 8))) as u8;
            self.buffer_curr[offset + (x1 / 8) as usize] &= byte;
        }

        if (x2) % 8 > 0 
        {
            let byte = (0xff >> (x2) % 8) as u8;
            self.buffer_curr[offset + ((x2)/8) as usize] &= byte ;
        }

        return;
    }


}
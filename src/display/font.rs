#[cfg(target_os = "none")]
use rtt_target::rprintln;
#[cfg(target_os = "linux")]
macro_rules! rprintln {($fmt:expr $(, $($arg:tt)*)?) => {println!($fmt, $($($arg)*)?);};}

use crate::display::{Display, SIZE};

// Each bit represents one pixel. 1 = white, 0 = black
const FONT: [u8; 135] =
[
    0b01010101, 0b01010101,            // <unknown character>
    0b11111111, 0b11111111,            // <space>
    0b11011011, 0b01111101,            // !
    0b01011111, 0b11111111,            // "
    0b01000001, 0b00000101,            // #
    0b10100001, 0b10001011,            // $
    0b01011010, 0b10110101,            // %
    0b10101010, 0b10111001,            // &
    0b01111111, 0b11111111,            // '
    0b10101101, 0b10111011,            // (
    0b10111011, 0b01101011,            // )
    0b01010101, 0b01111111,            // *
    0b11110100, 0b01011111,            // +
    0b11111111, 0b11010111,            // ,
    0b11111100, 0b01111111,            // -
    0b11111111, 0b11111011,            // .
    0b11011010, 0b10110111,            // /

    0b00001001, 0b00100001,            // 0
    0b00000111, 0b11111111,            // 1
    0b00011000, 0b00110001,            // 2
    0b00011000, 0b01100001,            // 3
    0b01001000, 0b01101101,            // 4
    0b00001100, 0b01100001,            // 5
    0b00001100, 0b00100001,            // 6
    0b00011011, 0b01101101,            // 7
    0b00001000, 0b00100001,            // 8
    0b00001000, 0b01100001,            // 9

    0b10101111, 0b11111111,            // :
    0b11110111, 0b11010111,            // ;
    0b11010101, 0b11011101,            // <
    0b11100011, 0b10001111,            // =
    0b01110111, 0b01010111,            // >
    0b00011010, 0b11111011,            // ?
    0b11100001, 0b00011111,            // @

    0b00001000, 0b00100101,            // A
    0b00101000, 0b00100011,            // B
    0b00001101, 0b10110001,            // C
    0b00101001, 0b00100011,            // D
    0b00001100, 0b00110001,            // E
    0b00001100, 0b00110111,            // F
    0b10001101, 0b00100001,            // G
    0b01001000, 0b00100101,            // H
    0b00000111, 0b11111111,            // I
    0b10101010, 0b00111111,            // J
    0b01001000, 0b10100101,            // K
    0b01101101, 0b10110001,            // L
    0b10000100, 0b01100110, 0b01101111,// M
    0b00001001, 0b00100101,            // N
    0b00001001, 0b00100001,            // O
    0b00001000, 0b00110111,            // P
    0b00010101, 0b01010101, 0b00001111,// Q
    0b00001000, 0b10100101,            // R
    0b00001100, 0b01100001,            // S
    0b00010110, 0b11011011,            // T
    0b01001001, 0b00100001,            // U
    0b01001001, 0b00101011,            // V
    0b01100110, 0b01100100, 0b00001111,// W
    0b01001010, 0b10100101,            // X
    0b01001000, 0b01100011,            // Y
    0b00011010, 0b10110001,            // Z

    0b10010110, 0b11011001,            // [
    0b01101110, 0b11101101,            // \
    0b00110110, 0b11010011,            // ]
    0b10101011, 0b11111111,            // ^
    0b11111111, 0b11110001,            // _
    0b01101111, 0b11111111,            // `
];


impl Display 
{
    fn get_width(&self, item: u8) -> u8
    {
        let mut width: u8 = 3;

        if      item == 'I' as u8 || item == ':' as u8 || item == '\'' as u8 || item == '1' as u8 {width = 1;}
        else if item == 'J' as u8 || item == '`' as u8 {width = 2;}
        else if item == 'W' as u8 || item == 'Q' as u8 || item == 'M' as u8 {width = 4;}

        return width;
    }

    pub fn get_text_width(&self, text: &str) -> u8
    {
        let mut total: u8 = 0;

        for i in 0..text.len()
        {
            total += self.get_width(text.as_bytes()[i as usize]);
            total += 1; //spacing
        }

        total -= 1; // remove with extra spacing
        return total;
    }

    fn get_index(&self, item: u8) -> usize
    {
        let mut index = 0;
        let mut ltr = item;

        if ltr >= 'a' as u8 && ltr <= 'z' as u8 { ltr -= 32; }

        if ltr >= ' ' as u8 && ltr <= '`' as u8
        {
            index = (ltr - 31) * 2;
            
            if ltr > 'M' as u8 {index += 1;}
            if ltr > 'Q' as u8 {index += 1;}
            if ltr > 'W' as u8 {index += 1;}
        }

        return index.into();
    }

    ////////////////////////////////////
    
    // TODO: translate to upper case
    pub fn text(&mut self, text: &str, x: u8, y: u8, s: u8)
    {
        let mut width = self.get_text_width(text);
        let mut ltr_offset = 0;
        width = width * s;

        if usize::from(width as u16) > SIZE {return;}
        if usize::from(x as u16 + width as u16) > SIZE || 
           usize::from(y as u16 + s as u16) > SIZE {return;}

        for ltr in text.bytes()
        {
            let buff_index = y as usize * (SIZE/8);
            let char_width = self.get_width(ltr);
            let ltr_index = self.get_index(ltr);
            let mut ltr_curr_bit: u8 = 0;
            
            for h in 0..5
            {
                let buff_offset = (h*s) as usize * (SIZE/8);

                for b in 0..char_width
                {
                    let font_index = ltr_index + (ltr_curr_bit / 8) as usize;
                    let bit_val = self.get_bit(&FONT, font_index, ltr_curr_bit % 8);
                    ltr_curr_bit += 1;

                    for n in 0..s
                    {
                        let buff_byte = buff_index + buff_offset + ((ltr_offset+x+n+b*s) / 8) as usize;
                        let buff_bit = (ltr_offset+x+n+b*s) % 8;

                        self.set_bit(buff_byte, buff_bit, bit_val);
                    }
                }
            }

            let mut padding = s/2;
            if padding == 0 {padding = 1;}
            ltr_offset += char_width*s + padding;
        }

        // // fill in bars between layers
        for h in 0..5
        {
            let buff_width = (((x%8) + width + 7) / 8) as usize; 
            let mut ref_start = (y as usize * (SIZE/8)) + (x / 8) as usize;
            ref_start += (s as usize) * h * (SIZE/8);

            for n in 1..s
            {
                let n: usize = n.into();
                let buff_start = ref_start + (SIZE/8)*n;
                
                for i in 0..buff_width
                {
                    let i: usize = i.into();
                    self.buffer_curr[buff_start + i] = self.buffer_curr[ref_start + i];
                }
            }
        }
    }
}
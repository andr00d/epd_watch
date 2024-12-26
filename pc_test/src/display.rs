#[path = "../../src/display/shape.rs"]
mod shape;

#[path = "../../src/display/font.rs"]
pub mod font;

pub const SIZE: usize = 200;
pub const BUFFSIZE: usize = SIZE*((SIZE+7)/8);

pub struct Display 
{
    buffer_curr: [u8; BUFFSIZE],
}

///////////////////////////////////////
///////////////////////////////////////

impl Display 
{
    pub fn new() -> Display
    {
        let buffer_curr: [u8; BUFFSIZE] = [0xFF; BUFFSIZE];
        
        return Display{
            buffer_curr: buffer_curr,
        };
    }    
    pub fn init(&mut self)
    {
        // print!("{esc}c", esc = 27 as char);
        println!("###########################################################################");
        println!("simple display debugger");
        println!("###########################################################################");
    }

    pub fn sleep(&self)
    {

    }

    pub fn update(&mut self)
    {
        // uses the braille block characters to generate the screen
        // https://en.wikipedia.org/wiki/Braille_Patterns#Block
        let mut output = "".to_string();

        for y in (0..SIZE).step_by(4)
        {
            for x in (0..SIZE).step_by(2)
            {
                output.push(self.gen_character(&self.buffer_curr, x, y));
            }
            output.push('\n');
        }

        println!("{}", output);
        self.buffer_curr.fill(0xff);
    }
    
    ////////////////////////////////////
    // used a lot by other display functions.

    pub(super) fn set_bit(&mut self, index: usize, bit_index: u8, value: bool) 
    {
        let clr_mask = 0xff ^ (0x80 >> bit_index);
        let set_mask = ((value as u8) << 7) >> bit_index;
    
        self.buffer_curr[index] &= clr_mask;
        self.buffer_curr[index] |= set_mask;
        // println!("{:8b} - {:8b} => {:x}", clr_mask, set_mask, self.buffer_curr[index]);
    }

    pub(super) fn get_bit(&mut self, arr: &[u8], index: usize, bit_index: u8) -> bool
    {
        let mask = 0x80 >> bit_index;
        return (arr[index] & mask) > 0;
    }

    ////////////////////////////////////

    fn gen_character(&self, buff: &[u8; BUFFSIZE], x:usize, y:usize) -> char
    {
        let mut output: u8 = 0x0;
        let width : usize = (SIZE+7)/8;

        let curr_byte = y*width + (x/8);
        let mask = 0x80 >> (x as u8)%8;
        output |= ((buff[curr_byte] & mask) > 0) as u8;
        output |= (((buff[curr_byte+width] & mask) > 0) as u8) << 1;
        output |= (((buff[curr_byte+(width*2)] & mask) > 0) as u8) << 2;
        output |= (((buff[curr_byte+(width*3)] & mask) > 0) as u8) << 6;

        let curr_byte = y*width + ((x+1)/8);
        let mask = 0x80 >> ((x+1) as u8)%8;
        output |= (((buff[curr_byte] & mask) > 0) as u8) << 3;
        output |= (((buff[curr_byte+width] & mask) > 0) as u8) << 4;
        output |= (((buff[curr_byte+(width*2)] & mask) > 0) as u8) << 5;
        output |= (((buff[curr_byte+(width*3)] & mask) > 0) as u8) << 7;

        return char::from_u32((0x2800 + output as u32) as u32).unwrap();
    }
}
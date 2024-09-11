#[path = "../display/shape.rs"]
mod shape;

#[path = "../display/font.rs"]
mod font;

pub const SIZE: usize = 200;
pub const BUFFSIZE: usize = SIZE*((SIZE+7)/8);

pub enum Buffer 
{
    A,
    B,
}

pub struct Display 
{
    buffer_curr: [u8; BUFFSIZE],
    buffer_old: [u8; BUFFSIZE],
}

///////////////////////////////////////
///////////////////////////////////////

impl Display 
{
    pub fn new() -> Display
    {
        let buffer_curr: [u8; BUFFSIZE] = [0xFF; BUFFSIZE];
        let buffer_old: [u8; BUFFSIZE] = [0xff; BUFFSIZE];
        let width = (SIZE+7)/8;
        
        return Display{
            buffer_curr: buffer_curr,
            buffer_old: buffer_old,
        };
    }    
    pub fn init(&mut self)
    {
        print!("{esc}c", esc = 27 as char);
        println!("###########################################################################");
        println!("because of font weirdness, screen might be a bit stretched");
        println!("As long as each character is the same width, it should be good.");
        println!("(on some terminals you can set cell spacing, a spacing of 1.5 should work.)");
        println!("###########################################################################");
    }

    pub fn sleep(&self)
    {

    }

    pub fn update(&self)
    {
        // uses the teletext G1 block mosaics to generate the display
        // https://en.wikipedia.org/wiki/Box-drawing_characters#BBC_and_Acorn
        let mut output = "".to_string();
        let pixels = [
            ' ','🬞','🬏','🬭','🬇','🬦','🬖','🬵',
            '🬃','🬢','🬓','🬱','🬋','🬩','🬚','🬹',
            '🬁','🬠','🬑','🬯','🬉','▐','🬘','🬷',
            '🬅','🬤','🬔','🬳','🬍','🬫','🬜','🬻',
            '🬀','🬟','🬐','🬮','🬈','🬧','🬗','🬶',
            '🬄','🬣','▌','🬲','🬌','🬪','🬛','🬺',
            '🬂','🬡','🬒','🬰','🬊','🬨','🬙','🬸',
            '🬆','🬥','🬕','🬴','🬎','🬬','🬝','█',
        ];
        
        for y in (0..SIZE).step_by(3)
        {
            for x in (0..SIZE).step_by(2)
            {
                output.push(pixels[self.gen_character(&self.buffer_curr, x, y)]);
            }
            output.push('\n');
        }

        println!("{}", output);
    }
    
    ////////////////////////////////////
    // used a lot by other display functions.

    fn set_bit(&mut self, index: usize, bit_index: u8, value: bool) 
    {
        let clr_mask = 0xff ^ (0x80 >> bit_index);
        let set_mask = ((value as u8) << 7) >> bit_index;
    
        self.buffer_curr[index] &= clr_mask;
        self.buffer_curr[index] |= set_mask;
    }

    fn get_bit(&mut self, arr: &[u8], index: usize, bit_index: u8) -> bool
    {
        let mask = 0x80 >> bit_index;
        return (arr[index] & mask) > 0;
    }

    ////////////////////////////////////

    fn gen_character(&self, buff: &[u8; BUFFSIZE], x:usize, y:usize) -> usize
    {
        let mut output: u8 = 0x0;
        let width : usize = (SIZE+7)/8;
        for y_off in 0..3
        {
            for x_off in 0..2
            {
                if (y+y_off >= SIZE) || (x+x_off >= SIZE) {continue;}
                  
                let out_mask = 0x20 >> (y_off*2 + x_off);
                let in_byte = buff[(y+y_off)*width + ((x+x_off) / 8)];
                let byte_mask = 0x80 >> ((x+x_off) % 8);

                if (in_byte & byte_mask) != 0 {output |= 0xFF & out_mask;}
            };
        }

        return output as usize;
    }
}
#[derive(Copy)]
#[derive(Clone)]
pub enum Event
{
    None,
    Alarm,
    Minute,
    BtnUp,
    BtnMid,
    BtnDown
}


pub struct Io
{
    event_buffer: [Event; 10],
}

////////////////////////////

impl Io
{
    pub fn new() -> Io
    {
        let buffer: [Event; 10] = [Event::None; 10];
        
        return Io{
            event_buffer: buffer,
        };
    }

    pub fn wait_for_input(&self)
    {
        // loop{}
    }    
}
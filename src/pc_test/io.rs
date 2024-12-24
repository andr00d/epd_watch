#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Event
{
    None,
    Minute,
    Alarm,
    BtnUp,
    BtnMid,
    BtnDown
}


pub struct Io{}

////////////////////////////

impl Io
{
    pub fn new() -> Io
    {
        return Io{};
    }

    pub fn wait_for_input(&self) -> Event
    {
        println!("(0): None\t(1): Minute\t(2): Alarm\t(3-5):Buttons");
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).expect("invalid input");

        let ev_int: i32 = input_line.trim().parse().unwrap_or(0);
        return match ev_int 
        {
            1 => Event::Minute,
            2 => Event::Alarm,
            3 => Event::BtnUp,
            4 => Event::BtnMid,
            5 => Event::BtnDown,
            _ => Event::None
        };
    }  
    
    pub fn set_datetime(&mut self, _dy: i32, _dm: u32, _dd: u32, _h: u32, _m: u32) -> bool
    {
        return true;
    }

    pub fn get_time_str(&mut self) -> String
    {
        return "12:45".to_string();
    }

    pub fn get_date_str(&mut self) -> String
    {
        return "19 mar".to_string();
    }

    pub fn get_dow_str(&mut self) -> String
    {
        return "wed".to_string();
    }
}
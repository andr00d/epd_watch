use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Event
{
    NoEvent,
    Minute,
    Alarm,
    BtnUp,
    BtnMid,
    BtnDown
}


pub struct Io
{
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

////////////////////////////

impl Io
{
    pub fn new() -> Io
    {
        return Io
        {
            year: 2025,
            month: 1,
            day: 1,
            hour: 12,
            minute: 0,
        };
    }

    ////////////////////////////////////////////

    pub fn wait_for_input(&mut self) -> Event
    {
        println!("(0): None\t(1): Minute\t(2): Alarm\t(3-5):Buttons");
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).expect("invalid input");

        let ev_int: i32 = input_line.trim().parse().unwrap_or(0);
        if ev_int == 1 {self.inc_clock();}

        return match ev_int 
        {
            1 => Event::Minute,
            2 => Event::Alarm,
            3 => Event::BtnUp,
            4 => Event::BtnMid,
            5 => Event::BtnDown,
            _ => Event::NoEvent
        };
    }  


    pub fn get_input_waitms(&mut self, _delay_ms: u16) -> Event
    {
        return self.wait_for_input();
    }
    
    ////////////////////////////////////////////

    fn inc_clock(&mut self)
    {
        self.minute = (self.minute + 1) % 60;
        if self.minute == 0 {self.hour = (self.hour + 1) % 24;}
        if self.hour == 0 {self.day = (self.day + 1) % 28;}
    }
    
    pub fn get_datetime(&self) -> NaiveDateTime
    {
        let nd = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let nt = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        return NaiveDateTime::new(nd, nt);
    }

    pub fn set_datetime(&mut self, dy: u16, dm: u8, dd: u8, h: u8, m: u8) -> bool
    {
        self.year = dy;
        self.month = dm;
        self.day = dd;
        self.hour = h;
        self.minute = m;
        return true;
    }

    pub fn get_time_str(&mut self) -> String
    {
        return format!("{:02}:{:02}", self.hour, self.minute);
    }

    pub fn get_dow_str(&mut self) -> String
    {
        // not accurate, who cares. 
        let days = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];
        return days[self.day as usize % 7].to_string();
    }

    pub fn get_date_str(&mut self) -> String
    {
        let months = ["jan", "feb", "mar", "apr" , "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];
        return format!("{} {}", self.day, months[self.month as usize % 12]);
    }

    ////////////////////////////////////////////

    pub fn set_alarm(&mut self, _d: u8, _h: u8, _m: u8)
    {
        rprintln!("set alarm.");
    }

    pub fn disable_alarm(&mut self)
    {
        rprintln!("disable alarm.");
    }

    ////////////////////////////////////////////

    pub fn play_tone(&mut self)
    {
        rprintln!("play tone.");
    }
}
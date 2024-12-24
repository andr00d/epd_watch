use crate::display::Display;
use crate::io::Io;

pub enum AlarmMode
{
    Off,
    Once,
    Daily,
    Weekly,
}

pub struct SharedData<'a>
{
    // Core function data
    pub display: &'a mut Display,
    pub io: &'a mut Io,
    pub update: bool,

    // time data
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub dow: u8,
    pub hour: u8,
    pub minute: u8,

    // alarm data
    pub dateset_alarm: bool,
    pub mode: AlarmMode,
    pub alarm_dow: u8,
    pub alarm_hour: u8,
    pub alarm_minute: u8,

    // stopwatch data
    pub stopwatch_started: bool,
    pub lap_index: u8,
}

impl SharedData<'_>
{
    pub fn new<'a>(display: &'a mut Display, io: &'a mut Io) -> SharedData<'a>
    {
        return SharedData
        {
            display: display,
            io: io,
            update: true,

            year: 2025,
            month: 1,
            day: 1,
            dow: 2,
            hour: 12,
            minute: 0,

            dateset_alarm: false,
            mode: AlarmMode::Off,
            alarm_dow: 0,
            alarm_hour: 0,
            alarm_minute: 0,

            stopwatch_started: false,
            lap_index: 0,
        };
    }
}
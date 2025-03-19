use chrono::{Duration, NaiveDateTime, NaiveDate, NaiveTime};

use crate::display::Display;
use crate::io::Io;

#[derive(PartialEq)]
#[derive(Clone)]
pub enum AlarmMode
{
    Off,
    Once,
    Daily,
    Weekly,
}

#[derive(PartialEq)]
#[derive(Clone)]
pub enum StopwatchState
{
    Started,
    Paused,
    Stopped,
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
    pub hour: u8,
    pub minute: u8,

    // alarm data
    pub dateset_alarm: bool,
    pub mode: AlarmMode,
    pub alarm_dow: u8,
    pub alarm_hour: u8,
    pub alarm_minute: u8,

    // stopwatch data
    pub stopwatch_state: StopwatchState,
    pub time_start: NaiveDateTime,
    pub laps: [Duration; 4],
    pub lap_index: usize,
}

impl SharedData<'_>
{
    pub fn new<'a>(display: &'a mut Display, io: &'a mut Io) -> SharedData<'a>
    {
        let nd = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let nt = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let dt = NaiveDateTime::new(nd, nt);
        let td = Duration::new(0,0).unwrap();

        return SharedData
        {
            display: display,
            io: io,
            update: true,

            year: 2025,
            month: 1,
            day: 1,
            hour: 12,
            minute: 0,

            dateset_alarm: false,
            mode: AlarmMode::Off,
            alarm_dow: 0,
            alarm_hour: 0,
            alarm_minute: 0,

            stopwatch_state: StopwatchState::Stopped,
            time_start: dt,
            laps: [td, td, td, td],
            lap_index: 0,
        };
    }
}
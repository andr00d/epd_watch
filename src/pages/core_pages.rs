#[cfg(target_os = "none")]
use core::fmt::Write;
#[cfg(target_os = "none")]
use heapless::String;
use chrono::Duration;

use crate::pages::Pages;
use crate::shared_data::{SharedData, AlarmMode};
use crate::display::font::Anchor;

impl Pages
{
    pub(super) fn menu_time(data: &mut SharedData) 
    {
        let time_str = data.io.get_time_str();
        let date_str = data.io.get_date_str();
        let dow_str = data.io.get_dow_str();

        let time_width = data.display.get_text_width(&time_str, 10);
        let date_width = data.display.get_text_width(&time_str, 4) + 
                         data.display.get_text_width(&date_str, 4) + 16;

        let offset = match date_width > time_width
        {
            true => (date_width / 2) as u8,
            false => (time_width / 2) as u8,
        };

        data.display.text(&time_str, 100, 75, 10, Anchor::Center);
        data.display.text(&dow_str, 100-offset, 50, 4, Anchor::Left);
        data.display.text(&date_str, 100+offset, 50, 4, Anchor::Right)
    }

    pub(super) fn menu_settings(data: &mut SharedData) 
    {
        data.display.text("set time", 100, 88, 5, Anchor::Center);
    }

    pub(super) fn menu_alarm(data: &mut SharedData) 
    {
        data.display.text("set alarm", 100, 88, 5, Anchor::Center);
    }

    pub(super) fn menu_stopwatch(data: &mut SharedData) 
    {
        data.display.text("stopwatch", 100, 88, 5, Anchor::Center);
    }

    pub(super) fn menu_alarmed(data: &mut SharedData) 
    {
        let time_str = data.io.get_time_str();
        data.display.text("alarm", 100, 70, 6, Anchor::Center);
        data.display.text(&time_str, 100, 105, 6, Anchor::Center);
    }

    ///////////////////////////////////////////

    pub(super) fn pg_alarm_mode(data: &mut SharedData) 
    {
        let modes = ["Off", "Once", "Daily", "Weekly"];
        data.display.text("mode:", 100, 70, 4, Anchor::Center);
        data.display.text(modes[data.mode.clone() as usize], 100, 96, 7, Anchor::Center);

    }

    pub(super) fn pg_stopwatch(data: &mut SharedData) 
    {
        if data.stopwatch_started
        {
            data.display.text("stop", 195, 35, 3, Anchor::Right);
            data.display.text("exit", 195, 85, 3, Anchor::Right);
            data.display.text("lap", 195, 135, 3, Anchor::Right);
        }
        else
        {
            data.display.text("start", 195, 35, 3, Anchor::Right);
            data.display.text("exit", 195, 85, 3, Anchor::Right);
            data.display.text("reset", 195, 135, 3, Anchor::Right); 
        }

        //TODO: add stopwatch.
    }

    ///////////////////////////////////////////

    fn dtset_shared(data: &mut SharedData, text: &str, val: &str)
    {
        data.display.text(text, 100, 5, 5, Anchor::Center);
        data.display.text("+", 195, 25, 5, Anchor::Right );
        data.display.text("-", 195, 150, 5, Anchor::Right);
        data.display.text(val, 100, 70, 6, Anchor::Center);
    }

    #[cfg(target_os = "none")]
    fn dtset_string(val: u16) -> String::<32>
    {
        let mut out = String::<32>::new();
        let _ = write!(out, "{}", val);
        return out
    }

    #[cfg(target_os = "linux")]
    fn dtset_string(val: u16) -> String
    {
        return val.to_string();
    }

    pub(super) fn pg_dtset_year(data: &mut SharedData) 
    {
        let val = Self::dtset_string(data.year);
        Self::dtset_shared(data, "set year", &val);
    }

    pub(super) fn pg_dtset_month(data: &mut SharedData) 
    {
        let val = Self::dtset_string(data.month as u16);
        Self::dtset_shared(data, "set month", &val);
    }

    pub(super) fn pg_dtset_day(data: &mut SharedData) 
    {
        let val = Self::dtset_string(data.day as u16);
        Self::dtset_shared(data, "set day", &val);
    }

    pub(super) fn pg_dtset_dow(data: &mut SharedData) 
    {
        let val = Self::dtset_string(data.alarm_dow as u16);
        Self::dtset_shared(data, "set dow", &val);
    }

    pub(super) fn pg_dtset_hour(data: &mut SharedData) 
    {
        if data.dateset_alarm 
        {
            let val = Self::dtset_string(data.alarm_hour as u16);
            Self::dtset_shared(data, "set hour", &val);
        }
        else 
        {
            let val = Self::dtset_string(data.hour as u16);
            Self::dtset_shared(data, "set hour", &val);
        }
    }

    pub(super) fn pg_dtset_min(data: &mut SharedData) 
    {
        if data.dateset_alarm 
        {
            let val = Self::dtset_string(data.alarm_minute as u16);
            Self::dtset_shared(data, "set minute", &val);
        }
        else 
        {
            let val = Self::dtset_string(data.minute as u16);
            Self::dtset_shared(data, "set minute", &val);
        }
    }

    ///////////////////////////////////////////
    ///////////////////////////////////////////

    pub(super) fn mv_stopwatch_exit(data: &mut SharedData)
    {
        let td = Duration::new(0,0).unwrap();
        data.stopwatch_started = false;
        data.laps = [td, td, td, td];
        data.lap_index = 0;
    }

    pub(super) fn mv_stopwatch_startstop(data: &mut SharedData)
    {
        if data.stopwatch_started
        {
            data.stopwatch_started = false;
            if data.lap_index < 4
            {
                let time = data.io.get_datetime() - data.time_start;
                data.laps[data.lap_index] = time;
                data.lap_index +=1;
            }
        }
        else
        {
            data.stopwatch_started = true;
            data.time_start = data.io.get_datetime();
        }
    }

    pub(super) fn mv_stopwatch_rstlap(data: &mut SharedData)
    {
        if data.stopwatch_started
        {
            if data.lap_index < 4
            {
                let time = data.io.get_datetime() - data.time_start;
                data.laps[data.lap_index] = time;
                data.lap_index +=1;
            }
        }
        else
        {
            let td = Duration::new(0,0).unwrap();
            data.laps = [td, td, td, td];
            data.lap_index = 0;
        }
    }

    pub(super) fn mv_alarm_reset(data: &mut SharedData)
    {
        if data.mode == AlarmMode::Once
        {
            data.io.disable_alarm();
        }
    }

    ///////////////////////////////////////////

    pub(super) fn mv_alarm_trigger(data: &mut SharedData)
    {
        data.io.play_tone();
    }

    pub(super) fn mv_alarm_exit(data: &mut SharedData)
    {
        data.io.set_alarm(data.alarm_dow, data.alarm_hour, data.minute);
    }

    pub(super) fn mv_alarm_modeup(data: &mut SharedData)
    {
        data.mode = match data.mode
        {
            AlarmMode::Off => AlarmMode::Once,
            AlarmMode::Once => AlarmMode::Daily,
            AlarmMode::Daily => AlarmMode::Weekly,
            AlarmMode::Weekly => AlarmMode::Off,
        };
    }

    pub(super) fn mv_alarm_modedown(data: &mut SharedData)
    {
        data.mode = match data.mode
        {
            AlarmMode::Off => AlarmMode::Weekly,
            AlarmMode::Once => AlarmMode::Off,
            AlarmMode::Daily => AlarmMode::Once,
            AlarmMode::Weekly => AlarmMode::Daily,
        };
    }

    pub(super) fn mv_alarm_dowup(data: &mut SharedData) 
    { data.alarm_dow = if data.alarm_dow >= 6 {0} else {data.alarm_dow + 1}; }

    pub(super) fn mv_alarm_dowdown(data: &mut SharedData) 
    { data.alarm_dow = if data.alarm_dow == 0 {6} else {data.alarm_dow - 1}; }

    pub(super) fn mv_alarm_hourup(data: &mut SharedData) 
    { data.alarm_hour = if data.alarm_hour >= 23 {0} else {data.alarm_hour + 1}; }

    pub(super) fn mv_alarm_hourdown(data: &mut SharedData) 
    { data.alarm_hour = if data.alarm_hour == 0 {23} else {data.alarm_hour - 1}; }

    pub(super) fn mv_alarm_minup(data: &mut SharedData) 
    { data.alarm_minute = if data.alarm_minute >= 59 {0} else {data.alarm_minute + 1}; }

    pub(super) fn mv_alarm_mindown(data: &mut SharedData) 
    { data.alarm_minute = if data.alarm_minute == 0 {59} else {data.alarm_minute - 1}; }

    ///////////////////////////////////////////
    
    pub(super) fn mv_dtset_exit(data: &mut SharedData)
    {
        data.io.set_datetime(data.year, data.month, data.day, data.hour, data.minute);
    }
    
    pub(super) fn mv_dtset_yearup(data: &mut SharedData) 
    { data.year = if data.year >= 2100 {1970} else {data.year + 1}; }

    pub(super) fn mv_dtset_yeardown(data: &mut SharedData) 
    { data.year = if data.year <= 1970 {2100} else {data.year - 1}; }

    pub(super) fn mv_dtset_monthup(data: &mut SharedData) 
    { data.month = if data.month >= 11 {0} else {data.month + 1}; }

    pub(super) fn mv_dtset_monthdown(data: &mut SharedData) 
    { data.month = if data.month == 0 {11} else {data.month - 1}; }

    pub(super) fn mv_dtset_hourup(data: &mut SharedData) 
    { data.hour = if data.hour >= 23 {0} else {data.hour + 1}; }

    pub(super) fn mv_dtset_hourdown(data: &mut SharedData) 
    { data.hour = if data.hour == 0 {23} else {data.hour - 1}; }

    pub(super) fn mv_dtset_minup(data: &mut SharedData) 
    { data.minute = if data.minute >= 59 {0} else {data.minute + 1}; }

    pub(super) fn mv_dtset_mindown(data: &mut SharedData) 
    { data.minute = if data.minute == 0 {59} else {data.minute - 1}; }


    pub(super) fn mv_dtset_dayup(data: &mut SharedData) 
    { 
        // TODO: doesn't allow setting day to leapday.
        let days: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        data.day = if data.day >= days[data.month as usize] {0} else {data.day + 1}; 
    }

    pub(super) fn mv_dtset_daydown(data: &mut SharedData) 
    { 
        // TODO: doesn't allow setting day to leapday.
        let days: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        data.day = if data.day == 0 {days[data.month as usize]} else {data.day - 1};
    }
    
}
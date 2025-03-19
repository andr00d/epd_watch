#[cfg(target_os = "none")]
use core::fmt::Write;
#[cfg(target_os = "none")]
use heapless::String;
use chrono::{Duration, Timelike};

use crate::pages::Pages;
use crate::shared_data::{SharedData, AlarmMode, StopwatchState};
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
        
        if data.io.get_datetime().minute() == 0 {data.display.set_clean_update();}
        
        data.display.text(&time_str, 100, 75, 10, Anchor::Center);
        data.display.text(&dow_str, 100-offset, 50, 4, Anchor::Left);
        data.display.text(&date_str, 100+offset, 50, 4, Anchor::Right);
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

    pub(super) fn menu_snake(data: &mut SharedData) 
    {
        data.display.text("snake", 100, 88, 5, Anchor::Center);
    }

    pub(super) fn menu_video(data: &mut SharedData) 
    {
        data.display.text(":)", 100, 88, 5, Anchor::Center);
    }

    pub(super) fn menu_qr(data: &mut SharedData) 
    {
        let qr_data = [0x1, 0xd0, 0xe0, 0xc0, 0x3e, 0xac, 0x19, 0xef, 0x91, 0x7c, 0xb4, 0xf4, 0x48, 0xa2, 0x97, 
         0xaa, 0x24, 0x5b, 0xe0, 0x2d, 0x13, 0xe9, 0x3e, 0xce, 0xf8, 0x5, 0x55, 0x55, 0x1, 0xff, 0x1a, 0xd5, 0xff, 
         0x4, 0x2f, 0x30, 0x2a, 0xfb, 0x68, 0x72, 0x2f, 0x21, 0x76, 0x5, 0x45, 0xe8, 0xee, 0x5c, 0x31, 0xbb, 0x41,
         0x4a, 0xd2, 0x7f, 0x71, 0xf0, 0x46, 0xd0, 0x72, 0x1f, 0x54, 0xa, 0x66, 0x5a, 0xe2, 0x6b, 0x5c, 0x4f, 0xbd,
         0xe6, 0x9f, 0x58, 0x41, 0x25, 0x35, 0x1e, 0xc, 0xc0, 0xb3, 0xc6, 0x5e, 0x21, 0x3d, 0x4f, 0x53, 0xda, 0x6a, 
         0xbd, 0xf8, 0x2, 0xc1, 0x93, 0x9a, 0xe5, 0xea, 0xf7, 0xdb, 0x94, 0x73, 0x70, 0x6e, 0x15, 0x7, 0x7f, 0x98, 
         0x81, 0x3b, 0x80, 0x5e, 0x3f, 0x94, 0x2f, 0xbe, 0x4e, 0xe, 0x4, 0x51, 0x43, 0x50, 0x2a, 0x2a, 0xfa, 0x39, 
         0x81, 0x15, 0x9b, 0xc9, 0x76, 0xfa, 0x9a, 0xc7, 0x13, 0x1, 0x6f, 0xae, 0x46, 0x80];

        let mut i = 0;
        for y in 0..33
        {
            for x in 0..33
            {
                let val = qr_data[i / 8] & (0b10000000 >> (i % 8));
                if val == 0 {data.display.rect(17+x*5, 17+y*5, 5, 5);}
                i += 1;
            }
        }
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
        data.display.rect(10, 40, 130, 2);

        if data.stopwatch_state == StopwatchState::Started
        {
            let time_elapsed = data.io.get_datetime() - data.time_start;
            let time_str = data.io.get_td_str(time_elapsed);

            data.display.text(&time_str, 70, 10, 5, Anchor::Center);
            data.display.text("stop", 195, 35, 3, Anchor::Right);
            data.display.text("exit", 195, 85, 3, Anchor::Right);
            data.display.text("lap", 195, 135, 3, Anchor::Right);
        }
        else
        {
            if data.stopwatch_state == StopwatchState::Paused
            {
                let time_elapsed = data.io.get_datetime() - data.time_start;
                let time_str = data.io.get_td_str(time_elapsed);
                data.display.text(&time_str, 70, 10, 5, Anchor::Center); 
            }
            else
            {
                data.display.text("00:00", 70, 10, 5, Anchor::Center);
            }

            data.display.text("start", 195, 35, 3, Anchor::Right);
            data.display.text("exit", 195, 85, 3, Anchor::Right);
            data.display.text("reset", 195, 135, 3, Anchor::Right); 
        }

        
        for i in 0..4
        {
            let time_str = data.io.get_td_str(data.laps[i as usize]);
            data.display.text(&time_str, 70, 52+(35*i), 5, Anchor::Center);
        }
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
        data.stopwatch_state = StopwatchState::Stopped;
        let td = Duration::new(0,0).unwrap();
        data.laps = [td, td, td, td];
        data.lap_index = 0;
    }

    pub(super) fn mv_stopwatch_startstop(data: &mut SharedData)
    {
        if data.stopwatch_state == StopwatchState::Started
        {
            data.stopwatch_state = StopwatchState::Paused;
            if data.lap_index >= data.laps.len() 
            {
                for i in 0..3 { data.laps[i] = data.laps[i+1]; }
                data.lap_index -= 1;
            }
            
            let time = data.io.get_datetime() - data.time_start;
            data.laps[data.lap_index] = time;
            data.lap_index +=1;
        }
        else
        {
            if data.stopwatch_state == StopwatchState::Stopped
            {
                data.time_start = data.io.get_datetime();
            }
            
            data.stopwatch_state = StopwatchState::Started;
        }
    }

    pub(super) fn mv_stopwatch_rstlap(data: &mut SharedData)
    {
        if data.stopwatch_state == StopwatchState::Started
        {
            if data.lap_index >= data.laps.len() 
            {
                for i in 0..3 { data.laps[i] = data.laps[i+1]; }
                data.lap_index -= 1;
            }

            let time = data.io.get_datetime() - data.time_start;
            data.laps[data.lap_index] = time;
            data.lap_index +=1;
        }
        else
        {
            data.stopwatch_state = StopwatchState::Stopped;
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
        data.day = if data.day >= days[data.month as usize] {1} else {data.day + 1}; 
    }

    pub(super) fn mv_dtset_daydown(data: &mut SharedData) 
    { 
        // TODO: doesn't allow setting day to leapday.
        let days: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        data.day = if data.day == 1 {days[data.month as usize]} else {data.day - 1};
    }
    
}
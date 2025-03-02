#[cfg(target_os = "none")]
use rtt_target::rprintln;

use crate::pages::{Pages, };
use crate::shared_data::SharedData;
use crate::pages::state::PageState::*;
use crate::pages::state::PageState;
use crate::io::Event::*;
use crate::io::Event;

impl Pages
{
    pub fn new() -> Pages
    {
        return Pages
        {
            curr_page: Self::menu_time,
            curr_state: MenuTime,
        };
    }

    fn sm_step(&mut self, data: &mut SharedData, goal_state: PageState, 
        pg_func: fn(&mut SharedData) -> (), trans_func: Option<fn(&mut SharedData) -> ()>)
    {
        if trans_func.is_some() {(trans_func.unwrap())(data);} 
        self.curr_state = goal_state;
        self.curr_page = pg_func;
        data.update = true;
    }

    pub fn update_page(&mut self, ev: Event, data: &mut SharedData)
    {
        match self.curr_state
        {
            MenuTime => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                Minute  => {self.sm_step(data, MenuTime,        Self::menu_time,        None);},
                BtnUp   => {self.sm_step(data, MenuVideo,       Self::menu_video,       None);},
                BtnDown => {self.sm_step(data, MenuSettings,    Self::menu_settings,    None);},
                _ => (),
            } },

            MenuSettings => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, MenuTime,        Self::menu_time,        None);},
                BtnMid  => {self.sm_step(data, PgDatesetYear,   Self::pg_dtset_year,    None);},
                BtnDown => {self.sm_step(data, MenuAlarm,       Self::menu_alarm,       None);},
                _ => (),
            } },

            MenuAlarm => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, MenuSettings,    Self::menu_settings,    None);},
                BtnMid  => {self.sm_step(data, PgAlarmMode,     Self::pg_alarm_mode,    None);},
                BtnDown => {self.sm_step(data, MenuStopwatch,   Self::menu_stopwatch,   None);},
                _ => (),
            } },

            MenuStopwatch => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, MenuAlarm,       Self::menu_alarm,       None);},
                BtnMid  => {self.sm_step(data, PgStopwatch,     Self::pg_stopwatch,     None);},
                BtnDown => {self.sm_step(data, MenuSnake,       Self::menu_snake,       None);},
                _ => (),
            } },

            MenuSnake => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, MenuStopwatch,   Self::menu_stopwatch,   None);},
                BtnMid  => {self.sm_step(data, MenuSnake,       Self::menu_snake,       Some(Self::pg_snake));},
                BtnDown => {self.sm_step(data, MenuVideo,       Self::menu_video,       None);},
                _ => (),
            } },

            MenuVideo => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, MenuStopwatch,   Self::menu_stopwatch,   None);},
                BtnMid  => {self.sm_step(data, MenuVideo,       Self::menu_video,       Some(Self::pg_video));},
                BtnDown => {self.sm_step(data, MenuTime,        Self::menu_time,        None);},
                _ => (),
            } },

            MenuAlarmed => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnMid  => {self.sm_step(data, MenuTime,        Self::menu_time,        Some(Self::mv_alarm_reset));},
                _ => (),
            } },

            PgAlarmMode => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, PgAlarmMode,     Self::pg_alarm_mode,    Some(Self::mv_alarm_modeup));},
                BtnMid  => {self.sm_step(data, PgAlarmDow,      Self::pg_dtset_dow,     None);},
                BtnDown => {self.sm_step(data, PgAlarmMode,     Self::pg_alarm_mode,    Some(Self::mv_alarm_modedown));},
                _ => (),
            } },

            PgAlarmDow => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, PgAlarmDow,      Self::pg_dtset_dow,     Some(Self::mv_alarm_dowup));},
                BtnMid  => {self.sm_step(data, PgAlarmHour,     Self::pg_dtset_hour,    None);},
                BtnDown => {self.sm_step(data, PgAlarmDow,      Self::pg_dtset_dow,     Some(Self::mv_alarm_dowdown));},
                _ => (),
            } },

            PgAlarmHour => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, PgAlarmHour,     Self::pg_dtset_hour,    Some(Self::mv_alarm_hourup));},
                BtnMid  => {self.sm_step(data, PgAlarmMin,      Self::pg_dtset_min,     None);},
                BtnDown => {self.sm_step(data, PgAlarmHour,     Self::pg_dtset_hour,    Some(Self::mv_alarm_hourdown));},
                _ => (),
            } },

            PgAlarmMin => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, PgAlarmMin,      Self::pg_dtset_min,     Some(Self::mv_alarm_minup));},
                BtnMid  => {self.sm_step(data, MenuAlarm,       Self::menu_alarm,       Some(Self::mv_alarm_exit));},
                BtnDown => {self.sm_step(data, PgAlarmMin,      Self::pg_dtset_min,     Some(Self::mv_alarm_mindown));},
                _ => (),
            } },

            PgStopwatch => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_stopwatch_exit));},
                BtnUp   => {self.sm_step(data, PgStopwatch,     Self::pg_stopwatch,     Some(Self::mv_stopwatch_startstop));},
                BtnMid  => {self.sm_step(data, MenuStopwatch,   Self::menu_stopwatch,   Some(Self::mv_stopwatch_exit));},
                BtnDown => {self.sm_step(data, PgStopwatch,     Self::pg_stopwatch,     Some(Self::mv_stopwatch_rstlap));},
                _ => (),
            } },

            PgDatesetYear => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, PgDatesetYear,   Self::pg_dtset_year,    Some(Self::mv_dtset_yearup));},
                BtnMid  => {self.sm_step(data, PgDatesetMonth,  Self::pg_dtset_month,   None);},
                BtnDown => {self.sm_step(data, PgDatesetYear,   Self::pg_dtset_year,    Some(Self::mv_dtset_yeardown));},
                _ => (),
            } },

            PgDatesetMonth => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, PgDatesetMonth,  Self::pg_dtset_month,   Some(Self::mv_dtset_monthup));},
                BtnMid  => {self.sm_step(data, PgDatesetDay,    Self::pg_dtset_day,     None);},
                BtnDown => {self.sm_step(data, PgDatesetMonth,  Self::pg_dtset_month,   Some(Self::mv_dtset_monthdown));},
                _ => (),
            } },

            PgDatesetDay => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, PgDatesetDay,    Self::pg_dtset_day,     Some(Self::mv_dtset_dayup));},
                BtnMid  => {self.sm_step(data, PgDatesetHour,   Self::pg_dtset_hour,    None);},
                BtnDown => {self.sm_step(data, PgDatesetDay,    Self::pg_dtset_day,     Some(Self::mv_dtset_daydown));},
                _ => (),
            } },

            PgDatesetHour => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, PgDatesetHour,   Self::pg_dtset_hour,    Some(Self::mv_dtset_hourup));},
                BtnMid  => {self.sm_step(data, PgDatesetMin,    Self::pg_dtset_min,     None);},
                BtnDown => {self.sm_step(data, PgDatesetHour,   Self::pg_dtset_hour,    Some(Self::mv_dtset_hourdown));},
                _ => (),
            } },

            PgDatesetMin => { match ev {
                Alarm   => {self.sm_step(data, MenuAlarmed,     Self::menu_alarmed,     Some(Self::mv_alarm_trigger));},
                BtnUp   => {self.sm_step(data, PgDatesetMin,    Self::pg_dtset_min,     Some(Self::mv_dtset_minup));},
                BtnMid  => {self.sm_step(data, MenuSettings,    Self::menu_settings,    Some(Self::mv_dtset_exit));},
                BtnDown => {self.sm_step(data, PgDatesetMin,    Self::pg_dtset_min,     Some(Self::mv_dtset_mindown));},
                _ => (),
            } },

            // _ => (),
        }

        rprintln!("page: {:?}", self.curr_state);
        (self.curr_page)(data);
    }
}
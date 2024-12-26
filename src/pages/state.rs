
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PageState
{
    MenuTime,
    MenuSettings,
    MenuAlarm,
    MenuStopwatch,
    MenuAlarmed,
    
    PgAlarmMode,
    PgAlarmDow,
    PgAlarmHour,
    PgAlarmMin,

    PgStopwatch,
    PgDatesetYear,
    PgDatesetMonth,
    PgDatesetDay,
    PgDatesetHour,
    PgDatesetMin,
}
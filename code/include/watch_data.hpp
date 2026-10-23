#ifndef _WATCH_DATA_HPP_
#define _WATCH_DATA_HPP_

enum AlarmMode 
{
    Off,
    Once,
    Daily,
    Weekly,
}; 

enum StateStopWatch 
{
    Started,
    Paused,
    Stopped,
};

struct SimpleDateTime
{
    uint16_t year,
    uint8_t month,
    uint8_t day,
    uint8_t hour,
    uint8_t minute,
}

struct 
{
    SimpleDateTime time,

    // alarm data
    bool dateset_alarm,
    AlarmMode mode,
    uint8_t alarm_dow,
    uint8_t alarm_hour,
    uint8_t alarm_minute,

    // stopwatch data
    StopwatchState stopwatch_state,
    SimpleDateTime stopwatch_starttime,
    int values[];



} WatchData;

#endif
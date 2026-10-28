#ifndef _WATCH_DATA_HPP_
#define _WATCH_DATA_HPP_

#include <time.h>


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

struct 
{
    tm start,

    // alarm data
    bool dateset_alarm,
    AlarmMode mode,
    uint8_t alarm_dow,
    uint8_t alarm_hour,
    uint8_t alarm_minute,

    // stopwatch data
    StopwatchState stopwatch_state,
    tm stopwatch_starttime,
    tm laps[4],
    uint8_t lap_index,

    // battery data
    uint8_t batt_level,
    bool is_charging,
} WatchData;

#endif
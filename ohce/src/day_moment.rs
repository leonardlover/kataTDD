#[cfg(not(test))]
use chrono::Local;
#[cfg(test)]
use crate::mocks::Local;

use chrono::NaiveTime;

pub enum DayMoment {
    Morning,
    Afternoon,
    Evening,
}

fn get_day_moment_start_time(dm: DayMoment) -> NaiveTime {
    let s = match dm {
        DayMoment::Morning => "06:00:00",
        DayMoment::Afternoon => "12:00:00",
        DayMoment::Evening => "20:00:00",
    };
    NaiveTime::parse_from_str(s, "%H:%M:%S").unwrap()
}

pub fn get_day_moment() -> DayMoment {
    let local_time = Local::now().time();
    let time_morning = get_day_moment_start_time(DayMoment::Morning);
    let time_afternoon = get_day_moment_start_time(DayMoment::Afternoon);
    let time_evening = get_day_moment_start_time(DayMoment::Evening);

    if time_morning <= local_time && local_time < time_afternoon {
        DayMoment::Morning
    } else if time_afternoon <= local_time && local_time < time_evening {
        DayMoment::Afternoon
    } else {
        DayMoment::Evening
    }
}


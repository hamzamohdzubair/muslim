use chrono::{Duration, NaiveTime};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Prayer {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}

impl fmt::Display for Prayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Prayer::Fajr => write!(f, "Fajr"),
            Prayer::Dhuhr => write!(f, "Dhuhr"),
            Prayer::Asr => write!(f, "Asr"),
            Prayer::Maghrib => write!(f, "Maghrib"),
            Prayer::Isha => write!(f, "Isha"),
        }
    }
}

pub struct NextPrayer {
    pub prayer: Prayer,
    pub time: NaiveTime,
    pub duration: Duration,
    pub is_tomorrow: bool,
}

/// Parse HHMM format (e.g., 0530 = 5:30 AM, 1830 = 6:30 PM)
pub fn parse_time(hhmm: u16) -> NaiveTime {
    let hours = (hhmm / 100) as u32;
    let minutes = (hhmm % 100) as u32;
    NaiveTime::from_hms_opt(hours, minutes, 0)
        .unwrap_or_else(|| NaiveTime::from_hms_opt(0, 0, 0).unwrap())
}

/// Get the next prayer time relative to current time
pub fn get_next_prayer(
    fajr: u16,
    dhuhr: u16,
    asr: u16,
    maghrib: u16,
    isha: u16,
    now: NaiveTime,
) -> NextPrayer {
    let prayers = [
        (Prayer::Fajr, parse_time(fajr)),
        (Prayer::Dhuhr, parse_time(dhuhr)),
        (Prayer::Asr, parse_time(asr)),
        (Prayer::Maghrib, parse_time(maghrib)),
        (Prayer::Isha, parse_time(isha)),
    ];

    // Find the first prayer that is after the current time
    for (prayer, time) in &prayers {
        if *time > now {
            let duration = *time - now;
            return NextPrayer {
                prayer: *prayer,
                time: *time,
                duration,
                is_tomorrow: false,
            };
        }
    }

    // All prayers have passed for today, return Fajr for tomorrow
    let fajr_time = parse_time(fajr);
    let duration = Duration::hours(24) - (now - fajr_time);
    NextPrayer {
        prayer: Prayer::Fajr,
        time: fajr_time,
        duration,
        is_tomorrow: true,
    }
}

/// Format duration in human-readable format
pub fn format_duration(duration: Duration) -> String {
    let total_minutes = duration.num_minutes();
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    if hours > 0 && minutes > 0 {
        let hour_str = if hours == 1 { "hour" } else { "hours" };
        let min_str = if minutes == 1 { "minute" } else { "minutes" };
        format!("{} {} {} {}", hours, hour_str, minutes, min_str)
    } else if hours > 0 {
        if hours == 1 {
            "1 hour".to_string()
        } else {
            format!("{} hours", hours)
        }
    } else if minutes > 1 {
        format!("{} minutes", minutes)
    } else if minutes == 1 {
        "1 minute".to_string()
    } else {
        "less than a minute".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time(0530), NaiveTime::from_hms_opt(5, 30, 0).unwrap());
        assert_eq!(
            parse_time(1245),
            NaiveTime::from_hms_opt(12, 45, 0).unwrap()
        );
        assert_eq!(
            parse_time(1830),
            NaiveTime::from_hms_opt(18, 30, 0).unwrap()
        );
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::minutes(5)), "5 minutes");
        assert_eq!(format_duration(Duration::minutes(1)), "1 minute");
        assert_eq!(format_duration(Duration::minutes(45)), "45 minutes");
        assert_eq!(format_duration(Duration::hours(1)), "1 hour");
        assert_eq!(format_duration(Duration::hours(2)), "2 hours");
        assert_eq!(
            format_duration(Duration::hours(2) + Duration::minutes(15)),
            "2 hours 15 minutes"
        );
    }
}

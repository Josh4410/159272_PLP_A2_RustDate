//! Proleptic Gregorian date type stored as a day count from 0/1/1
//! (astronomical year 0, January 1st). Negative counts are dates
//! before the reference day. No external crates.

use std::fmt;
use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Date {
    days: i32,
}

impl Date {
    /// Builds a date from an astronomical year (0 = 1 BC), month and day.
    fn from_ymd(year: i32, month: i32, day: i32) -> Date {
        let mut total_days = 0;
        if year > 0 {
            for y in 0..year {
                total_days += days_in_year(y);
            }
        } else {
            for y in year..0 {
                total_days -= days_in_year(y);
            }
        }
        for m in 1..month {
            total_days += days_in_month(year, m);
        }
        Date { days: total_days + day - 1 }
    }

    /// Converts back to (astronomical year, month, day).
    fn ymd(&self) -> (i32, i32, i32) {
        let mut remaining = self.days;
        let mut year = 0;

        // Walk whole years forwards or backwards until `remaining`
        // falls inside the current year.
        loop {
            let len = days_in_year(year);
            if (0..len).contains(&remaining) {
                break;
            }
            if remaining >= len {
                remaining -= len;
                year += 1;
            } else {
                year -= 1;
                remaining += days_in_year(year);
            }
        }

        let mut month = 1;
        while remaining >= days_in_month(year, month) {
            remaining -= days_in_month(year, month);
            month += 1;
        }
        (year, month, remaining + 1)
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_year(year: i32) -> i32 {
    if is_leap_year(year) { 366 } else { 365 }
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Date").field("days", &self.days).finish()
    }
}

impl fmt::Display for Date {
    /// Gregorian notation: there is no year 0, so astronomical year 0
    /// is 1 BC, -1 is 2 BC, and so on.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (year, month, day) = self.ymd();
        if year <= 0 {
            write!(f, "{}/{}/{} BC", 1 - year, month, day)
        } else {
            write!(f, "{}/{}/{}", year, month, day)
        }
    }
}

impl Add<i32> for Date {
    type Output = Date;

    fn add(self, days: i32) -> Date {
        Date { days: self.days + days }
    }
}

fn main() {
    println!("{:?}", Date::from_ymd(2022, 12, 31));

    let date = Date::from_ymd(-1, 12, 31);
    for i in 0..20 {
        println!("{}", date + i * 30);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_day_is_zero() {
        assert_eq!(Date::from_ymd(0, 1, 1).days, 0);
    }

    #[test]
    fn round_trips_across_bc_ad() {
        for days in (-800_000..800_000).step_by(97) {
            let (y, m, d) = Date { days }.ymd();
            assert_eq!(Date::from_ymd(y, m, d).days, days);
        }
    }

    #[test]
    fn leap_years() {
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(0));
        assert!(is_leap_year(-4));
    }

    #[test]
    fn display_has_no_year_zero() {
        assert_eq!(Date::from_ymd(0, 1, 1).to_string(), "1/1/1 BC");
        assert_eq!(Date::from_ymd(-1, 12, 31).to_string(), "2/12/31 BC");
        assert_eq!(Date::from_ymd(1, 1, 1).to_string(), "1/1/1");
    }

    #[test]
    fn debug_matches_spec() {
        assert_eq!(format!("{:?}", Date::from_ymd(2022, 12, 31)), "Date { days: 738885 }");
    }
}

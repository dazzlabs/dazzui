//! Gregorian civil arithmetic. A day has no UTC offset, so moving across a
//! daylight-saving boundary never changes the selected calendar day.
use gtk::glib;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, glib::Boxed)]
#[boxed_type(name = "DazzCalendarDate")]
pub struct CalendarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}
impl CalendarDate {
    pub fn new(year: i32, month: u8, day: u8) -> Option<Self> {
        ((1..=12).contains(&month) && day >= 1 && day <= days_in_month(year, month))
            .then_some(Self { year, month, day })
    }
    pub fn parse(value: &str) -> Option<Self> {
        let date = value.split('T').next()?;
        let mut parts = date.rsplitn(3, '-');
        let day = parts.next()?.parse().ok()?;
        let month = parts.next()?.parse().ok()?;
        let year = parts.next()?.parse().ok()?;
        Self::new(year, month, day)
    }
    pub fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
    pub fn today() -> Self {
        let now = glib::DateTime::now_local().expect("Local clock date");
        Self {
            year: now.year(),
            month: now.month() as u8,
            day: now.day_of_month() as u8,
        }
    }
    pub fn first_of_month(self) -> Self {
        Self { day: 1, ..self }
    }
    pub fn same_month(self, other: Self) -> bool {
        self.year == other.year && self.month == other.month
    }
    pub fn add_months(self, months: i32) -> Self {
        let index = i64::from(self.year) * 12 + i64::from(self.month) - 1 + i64::from(months);
        Self {
            year: index.div_euclid(12) as i32,
            month: (index.rem_euclid(12) + 1) as u8,
            day: 1,
        }
    }
    pub fn page_month(self, months: i32) -> Self {
        let month = self.add_months(months);
        Self {
            day: self.day.min(days_in_month(month.year, month.month)),
            ..month
        }
    }
    fn epoch_day(self) -> i64 {
        let year = i64::from(self.year) - i64::from(self.month <= 2);
        let era = year.div_euclid(400);
        let yoe = year - era * 400;
        let month = i64::from(self.month) + if self.month > 2 { -3 } else { 9 };
        let doy = (153 * month + 2) / 5 + i64::from(self.day) - 1;
        era * 146097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 719468
    }
    pub fn add_days(self, days: i32) -> Self {
        let z = self.epoch_day() + i64::from(days) + 719468;
        let era = z.div_euclid(146097);
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = mp + if mp < 10 { 3 } else { -9 };
        Self {
            year: (yoe + era * 400 + i64::from(month <= 2)) as i32,
            month: month as u8,
            day: day as u8,
        }
    }
    /// Sunday is zero, matching the React date vocabulary.
    pub fn weekday(self) -> u8 {
        (self.epoch_day() + 4).rem_euclid(7) as u8
    }
    pub fn month_grid(self, week_starts_on: u8) -> [Self; 42] {
        let first = self.first_of_month();
        let offset = (i32::from(first.weekday()) - i32::from(week_starts_on) + 7) % 7;
        std::array::from_fn(|i| first.add_days(i as i32 - offset))
    }
}
fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leap_days_month_pages_and_week_grids() {
        let date = |s| CalendarDate::parse(s).unwrap();
        assert_eq!(date("2026-10-16").weekday(), 5);
        assert_eq!(date("2000-02-28").add_days(1), date("2000-02-29"));
        assert_eq!(date("1900-02-28").add_days(1), date("1900-03-01"));
        assert_eq!(date("2026-01-31").page_month(1), date("2026-02-28"));
        assert_eq!(date("2024-02-29").page_month(12), date("2025-02-28"));
        for year in [1, 1600, 1900, 1970, 2000, 2024, 2026] {
            let day = CalendarDate::new(year, 1, 1).unwrap();
            for offset in -400..400 {
                assert_eq!(day.add_days(offset).add_days(-offset), day);
            }
        }
        let grid = date("2026-10-16").month_grid(1);
        assert_eq!(grid[0], date("2026-09-28"));
        assert_eq!(grid[41], date("2026-11-08"));
        assert_eq!(date("2026-10-16").month_grid(0)[0], date("2026-09-27"));
    }
}

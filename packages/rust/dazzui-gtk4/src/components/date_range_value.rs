//! Picker dates retain their instant and time zone. Calendar day changes use
//! civil dates, while durations and carried end times use elapsed time.
use crate::{CalendarDate, DateRange};
use gtk::glib;
#[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "DazzDateRangeValue")]
pub struct DateRangeValue {
    pub start: glib::DateTime,
    pub end: glib::DateTime,
    pub all_day: bool,
}
impl DateRangeValue {
    pub fn from_local(start: &str, end: &str, all_day: bool) -> Result<Self, String> {
        Ok(Self {
            start: parse(start)?,
            end: parse(end)?,
            all_day,
        })
    }
    pub fn days(&self) -> DateRange {
        DateRange {
            start: Some(day(&self.start)),
            end: Some(day(&self.end)),
        }
    }
    pub fn one_day(&self) -> bool {
        day(&self.start) == day(&self.end)
    }
    pub fn duration_minutes(&self) -> f64 {
        self.end.difference(&self.start).as_microseconds() as f64 / 60_000_000.
    }
}
pub(super) fn day(value: &glib::DateTime) -> CalendarDate {
    CalendarDate::new(
        value.year(),
        value.month() as u8,
        value.day_of_month() as u8,
    )
    .unwrap()
}
pub(super) fn clock(value: &glib::DateTime) -> String {
    format!("{:02}:{:02}", value.hour(), value.minute())
}
pub(super) fn at(date: CalendarDate, hour: i32, minute: i32) -> Result<glib::DateTime, String> {
    glib::DateTime::from_local(
        date.year,
        date.month as i32,
        date.day as i32,
        hour,
        minute,
        0.,
    )
    .map_err(|e| e.to_string())
}
pub(super) fn parse(value: &str) -> Result<glib::DateTime, String> {
    let date = CalendarDate::parse(value).ok_or("Invalid range date")?;
    let time = value.split_once('T').map(|(_, t)| t).unwrap_or("00:00");
    let (h, m) = parse_clock(time)?;
    at(date, h, m)
}
fn parse_clock(value: &str) -> Result<(i32, i32), String> {
    let mut parts = value.split(':');
    let h = parts
        .next()
        .and_then(|p| p.parse::<i32>().ok())
        .ok_or("Invalid hour")?;
    let m = parts
        .next()
        .and_then(|p| p.parse::<i32>().ok())
        .ok_or("Invalid minute")?;
    if !(0..24).contains(&h) || !(0..60).contains(&m) {
        return Err("Invalid clock time".into());
    }
    Ok((h, m))
}
pub(super) fn blank(today: CalendarDate) -> DateRangeValue {
    let now = glib::DateTime::now_local().unwrap();
    let hour = if day(&now) == today {
        (now.hour() + 1).min(23)
    } else {
        9
    };
    let start = at(today, hour, 0).unwrap();
    let end = start.add_hours(1).unwrap();
    DateRangeValue {
        start,
        end,
        all_day: false,
    }
}
pub(super) struct Draft {
    pub value: DateRangeValue,
    pub days: DateRange,
    pub month: CalendarDate,
}
impl Draft {
    pub fn new(value: DateRangeValue) -> Self {
        Self {
            days: value.days(),
            month: day(&value.start).first_of_month(),
            value,
        }
    }
    pub fn set_days(&mut self, range: DateRange) -> Result<(), String> {
        let start = range.start.unwrap_or_else(|| day(&self.value.start));
        let end = range.end.unwrap_or(start);
        let next_start = at(start, self.value.start.hour(), self.value.start.minute())?;
        let mut next_end = at(end, self.value.end.hour(), self.value.end.minute())?;
        if next_end < next_start {
            next_end = next_start.add_hours(1).map_err(|e| e.to_string())?;
        }
        self.days = range;
        self.value.start = next_start;
        self.value.end = next_end;
        Ok(())
    }
    pub fn start_time(&mut self, time: &str) -> Result<(), String> {
        let (h, m) = parse_clock(time)?;
        let length = self.value.end.difference(&self.value.start);
        let start = at(day(&self.value.start), h, m)?;
        let end = start.add(length).map_err(|e| e.to_string())?;
        self.value.start = start;
        self.value.end = end;
        Ok(())
    }
    pub fn end_time(&mut self, time: &str) -> Result<(), String> {
        let (h, m) = parse_clock(time)?;
        let end = at(day(&self.value.end), h, m)?;
        if end < self.value.start {
            self.value.start = end.add_hours(-1).map_err(|e| e.to_string())?;
        }
        self.value.end = end;
        Ok(())
    }
    pub fn duration(&mut self, minutes: i32) -> Result<(), String> {
        self.value.end = self
            .value
            .start
            .add_minutes(minutes)
            .map_err(|e| e.to_string())?;
        self.value.all_day = false;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn value() -> DateRangeValue {
        DateRangeValue {
            start: parse("2026-10-16T14:30").unwrap(),
            end: parse("2026-10-16T15:30").unwrap(),
            all_day: false,
        }
    }
    #[test]
    fn draft_carries_duration_and_keeps_calendar_days_separate() {
        let mut draft = Draft::new(value());
        draft.start_time("23:30").unwrap();
        assert_eq!(
            day(&draft.value.end),
            CalendarDate::new(2026, 10, 17).unwrap()
        );
        assert_eq!(clock(&draft.value.end), "00:30");
        assert_eq!(
            draft.days.end,
            Some(CalendarDate::new(2026, 10, 16).unwrap())
        );
        draft.end_time("00:00").unwrap();
        assert_eq!(clock(&draft.value.start), "23:30");
        assert_eq!(draft.value.duration_minutes(), 30.);
        draft.duration(120).unwrap();
        assert_eq!(clock(&draft.value.end), "01:30");
        let mut draft = Draft::new(value());
        draft.end_time("13:00").unwrap();
        assert_eq!(clock(&draft.value.start), "12:00");
        assert!(!draft.value.all_day);
    }
    #[test]
    fn range_days_keep_clock_values_and_repair_an_earlier_end() {
        let mut draft = Draft::new(value());
        draft
            .set_days(DateRange {
                start: Some(CalendarDate::new(2026, 10, 20).unwrap()),
                end: None,
            })
            .unwrap();
        assert_eq!(clock(&draft.value.start), "14:30");
        assert_eq!(clock(&draft.value.end), "15:30");
        assert_eq!(draft.days.end, None);
        draft.value.end = parse("2026-10-20T09:00").unwrap();
        draft
            .set_days(DateRange {
                start: Some(CalendarDate::new(2026, 10, 21).unwrap()),
                end: Some(CalendarDate::new(2026, 10, 21).unwrap()),
            })
            .unwrap();
        assert_eq!(clock(&draft.value.end), "15:30");
    }
}

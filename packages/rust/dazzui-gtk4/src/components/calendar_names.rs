use super::calendar_date::CalendarDate;
use icu_calendar::{Date, Gregorian};
use icu_datetime::{
    DateTimeFormatter, DateTimeFormatterPreferences,
    fieldsets::{E, MDE, YM, YMDE},
    pattern::{DateTimePattern, FixedCalendarDateTimeNames, WeekdayNameLength},
};
use writeable::{TryWriteable, Writeable};
pub(super) struct CalendarNames {
    title: DateTimeFormatter<YM>,
    short_day: DateTimeFormatter<MDE>,
    full: DateTimeFormatter<YMDE>,
    weekday: FixedCalendarDateTimeNames<Gregorian, E>,
    narrow: DateTimePattern,
}
impl CalendarNames {
    pub fn new(locale: Option<&str>) -> Result<Self, String> {
        let languages = gtk::glib::language_names();
        let system = languages
            .first()
            .map(|l| {
                l.split(['.', '@'])
                    .next()
                    .unwrap_or("en-US")
                    .replace('_', "-")
            })
            .unwrap_or_else(|| "en-US".into());
        let locale = locale.unwrap_or(&system);
        let locale = if locale == "C" || locale == "POSIX" {
            "en-US"
        } else {
            locale
        };
        let prefs = locale
            .parse()
            .ok()
            .map(|locale| {
                DateTimeFormatterPreferences::from_locale_strict(&locale)
                    .unwrap_or_else(|prefs| prefs)
            })
            .ok_or_else(|| format!("Invalid calendar locale: {locale}"))?;
        let title = DateTimeFormatter::try_new(prefs, YM::long()).map_err(|e| e.to_string())?;
        let short_day =
            DateTimeFormatter::try_new(prefs, MDE::medium()).map_err(|e| e.to_string())?;
        let full = DateTimeFormatter::try_new(prefs, YMDE::long()).map_err(|e| e.to_string())?;
        let mut weekday = FixedCalendarDateTimeNames::try_new(prefs).map_err(|e| e.to_string())?;
        weekday
            .include_weekday_names(WeekdayNameLength::Narrow)
            .map_err(|e| e.to_string())?;
        Ok(Self {
            title,
            short_day,
            full,
            weekday,
            narrow: "EEEEE".parse().unwrap(),
        })
    }
    pub fn title(&self, date: CalendarDate) -> String {
        let date =
            Date::try_new_iso(date.year, date.month, date.day).expect("Validated calendar day");
        self.title.format(&date).write_to_string().into_owned()
    }
    pub fn short_day(&self, date: CalendarDate) -> String {
        let date =
            Date::try_new_iso(date.year, date.month, date.day).expect("Validated calendar day");
        self.short_day.format(&date).write_to_string().into_owned()
    }
    pub fn full(&self, date: CalendarDate) -> String {
        let date =
            Date::try_new_iso(date.year, date.month, date.day).expect("Validated calendar day");
        self.full.format(&date).write_to_string().into_owned()
    }
    pub fn weekday(&self, date: CalendarDate) -> String {
        let date = Date::try_new_gregorian(date.year, date.month, date.day)
            .expect("Validated calendar day");
        self.weekday
            .with_pattern_unchecked(&self.narrow)
            .format(&date)
            .try_write_to_string()
            .expect("Loaded weekday names")
            .into_owned()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn browser_style_calendar_names() {
        let names = CalendarNames::new(Some("en-US")).unwrap();
        let day = CalendarDate::new(2026, 10, 16).unwrap();
        assert_eq!(names.title(day), "October 2026");
        assert_eq!(names.short_day(day), "Fri, Oct 16");
        assert_eq!(names.full(day), "Friday, October 16, 2026");
        assert_eq!(names.weekday(day), "F");
        let ignored = CalendarNames::new(Some("en-US-u-foo-bar")).unwrap();
        assert_eq!(ignored.title(day), "October 2026");
        assert!(CalendarNames::new(Some("not a locale")).is_err());
        let names = CalendarNames::new(Some("zh-CN")).unwrap();
        assert_eq!(names.title(day), "2026年10月");
        assert_eq!(names.weekday(day), "五");
    }
}

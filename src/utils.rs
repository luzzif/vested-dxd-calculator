use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Weekday};

pub fn get_maximum_to_date(from: &DateTime<Local>) -> eyre::Result<DateTime<Local>> {
    let day = from.day();
    let month = from.month();
    let year = from.year();

    let to_month = if (month + 1) % 13 == 0 {
        1
    } else {
        (month + 1) % 13
    };
    let to_year = if (month == 12) && (to_month == 1) {
        year + 1
    } else {
        year
    };

    let to_month = if to_month < 10 {
        format!("0{to_month}")
    } else {
        to_month.to_string()
    };

    let day = if day < 10 {
        format!("0{day}")
    } else {
        day.to_string()
    };

    Ok(Local.datetime_from_str(
        format!("{day}-{to_month}-{to_year} 00:00").as_str(),
        "%d-%m-%Y %R",
    )?)
}

pub fn get_working_days_in_period(from: &DateTime<Local>, to: &DateTime<Local>) -> u16 {
    let day_duration = Duration::days(1);

    let mut working_days: u16 = 0;
    let mut from_copy = from.clone();
    while from_copy.lt(to) {
        let weekday = from_copy.weekday();
        if (weekday != Weekday::Sat) && (weekday != Weekday::Sun) {
            working_days += 1;
        }
        from_copy = from_copy + day_duration;
    }

    working_days
}

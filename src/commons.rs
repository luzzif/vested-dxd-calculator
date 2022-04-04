use chrono::{DateTime, Local, TimeZone};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ONE_JAN_2022: DateTime<Local> = Local
        .datetime_from_str("01-01-2022 00:00", "%d-%m-%Y %R")
        .unwrap();
    pub static ref USD_DXD_SALARY_POST_2022: [u16; 8] =
        [1500, 2000, 3000, 4000, 5000, 6000, 7500, 9500];
    pub static ref USD_DXD_SALARY_PRE_2022: [u16; 5] = [2000, 3000, 4000, 5000, 6000];
}

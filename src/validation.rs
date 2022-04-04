use crate::commons::ONE_JAN_2022;
use chrono::{DateTime, Local};
use clap::{Command, ErrorKind};

pub fn validate_from_and_to(
    cmd: &mut Command,
    from: &DateTime<Local>,
    to: &DateTime<Local>,
    maximum_to_date: &DateTime<Local>,
) {
    let now = Local::now();

    if to.gt(&now) {
        cmd.error(
            ErrorKind::ArgumentConflict,
            "Invalid to value: it must be in the past",
        )
        .exit();
    }

    if to.gt(maximum_to_date) {
        cmd.error(
            ErrorKind::ArgumentConflict,
            "Invalid to value: the given period cannot span more than a month",
        )
        .exit();
    }

    if from.gt(&now) {
        cmd.error(
            ErrorKind::ArgumentConflict,
            "Invalid from value: it must be in the past",
        )
        .exit();
    }

    if from.ge(to) {
        cmd.error(
            ErrorKind::ArgumentConflict,
            "Inconsistent from and to values",
        )
        .exit();
    }

    if from.ge(to) {
        cmd.error(
            ErrorKind::ArgumentConflict,
            "Inconsistent from and to values",
        )
        .exit();
    }
}

pub fn validate_level(cmd: &mut Command, from: &DateTime<Local>, to: &DateTime<Local>, level: u8) {
    if (from.lt(&ONE_JAN_2022)) && (to.gt(&ONE_JAN_2022)) {
        cmd.error(
            ErrorKind::ArgumentConflict,
            "From and to can't be respectively in 2021 and 2022 due to changes in salary structure. Please split the period in 2 (one exclusively in 2021 and the other exclusively in 2022).",
        )
        .exit();
    }

    if (level == 0)
        || ((from.gt(&ONE_JAN_2022)) && (level > 8))
        || (from.lt(&ONE_JAN_2022) && level > 5)
    {
        cmd.error(ErrorKind::ArgumentConflict, "Invalid level")
            .exit();
    }
}

mod commons;
mod pricing;
mod utils;
mod validation;

use chrono::{Local, TimeZone};
use clap::{CommandFactory, Parser};
use commons::{ONE_JAN_2022, USD_DXD_SALARY_POST_2022, USD_DXD_SALARY_PRE_2022};
use pricing::get_ath_at_date;
use utils::{get_maximum_to_date, get_working_days_in_period};
use validation::{validate_from_and_to, validate_level};

/// Utility program to calculate the amout of vested DXD to ask for in a certain period of time.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Period start date in a dd-mm-yyyy format.
    #[clap(long)]
    from: String,

    /// Period end date in a dd-mm-yyyy format.
    #[clap(long)]
    to: String,

    /// The worker level in the specified period.
    #[clap(long)]
    level: u8,

    /// The worker's full time percentage (1 to 100). If a worker only worked half of the hours in the period, 50 is the value to use here.
    #[clap(long)]
    full_time_percentage: f32,

    /// Whether the specified period represented a trial period for the worker.
    #[clap(long)]
    trial: bool,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut cmd = Args::command();
    let args = Args::parse();
    let from = args.from;
    let to = args.to;

    let parsed_from = Local.datetime_from_str(format!("{from} 00:00").as_str(), "%d-%m-%Y %R")?;
    let parsed_to = Local.datetime_from_str(format!("{to} 00:00").as_str(), "%d-%m-%Y %R")?;

    let maximum_to_date = get_maximum_to_date(&parsed_from)?;

    validate_from_and_to(&mut cmd, &parsed_from, &parsed_to, &maximum_to_date);
    validate_level(&mut cmd, &parsed_from, &parsed_to, args.level);

    let ath = get_ath_at_date(&mut cmd, &parsed_from).await?;

    let working_days_in_period = get_working_days_in_period(&parsed_from, &parsed_to);
    let working_days_in_month = get_working_days_in_period(&parsed_from, &maximum_to_date);
    let working_days_multiplier = working_days_in_period as f32 / working_days_in_month as f32;

    let mut dxd_usd_salary = if parsed_from.lt(&ONE_JAN_2022) {
        USD_DXD_SALARY_PRE_2022[(args.level - 1) as usize] as f32 * working_days_multiplier
    } else {
        USD_DXD_SALARY_POST_2022[(args.level - 1) as usize] as f32 * working_days_multiplier
    };
    if args.trial {
        dxd_usd_salary /= 2.0;
    }
    if args.full_time_percentage < 100.0 {
        dxd_usd_salary = dxd_usd_salary * args.full_time_percentage / 100.0
    }

    println!(
        "DXD owed: {} (ATH at {}: {} USD, USD amount of DXD: {})",
        dxd_usd_salary as f32 / ath,
        from,
        ath,
        dxd_usd_salary
    );

    Ok(())
}

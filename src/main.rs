mod commons;
mod pricing;
mod validation;

use chrono::{Local, TimeZone};
use clap::{CommandFactory, Parser};
use commons::{ONE_JAN_2022, USD_DXD_SALARY_POST_2022, USD_DXD_SALARY_PRE_2022};
use pricing::get_ath_in_range;
use validation::{validate_from_and_to, validate_level};

/// Utility program to calculate the amout of vested DXD to ask for in a certain period of time.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Worker level.
    #[clap(long)]
    level: u8,

    /// Period start date in a dd-mm-yyyy format.
    #[clap(long)]
    from: String,

    /// Period end date in a dd-mm-yyyy format.
    #[clap(long)]
    to: String,

    /// The worker's full time percentage (1 to 100).
    #[clap(long)]
    full_time_percentage: f32,

    /// Whether the worker is currently in a trial period.
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

    validate_from_and_to(&mut cmd, &parsed_from, &parsed_to);
    validate_level(&mut cmd, &parsed_from, &parsed_to, args.level);

    let ath = get_ath_in_range(&mut cmd, &parsed_from, &parsed_to).await?;

    let mut dxd_usd_salary = if parsed_from.lt(&ONE_JAN_2022) {
        USD_DXD_SALARY_PRE_2022[(args.level - 1) as usize]
    } else {
        USD_DXD_SALARY_POST_2022[(args.level - 1) as usize]
    };
    if args.trial {
        dxd_usd_salary /= 2;
    }

    println!(
        "DXD owed: {} (ATH in period: {} USD, USD amount of DXD: {})",
        dxd_usd_salary as f32 / ath,
        ath,
        dxd_usd_salary
    );

    Ok(())
}

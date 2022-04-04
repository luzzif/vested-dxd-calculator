use chrono::{DateTime, Local};
use clap::{Command, ErrorKind};
use reqwest::get;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Price {
    price: f32,
}

#[derive(Deserialize, Debug)]
struct Data {
    prices: Vec<Price>,
}

#[derive(Deserialize, Debug)]
struct Response {
    data: Data,
}

pub async fn get_ath_in_range(
    cmd: &mut Command<'_>,
    from: &DateTime<Local>,
    to: &DateTime<Local>,
) -> eyre::Result<f32> {
    let days_number = to.signed_duration_since(*from).num_days();

    let formatted_from = from.format("%Y-%m-%d");
    let formatted_to = to.format("%Y-%m-%d");

    let url = format!("https://api.covalenthq.com/v1/pricing/historical/USD/DXD/?quote-currency=USD&format=JSON&from={formatted_from}&to={formatted_to}&page-number=0&page-size={days_number}&prices-at-asc=false&key=ckey_fcc79a1d8a22411fa535e64176b");
    let prices = get(url).await?.json::<Response>().await?.data.prices;

    if prices.len() == 0 {
        cmd.error(ErrorKind::Io, "Can't find DXD ATH in the given period.")
            .exit();
    }

    let mut ath = 0.0;
    for wrapped_price in prices {
        if wrapped_price.price > ath {
            ath = wrapped_price.price;
        }
    }

    if ath == f32::INFINITY {
        cmd.error(ErrorKind::Io, "Can't find DXD ATH in the given period.")
            .exit();
    }

    Ok(ath)
}

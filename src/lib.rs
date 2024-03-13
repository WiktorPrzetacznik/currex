use std::collections::HashMap;
use std::fmt::Write;

use rust_decimal::Decimal;

pub use crate::args::Args;
use crate::converter::CurrencyConverter;

mod args;
pub mod converter;

pub fn run(config: Args) {
    let mut conv = CurrencyConverter::load()
        .unwrap_or_default();
    let result = if config.is_list_enabled() {
        get_message_list(&config, conv.get_list_data(config.base_code()))
    } else {
        get_message_pair(&config, conv.get_pair_data(config.base_code(), config.target_code()))
    };
    display_result(result);
    if let Err(err) = conv.save() {
        eprintln!("could not save cache file, reason: {err}")
    }
}

fn display_result(result: converter::Result<String>) {
    match result {
        Ok(data) => println!("{data}"),
        Err(err) => eprintln!("{err}")
    }
}

fn get_message_pair(config: &Args, data: converter::Result<Decimal>) -> converter::Result<String> {
    if data.is_ok() {
        let base = config.base_code();
        let target = config.target_code();
        let rate = data.unwrap();
        let target_amount = config.amount() * rate;
        Ok(format!("conversion result: {target_amount} {target} (rate: {rate}, from {base})"))
    } else {
        let err = data.unwrap_err();
        Err(format!("could not convert, reason: {err}").into())
    }
}

fn get_message_list(config: &Args, data: converter::Result<HashMap<String, Decimal>>) -> converter::Result<String> {
    if data.is_ok() {
        let base = config.base_code();
        let data = data.unwrap()
            .iter()
            .fold(String::from("\n"), |mut info, entry| {
                let key = entry.0;
                let val = entry.1;
                writeln!(info, "{key} : {val}")
                    .expect("string format and write do not fail normally");
                info
            });
        Ok(format!("available currencies with conversion rates (from {base}): {data}"))
    } else {
        let err = data.unwrap_err();
        Err(format!("could not fetch currency list, reason: {err}").into())
    }
}
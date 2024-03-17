use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::converter;

const API_KEY: &str = "3d5aee611eaa045984558a75";
const BASE_URL: &str = "https://v6.exchangerate-api.com/v6/";

pub(super) fn get_pair(base: &String, target: &String) -> converter::Result<Decimal> {
    let url = format!("{BASE_URL}/{API_KEY}/pair/{base}/{target}");
    let response = process_request(url.as_str())?;
    Ok(response.conversion_rate
        .expect("external api ensures proper structure"))
}

pub(super) fn get_list(base: &String) -> converter::Result<HashMap<String, Decimal>> {
    let url = format!("{BASE_URL}/{API_KEY}/latest/{base}");
    let response = process_request(url.as_str())?;
    Ok(response.conversion_rates
        .expect("external api ensures proper structure"))
}

fn process_request(url: &str) -> converter::Result<CurrencyRateResponse> {
    let resp = reqwest::blocking::get(url)?
        .json::<CurrencyRateResponse>()?;
    if resp.result.eq("success") {
        Ok(resp)
    } else {
        Err(resp.error_type
            .expect("external api ensures proper structure")
            .into())
    }
}

#[derive(Deserialize)]
struct CurrencyRateResponse {
    result: String,
    #[serde(alias = "error-type")]
    error_type: Option<String>,
    conversion_rate: Option<Decimal>,
    conversion_rates: Option<HashMap<String, Decimal>>,
}

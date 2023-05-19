use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use crate::domain::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Obj {
    pub meta: Meta,
    pub values: Vec<Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub symbol: String,
    pub interval: String,
    pub currency_base: String,
    pub currency_quote: String,
    pub exchange: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    pub datetime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    #[serde(skip_serializing)]
    pub diff: Option<f32>
}

#[derive(Deserialize)]
pub struct TwelveError {
    pub message: String,
}

pub fn format_endpoint(api_key: String, symbol: String, start: String, end: String, interval: String) -> String {
    format!("https://api.twelvedata.com/time_series?&previous_close=true&timezone=America/Sao_Paulo&outputsize=5000&dp=8&apikey={api_key}&symbol={symbol}&start_date={start}&end_date={end}&interval={interval}&order=ASC")
}

pub fn greater_change_than(open: &Value, close: &Value) -> (bool, f32) {

    let open_date: DateTime<Utc> = Utc.datetime_from_str(&open.datetime, "%Y-%m-%d %H:%M:%S").unwrap();
    let close_date: DateTime<Utc> = Utc.datetime_from_str(&close.datetime, "%Y-%m-%d %H:%M:%S").unwrap();

    if open_date > close_date {
        let open_value: f32 = open.open.parse().unwrap();
        let close_value: f32 = close.close.parse().unwrap();

        let diff: f32 = utils::calculate_percentage_change(close_value, open_value);

        return (true, diff);
    }

    let open_value: f32 = open.open.parse().unwrap();
    let close_value: f32 = close.close.parse().unwrap();

    let diff: f32 = utils::calculate_percentage_change(open_value, close_value);

    return (false, diff);

}
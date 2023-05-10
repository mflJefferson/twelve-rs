use serde::{Deserialize, Serialize};

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
    pub exchange: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    pub datetime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub previous_close: String,
    #[serde(skip_serializing)]
    pub diff: Option<f32>
}

pub fn format_endpoint(api_key: String, symbol: String, start: String, end: String, interval: String) -> String {
    format!("https://api.twelvedata.com/time_series?&interval=5min&previous_close=true&timezone=America/Sao_Paulo&outputsize=5000&dp=8&apikey={api_key}&symbol={symbol}&start_date={start}&end_date={end}&interval={interval}&order=ASC")
}
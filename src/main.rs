use serde::{Deserialize, Serialize};

use percentage_change_calculator::calculate_percentage_change;

#[macro_use]
extern crate dotenv_codegen;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = dotenv!("TWELVE_SECRET").to_owned();
    let mut url: String = "https://api.twelvedata.com/".to_owned();
    let mut endpoint :String = "time_series?symbol=ETH/BTC:Huobi&interval=5min&previous_close=true&date=today&timezone=America/Sao_Paulo&outputsize=3000&dp=8&apikey=".to_owned();

    endpoint.push_str(&api_key);
    url.push_str(&endpoint);

    println!("{:?}", url);

    let resp = reqwest::get(url)
        .await?;

    let text = resp.text().await?;
    let root: Obj = serde_json::from_str::<Obj>(&text).unwrap();
    for mut i in root.values {
        let close: f32 = i.close.parse().unwrap();
        let previous_close: f32 = i.previous_close.parse().unwrap();
        
        let diff: f32 = calculate_percentage_change(previous_close, close);
        i.diff = Some(diff);
        println!("{:#?}", i);
    }
    
    Ok(())
}
mod command_error;
use log4rs;
use clap::Parser;
use log::error;
use serde_yaml;
use command_error::CommandError;

use core_service;
use core_service::domain::fetch_record;

#[macro_use]
extern crate dotenv_codegen;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    symbol: String,
    #[arg(long)]
    start: String,
    #[arg(long)]
    end: String,
    #[arg(short, long)]
    interval: String,
    #[arg(long)]
    exchange: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), CommandError> {
    let config_str = include_str!("config/log4rs.yaml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();

    let cli = Cli::parse();
    let api_key: String = dotenv!("TWELVE_SECRET").to_owned();

    let url :String = fetch_record::format_endpoint(api_key, cli.symbol, cli.start, cli.end, cli.interval, cli.exchange);
    println!("{:?}", url);

    let resp = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(e) => {
            error!("{}", e);
            error!("{}", url);
            return Err(CommandError::ReqwestError(e))
        }
    };


    let text = match resp.text().await {
        Ok(text) => text,
        Err(e) => {
            error!("{}", e);
            error!("{}", url);
            return Err(CommandError::ReqwestError(e))
        }
    };

    let root: fetch_record::Obj = match serde_json::from_str::<fetch_record::Obj>(&text) {
        Ok(root) => root,
        Err(e) => {
            let json_error = serde_json::from_str::<fetch_record::TwelveError>(&text).unwrap();
            error!("{}", e);
            error!("{}", url);
            error!("{}", json_error.message);
            return Err(CommandError::SerdeJsonError(e))
        }
    };

    let start = root.values.first().unwrap();
    let end = root.values.last().unwrap();

    let max_open = root.values.iter().max_by_key(|v| &v.open).unwrap();
    let min_close = root.values.iter().min_by_key(|v| &v.close).unwrap();

    let min_open = root.values.iter().min_by_key(|v| &v.open).unwrap();
    let max_close = root.values.iter().max_by_key(|v| &v.close).unwrap();

    let (_is_is_greater_1, diff_1) = fetch_record::greater_change_than(max_open, min_close);
    let (_is_is_greater_2, diff_2) = fetch_record::greater_change_than(min_open, max_close);

    let open_date: String;
    let open_value: String;
    let close_date: String;
    let close_value: String;
    let diff = if diff_1.abs() > diff_2.abs() {
        println!("Diff 1 = {}", diff_1);
        println!("Usando max_open data: {} valor: {}", max_open.datetime, max_open.open);
        println!("Usando min_close data: {} valor: {}", min_close.datetime, min_close.close);
        open_date = max_open.datetime.clone();
        open_value = max_open.open.clone();
        close_date = min_close.datetime.clone();
        close_value = min_close.close.clone();
        diff_1
    } else {
        println!("Diff 2 = {}", diff_2);
        println!("Usando min_open data: {} valor: {}", min_open.datetime, min_open.open);
        println!("Usando max_close data: {} valor: {}", max_close.datetime, max_close.close);
        open_date = min_open.datetime.clone();
        open_value = min_open.open.clone();
        close_date = max_close.datetime.clone();
        close_value = max_close.close.clone();
        diff_2
    };

    match core_service::repository::timeseries::insert(root.meta.symbol, &start.datetime, &end.datetime, diff, open_date, open_value, close_date, close_value, root.meta.exchange).await {
        Ok(n) => n,
        Err(e) => {
            error!("{:?}", e);
            error!("{}", url);
            return Err(CommandError::SqlxError(e))
        }
    };

    Ok(())
}
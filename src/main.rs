use clap::Parser;

use percentage_change_calculator::calculate_percentage_change;
mod twelve_api;
mod db;

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let api_key: String = dotenv!("TWELVE_SECRET").to_owned();

    let url :String = twelve_api::format_endpoint(api_key, cli.symbol, cli.start, cli.end, cli.interval);
    println!("{:?}", url);

    let resp = reqwest::get(url)
        .await?;

    let text = resp.text().await?;
    let root: twelve_api::Obj = serde_json::from_str::<twelve_api::Obj>(&text).unwrap();

    let start = root.values.first().unwrap();
    let end = root.values.last().unwrap();

    let open: f32 = start.open.parse().unwrap();
    let close: f32 = end.close.parse().unwrap();

    let diff: f32 = calculate_percentage_change(open, close);

    db::insert(root.meta.symbol, &start.datetime, &end.datetime, diff).await?;

    Ok(())
}
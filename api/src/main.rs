mod error;

use std::fmt::Debug;
pub use self::error::{Error, Result};

use axum::{
    routing::get,
    Router,
    response::Json,
    extract::Query
};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use core_service;
use crate::core_service::model::tw_queries::TwQuery;

//use serde_json::{Value, json};

#[derive(Serialize, Deserialize, Debug)]
struct TimeseriesParams {
    symbol: Option<String>,
    start: Option<String>,
    end: Option<String>,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/timeseries", get(get_timeseries));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_timeseries(params: Query<TimeseriesParams>) -> impl IntoResponse {
    let symbol = params.symbol.as_deref().unwrap_or("BTC/USD");
    let start = params.symbol.as_deref().unwrap_or("2023-05-10 00:00:00");
    let end = params.symbol.as_deref().unwrap_or("2023-05-10 23:59:59");

    let timeseries = match core_service::repository::timeseries::retrieve(symbol, start, end).await {
        Ok(timeseries) => timeseries,
        Err(e) => {
            eprintln!("failed: {:?}", e);
            return Err(Error::FetchFail)
        }
    };



    Ok(Json(timeseries))
}
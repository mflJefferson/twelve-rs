use chrono::{NaiveDateTime};
use rust_decimal::Decimal;
use sqlx::{FromRow};
use serde::Serialize;

#[derive(FromRow, Serialize)]
pub struct TwQuery {
    id: u64,
    symbol: String,
    start_period: NaiveDateTime,
    end_period: NaiveDateTime,
    diff_percentage: Decimal,
    open_date: NaiveDateTime,
    open_value: Decimal,
    close_date: NaiveDateTime,
    close_value: Decimal,
    created_at: NaiveDateTime,
}
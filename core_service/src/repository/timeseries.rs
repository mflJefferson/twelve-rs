use sqlx::mysql::MySqlPoolOptions;
use crate::model::tw_queries;

pub async fn insert(symbol: String, start: &String, end: &String, diff: f32, open_date: String, open_value: String, close_date: String, close_value: String) -> Result<(), sqlx::Error>{
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await?;
    
    sqlx::query("INSERT INTO tw_queries(symbol, start_period, end_period, diff_percentage, open_date, open_value, close_date, close_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(symbol)
        .bind(start)
        .bind(end)
        .bind(diff)
        .bind(open_date)
        .bind(open_value)
        .bind(close_date)
        .bind(close_value)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn retrieve(symbol: impl Into<String>, start: impl Into<String>, end: impl Into<String>) -> Result<Vec<tw_queries::TwQuery>, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await?;

    let result : Vec<tw_queries::TwQuery> = sqlx::query_as(
        r#"
        SELECT *
        FROM `tw_queries` tq
        WHERE tq.symbol = ?
        AND tq.open_date BETWEEN ? AND ?
        "#
    )
        .bind(symbol.into())
        .bind(start.into())
        .bind(end.into())
        .fetch_all(&pool)
        .await?;

    return Ok(result);
}
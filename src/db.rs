use sqlx::mysql::MySqlPoolOptions;

pub async fn insert(symbol: String, start: &String, end: &String, diff: f32) -> Result<(), sqlx::Error>{
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await?;
    
    sqlx::query("INSERT INTO tw_queries(symbol, start_period, end_period, diff_percentage) VALUES (?, ?, ?, ?)")
        .bind(symbol)
        .bind(start)
        .bind(end)
        .bind(diff)
        .execute(&pool)
        .await?;

    Ok(())
}
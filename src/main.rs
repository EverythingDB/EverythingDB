use std::error::Error;
use sqlx::Row;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>>{
    dotenvy::dotenv().ok();

    let username = env::var("DB_USERNAME")?;
    let password = env::var("DB_PASSWORD")?;
    let database = env::var("DB_NAME")?;
    let host = env::var("DB_HOST")?;
    let port = env::var("DB_PORT")?;

    let options = PgConnectOptions::new()
        .host(&host)
        .port(port.parse()?)
        .username(&username)
        .password(&password)
        .database(&database);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    
    let res = sqlx::query("SELECT 1+1 as sum")
        .fetch_one(&pool)
        .await?;
    
    let sum: i32 = res.get("sum");
    println!("1+1 = {}", sum);
    Ok(())
}
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;

use std::error::Error;
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
    
    //let mut tx = pool.begin().await?;

    Ok(())
}
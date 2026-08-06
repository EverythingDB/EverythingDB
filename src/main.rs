use std::error::Error;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::traits::HasID;
use crate::traits::HasTitle;

mod structs;
mod traits;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenvy::dotenv().ok();

    let media: structs::media::Media = structs::media::Media::new(1, "some show");

    let book= structs::book::Book::new(media, "James".to_string(), 42);

    println!("title: {}, author: {}, pages: {}, id: {}", book.title(), book.author, book.page_count, book.id());

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
    
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}
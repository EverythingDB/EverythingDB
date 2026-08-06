use std::error::Error;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use std::env;

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query: &str = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn update(
    book: &Book, isbn: &str, pool: &sqlx::PgPool
) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = %1, author = $2 WHERE isbn = $3";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

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
    
    sqlx::migrate!("./migrations").run(&pool).await?;

    let book = Book{
        title: "Salem's Lot".to_string(),
        author: "Stephen King".to_string(),
        isbn: "978-0-385-00751-1".to_string(),
    };

    create(&book, &pool).await?;

    Ok(())
}
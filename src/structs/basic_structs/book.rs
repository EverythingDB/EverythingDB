use sqlx::query_scalar;

use crate::structs::property_structs::print::Print;
use crate::{structs::macros, traits::{HasAuthor, HasID, HasISBN, HasPageCount, HasSynopsis, HasTitle, Insertable}};


#[derive(Debug)]
pub struct Book {
    pub print: Print,
    pub isbn: String,
    pub author: String,
    pub page_count: i32,
}

impl Book{
    pub fn new(print: Print, isbn:String, author: String, page_count: i32) -> Self{
        Book{
            print: print,
            isbn: isbn,
            author: author,
            page_count: page_count
        }
    }
}

impl Insertable for Book {
    async fn insert<'e, E>(&self, executor: E) -> Result<i32, sqlx::Error>
    where E: sqlx::Executor<'e, Database = sqlx::Postgres>
    {
        let id = query_scalar!(
            "INSERT INTO book (media_id, isbn, author,page_count) VALUES ($1, $2, $3, $4) RETURNING media_id",
            self.id(), self.isbn(), self.author(), self.page_count())
            .fetch_one(executor)
            .await?;
        
        Ok(id)
    }
}

has_property!(
    Book => {
        [HasID, id, Option<i32>, print.media.id],
        [HasTitle, title, &str, print.media.title.as_str()],
        [HasSynopsis, synopsis, &str, print.media.synopsis.as_str()],
        [HasISBN, isbn, &str, isbn.as_str()],
        [HasAuthor, author, &str, author.as_str()],
        [HasPageCount, page_count, i32, page_count]
    }
);
use sqlx::query_scalar;

use crate::structs::print::Print;
use crate::traits::{HasAuthor, HasID, HasISBN, HasPageCount, HasSynopsis, HasTitle, Insertable};


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

impl HasID for Book {
    fn id(&self) -> Option<i32> {
        self.print.media.id
    }
}
impl HasTitle for Book {
    fn title(&self) -> &str {
        self.print.media.title.as_str()
    }   
}
impl HasSynopsis for Book {
    fn synopsis(&self) -> &str {
        self.print.synopsis()
    }
}
impl HasISBN for Book {
    fn isbn(&self) -> &str {
        &self.isbn.as_str()
    }
}
impl HasAuthor for Book {
    fn author(&self) -> &str {
        self.author.as_str()
    }
}
impl HasPageCount for Book {
    fn page_count(&self) -> i32 {
        self.page_count
    }
}

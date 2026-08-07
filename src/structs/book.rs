use sqlx::{PgPool};

use crate::structs::media::Media;
use crate::traits::{HasAuthor, HasID, HasISBN, HasPageCount, HasTitle, PublicTable};


#[derive(Debug)]
pub struct Book {
    pub media: Media,
    pub isbn: String,
    pub author: String,
    pub page_count: u32,
}

impl Book{
    pub fn new(media: Media, isbn:String, author: String, page_count: u32) -> Self{
        Book{
            media: media,
            isbn: isbn,
            author: author,
            page_count: page_count
        }
    }
}

impl PublicTable for Book {   
    async fn insert(&self, pool: &PgPool) -> Result<u32, sqlx::Error> {
        todo!()
    }
    async fn delete(&self, pool: &PgPool) {
        todo!()
    }
}

impl HasID for Book {
    fn id(&self) -> Option<u32> {
        self.media.id
    }
}
impl HasTitle for Book {
    fn title(&self) -> &str {
        self.media.title.as_str()
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
    fn page_count(&self) -> u32 {
        self.page_count
    }
}

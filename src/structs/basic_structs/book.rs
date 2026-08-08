use sqlx::{query_scalar, Postgres, Transaction};

use crate::structs::property_structs::print::Print;
use crate::{traits::{HasAuthor, HasID, HasISBN, HasPageCount, HasSynopsis, HasTitle, Insertable}};


#[derive(Debug)]
pub struct Book {
    print: Print,
    isbn: String,
    author: String,
    page_count: i32,
}

impl Book{
    pub fn new(
        id: Option<i32>, title: String, synopsis: String,
        isbn:String, author: String, page_count: i32) -> Self{
            let print = Print::new(id, title, synopsis);
            Book {
                print: print,
                isbn: isbn,
                author: author,
                page_count: page_count
            }
    }
}

impl Insertable for Book {
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let print_id = self.print.insert(tx).await?;

        query_scalar!(
            r#"
            INSERT INTO book (media_id, isbn)
            VALUES ($1, $2)
            RETURNING media_id
            "#,
            print_id,
            self.isbn
        )
        .fetch_one(&mut **tx)
        .await
    }
}

has_property!(
    Book => {
        [HasID, id, Option<i32>, print.id()],
        [HasTitle, title, &str, print.title()],
        [HasSynopsis, synopsis, &str, print.synopsis()],
        [HasISBN, isbn, &str, isbn.as_str()],
        [HasAuthor, author, &str, author.as_str()],
        [HasPageCount, page_count, i32, page_count]
    }
);
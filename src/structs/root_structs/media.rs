use sqlx::query_scalar;

use crate::traits::{Deletable, HasID, HasSynopsis, HasTitle, Insertable};

#[derive(Debug)]
pub struct Media {
    pub id: Option<i32>,
    pub title: String,
    pub synopsis: String
}

impl Media {
    /// Set id as None unless reading from the db
    pub fn new(id: Option<i32>, title: String, synopsis: String) -> Self{        
        Media{
            id: id,
            title: title,
            synopsis: synopsis
        }
    }
}

impl Insertable for Media {
    async fn insert<'e, E>(&self, executor: E) -> Result<i32, sqlx::Error>
    where E: sqlx::Executor<'e, Database = sqlx::Postgres>
    {
        let id = query_scalar!(
            "INSERT INTO media (title, synopsis) VALUES ($1, $2) RETURNING id",
            self.title(), self.synopsis())
            .fetch_one(executor)
            .await?;
        
        Ok(id)
    }
}
impl Deletable for Media {
    async fn delete<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where E: sqlx::Executor<'e, Database = sqlx::Postgres>
    {
        todo!()
    }
}

has_property!(
    Media => {
        [HasID, id, Option<i32>, id],
        [HasTitle, title, &str, title.as_str()],
        [HasSynopsis, synopsis, &str, synopsis.as_str()]
    }
);
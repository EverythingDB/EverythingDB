use sqlx::query_scalar;

use crate::{structs::media::Media, traits::{HasID, HasSynopsis, HasTitle, Insertable}};

#[derive(Debug)]
pub struct Print {
    pub media: Media
}

impl Print {
    pub fn new(media: Media) -> Self{
        Print{
            media: media
        }
    }
}

impl Insertable for Print {
    async fn insert<'e, E>(&self, executor: E) -> Result<i32, sqlx::Error>
    where E: sqlx::Executor<'e, Database = sqlx::Postgres>
    {
        let id = query_scalar!(
            "INSERT INTO print (media_id) VALUES ($1) RETURNING media_id",
            self.media.id)
            .fetch_one(executor)
            .await?;
        
        Ok(id)
    }
}

impl HasID for Print {
    fn id(&self) -> Option<i32> {
        self.media.id
    }
}
impl HasTitle for Print {
    fn title(&self) -> &str {
        self.media.synopsis()
    }
}
impl HasSynopsis for Print {
    fn synopsis(&self) -> &str {
        self.media.synopsis()
    }
}

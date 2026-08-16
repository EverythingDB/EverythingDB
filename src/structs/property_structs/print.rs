use sqlx::{query_scalar, Postgres, Transaction};

use crate::structs::root_structs::media::*;
use crate::{traits::{Insertable}};

#[derive(Debug)]
pub struct Print {
    media: Media
}

impl Print {
    pub fn new(
        id: Option<i32>, title: String, synopsis:String) -> Self{
        let media = Media::new(id, title, synopsis);
        Print { 
            media: media
        }
    }
}

impl Insertable for Print {
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let media_id = self.media.insert(tx).await?;

        query_scalar!(
            r#"
            INSERT INTO print (media_id)
            VALUES ($1)
            RETURNING media_id
            "#,
            media_id
        )
        .fetch_one(&mut **tx)
        .await
    }
}

has_property!(
    Print => {
        [HasID, id, Option<&i32>, media.id()],
        [HasTitle, title, &str, media.title()],
        [HasSynopsis, synopsis, &str, media.synopsis()]
    }
);

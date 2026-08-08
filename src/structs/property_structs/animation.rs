use sqlx::{query_scalar, Postgres, Transaction};

use crate::{structs::root_structs::media::Media, traits::Insertable};

#[derive(Debug)]
pub struct Animation{
    media: Media,
    animators: Vec<String>
}

impl Animation {
    pub fn new(
        id: Option<i32>, title: String, synopsis: String,
        animators: Vec<String>) -> Self{
            let media = Media::new(id, title, synopsis);
            Animation { media, animators }
        }
}

impl Insertable for Animation{
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
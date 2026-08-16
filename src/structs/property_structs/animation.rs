use sqlx::{query_scalar, Postgres, Transaction};

use crate::structs::root_structs::media::*;

use crate::{traits::Insertable};

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
            INSERT INTO animation (media_id, animators)
            VALUES ($1, $2)
            RETURNING media_id
            "#,
            media_id,
            &self.animators
        )
        .fetch_one(&mut **tx)
        .await
    }
}

pub trait HasAnimators {
    fn animators(&self) -> &Vec<String>;
}

has_property!(
    Animation => {
        [HasID, id, Option<&i32>, media.id()],
        [HasTitle, title, &str, media.title()],
        [HasSynopsis, synopsis, &str, media.synopsis()],
        [HasAnimators, animators, &Vec<String>, animators.as_ref()]
    }
);
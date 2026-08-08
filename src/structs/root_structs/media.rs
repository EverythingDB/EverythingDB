use sqlx::{query, query_scalar, Postgres, Transaction};

use crate::traits::{Deletable, HasID, HasSynopsis, HasTitle, Insertable};

#[derive(Debug)]
pub struct Media {
    id: Option<i32>,
    title: String,
    synopsis: String
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
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let id = query_scalar!(
            r#"
            INSERT INTO media (title)
            VALUES ($1)
            RETURNING id
            "#,
            self.title
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }
}
impl Deletable for Media {
    async fn delete(
        id: i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error>
    {
        query!(
            "DELETE FROM media WHERE id = $1",
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

has_property!(
    Media => {
        [HasID, id, Option<i32>, id],
        [HasTitle, title, &str, title.as_str()],
        [HasSynopsis, synopsis, &str, synopsis.as_str()]
    }
);
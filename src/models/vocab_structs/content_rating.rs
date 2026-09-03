use serde::{Serialize, Deserialize};

use crate::models::traits::RootStruct;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentRating {
    id:               Option<i32>,
    rating_system_id: i32,
    code:             String, // 'M', '18', 'PG-13', 'Z'
    label:            Option<String>,
    minimum_age:      Option<i16>,
    sort_order:       i16
}

#[allow(dead_code)]
pub trait HasContentRating{
    fn content_rating(&self) -> &ContentRating;

    fn id(&self) -> Option<&i32>;
    fn rating_system_id(&self) -> &i32;
    fn code(&self) -> &str;
    fn label(&self) -> Option<&str>;
    fn minimum_age(&self) -> Option<&i16>;
    fn sort_order(&self) -> &i16;
}

impl HasContentRating for ContentRating {
    fn content_rating(&self) -> &ContentRating { self }

    fn id(&self) -> Option<&i32> { self.id.as_ref() }
    fn rating_system_id(&self) -> &i32 { &self.rating_system_id }
    fn code(&self) -> &str { &self.code }
    fn label(&self) -> Option<&str> { self.label.as_deref() }
    fn minimum_age(&self) -> Option<&i16> { self.minimum_age.as_ref() }
    fn sort_order(&self) -> &i16 { &self.sort_order }
}

impl RootStruct for ContentRating {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query_scalar!(
        r#"
            INSERT INTO content_rating (
                rating_system_id, code, label, minimum_age, sort_order
                )
                VALUES (
                $1, $2, $3, $4, $5
                )
                RETURNING id AS "id!"
            "#,
            self.rating_system_id(), self.code(), self.label(), self.minimum_age(), self.sort_order()
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    async fn delete(
        id: i32,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM content_rating WHERE id = $1"#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

impl Default for ContentRating {
    fn default() -> Self {
        ContentRating {
            id: None,
            rating_system_id: 1, code: String::from("M"),
            label: Some(String::from("Mature 17+")), minimum_age: Some(17), sort_order: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::vocab_structs::rating_system::RatingSystem;
    use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
    use std::env;
    use tokio::sync::OnceCell;

    static POOL: OnceCell<PgPool> = OnceCell::const_new();

    async fn pool() -> &'static PgPool {
        POOL.get_or_init(|| async {
            dotenvy::dotenv().ok();

            let username = env::var("DB_USERNAME").unwrap();
            let password = env::var("DB_PASSWORD").unwrap();
            let database = env::var("DB_NAME").unwrap();
            let host = env::var("DB_HOST").unwrap();
            let port = env::var("DB_PORT").unwrap();

            let options = PgConnectOptions::new()
                .host(&host)
                .port(port.parse().unwrap())
                .username(&username)
                .password(&password)
                .database(&database);

            PgPoolOptions::new()
                .max_connections(5)
                .connect_with(options)
                .await
                .unwrap()
        })
        .await
    }

    #[tokio::test]
    async fn test_insert_and_delete() -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = pool().await.begin().await?;
        let rating_system = RatingSystem::default();
        let rating_system_id = rating_system.insert(&mut tx).await?;

        let mut content_rating = ContentRating::default();
        content_rating.rating_system_id = rating_system_id;
        let id = content_rating.insert(&mut tx).await?;

        ContentRating::delete(id, &mut tx).await?;
        RatingSystem::delete(rating_system_id, &mut tx).await?;

        Ok(())
    }
}

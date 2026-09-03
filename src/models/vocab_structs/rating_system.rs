use serde::{Serialize, Deserialize};

use crate::models::traits::RootStruct;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RatingSystem {
    id:         Option<i32>,
    slug:       String,
    name:       String, // ESRB, PEGI, MPAA, CERO, BBFC, TV Parental
    country_id: Option<i32>,
    applies_to: Option<String>
}

#[allow(dead_code)]
pub trait HasRatingSystem{
    fn rating_system(&self) -> &RatingSystem;

    fn id(&self) -> Option<&i32>;
    fn slug(&self) -> &str;
    fn name(&self) -> &str;
    fn country_id(&self) -> Option<&i32>;
    fn applies_to(&self) -> Option<&str>;
}

impl HasRatingSystem for RatingSystem {
    fn rating_system(&self) -> &RatingSystem { self }

    fn id(&self) -> Option<&i32> { self.id.as_ref() }
    fn slug(&self) -> &str { &self.slug }
    fn name(&self) -> &str { &self.name }
    fn country_id(&self) -> Option<&i32> { self.country_id.as_ref() }
    fn applies_to(&self) -> Option<&str> { self.applies_to.as_deref() }
}

impl RootStruct for RatingSystem {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query_scalar!(
        r#"
            INSERT INTO rating_system (
                slug, name, country_id, applies_to
                )
                VALUES (
                $1, $2, $3, $4
                )
                RETURNING id AS "id!"
            "#,
            self.slug(), self.name(), self.country_id(), self.applies_to()
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
            r#"DELETE FROM rating_system WHERE id = $1"#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

impl Default for RatingSystem {
    fn default() -> Self {
        RatingSystem {
            id: None,
            slug: String::from("esrb"), name: String::from("ESRB"),
            country_id: None, applies_to: Some(String::from("games"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let id = rating_system.insert(&mut tx).await?;
        RatingSystem::delete(id, &mut tx).await?;

        Ok(())
    }
}

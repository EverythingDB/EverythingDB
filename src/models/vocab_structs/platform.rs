use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

use crate::models::{enums::common::PlatformKind, traits::RootStruct};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Platform {
    id:              Option<i32>,
    slug:            String,
    name:            String,
    kind:            PlatformKind,
    manufacturer_id: Option<i32>,
    released_on:     Option<NaiveDate>,
    discontinued_on: Option<NaiveDate>,
    generation:      Option<i16>
}

#[allow(dead_code)]
pub trait HasPlatform{
    fn platform(&self) -> &Platform;

    fn id(&self) -> Option<&i32>;
    fn slug(&self) -> &str;
    fn name(&self) -> &str;
    fn kind(&self) -> &PlatformKind;
    fn manufacturer_id(&self) -> Option<&i32>;
    fn released_on(&self) -> Option<&NaiveDate>;
    fn discontinued_on(&self) -> Option<&NaiveDate>;
    fn generation(&self) -> Option<&i16>;
}

impl HasPlatform for Platform {
    fn platform(&self) -> &Platform { self }

    fn id(&self) -> Option<&i32> { self.id.as_ref() }
    fn slug(&self) -> &str { &self.slug }
    fn name(&self) -> &str { &self.name }
    fn kind(&self) -> &PlatformKind { &self.kind }
    fn manufacturer_id(&self) -> Option<&i32> { self.manufacturer_id.as_ref() }
    fn released_on(&self) -> Option<&NaiveDate> { self.released_on.as_ref() }
    fn discontinued_on(&self) -> Option<&NaiveDate> { self.discontinued_on.as_ref() }
    fn generation(&self) -> Option<&i16> { self.generation.as_ref() }
}

impl RootStruct for Platform {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query_scalar!(
        r#"
            INSERT INTO platform (
                slug, name, kind, manufacturer_id, released_on, discontinued_on, generation
                )
                VALUES (
                $1, $2, $3::platform_kind, $4, $5, $6, $7
                )
                RETURNING id AS "id!"
            "#,
            self.slug(), self.name(), self.kind() as &PlatformKind, self.manufacturer_id(), self.released_on(), self.discontinued_on(), self.generation()
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
            r#"DELETE FROM platform WHERE id = $1"#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

impl Default for Platform {
    fn default() -> Self {
        Platform {
            id: None,
            slug: String::from("pc"), name: String::from("PC"),
            kind: PlatformKind::Computer, manufacturer_id: None,
            released_on: None, discontinued_on: None, generation: None
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
        let platform = Platform::default();
        let id = platform.insert(&mut tx).await?;
        Platform::delete(id, &mut tx).await?;

        Ok(())
    }
}

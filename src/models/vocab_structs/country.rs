use serde::{Serialize, Deserialize};

use crate::models::traits::RootStruct;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Country {
    id:          Option<i32>,
    iso_3166_1:  String,
    name:        String,
    native_name: Option<String>,
    region:      Option<String>
}

#[allow(dead_code)]
pub trait HasCountry{
    fn country(&self) -> &Country;

    fn id(&self) -> Option<&i32>;
    fn iso_3166_1(&self) -> &str;
    fn name(&self) -> &str;
    fn native_name(&self) -> Option<&str>;
    fn region(&self) -> Option<&str>;
}

impl HasCountry for Country {
    fn country(&self) -> &Country { self }

    fn id(&self) -> Option<&i32> { self.id.as_ref() }
    fn iso_3166_1(&self) -> &str { &self.iso_3166_1 }
    fn name(&self) -> &str { &self.name }
    fn native_name(&self) -> Option<&str> { self.native_name.as_deref() }
    fn region(&self) -> Option<&str> { self.region.as_deref() }
}

impl RootStruct for Country {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query_scalar!(
        r#"
            INSERT INTO country (
                iso_3166_1, name, native_name, region
                )
                VALUES (
                $1, $2, $3, $4
                )
                RETURNING id AS "id!"
            "#,
            self.iso_3166_1(), self.name(), self.native_name(), self.region()
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
            r#"DELETE FROM country WHERE id = $1"#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

impl Default for Country {
    fn default() -> Self {
        Country {
            id: None,
            iso_3166_1: String::from("JP"), name: String::from("Japan"),
            native_name: Some(String::from("日本")), region: Some(String::from("Asia"))
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
        let country = Country::default();
        let id = country.insert(&mut tx).await?;
        Country::delete(id, &mut tx).await?;

        Ok(())
    }
}

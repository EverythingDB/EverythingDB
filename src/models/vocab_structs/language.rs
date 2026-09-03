use serde::{Serialize, Deserialize};

use crate::models::{enums::print::ReadingDirection, traits::RootStruct};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Language {
    id:          Option<i32>,
    iso_639_1:   Option<String>,
    iso_639_3:   String,
    name:        String,
    native_name: Option<String>,
    script:      Option<String>,
    direction:   ReadingDirection
}

#[allow(dead_code)]
pub trait HasLanguage{
    fn language(&self) -> &Language;

    fn id(&self) -> Option<&i32>;
    fn iso_639_1(&self) -> Option<&str>;
    fn iso_639_3(&self) -> &str;
    fn name(&self) -> &str;
    fn native_name(&self) -> Option<&str>;
    fn script(&self) -> Option<&str>;
    fn direction(&self) -> &ReadingDirection;
}

impl HasLanguage for Language {
    fn language(&self) -> &Language { self }

    fn id(&self) -> Option<&i32> { self.id.as_ref() }
    fn iso_639_1(&self) -> Option<&str> { self.iso_639_1.as_deref() }
    fn iso_639_3(&self) -> &str { &self.iso_639_3 }
    fn name(&self) -> &str { &self.name }
    fn native_name(&self) -> Option<&str> { self.native_name.as_deref() }
    fn script(&self) -> Option<&str> { self.script.as_deref() }
    fn direction(&self) -> &ReadingDirection { &self.direction }
}

impl RootStruct for Language {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query_scalar!(
        r#"
            INSERT INTO language (
                iso_639_1, iso_639_3, name, native_name, script, direction
                )
                VALUES (
                $1, $2, $3, $4, $5, $6::reading_direction
                )
                RETURNING id AS "id!"
            "#,
            self.iso_639_1(), self.iso_639_3(), self.name(), self.native_name(), self.script(), self.direction() as &ReadingDirection
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
            r#"DELETE FROM language WHERE id = $1"#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

impl Default for Language {
    fn default() -> Self {
        Language {
            id: None,
            iso_639_1: Some(String::from("ja")), iso_639_3: String::from("jpn"),
            name: String::from("Japanese"), native_name: Some(String::from("日本語")),
            script: Some(String::from("Kanji/Kana")), direction: ReadingDirection::Ltr
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
        let language = Language::default();
        let id = language.insert(&mut tx).await?;
        Language::delete(id, &mut tx).await?;

        Ok(())
    }
}

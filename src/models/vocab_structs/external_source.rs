use serde::{Serialize, Deserialize};

use crate::models::traits::RootStruct;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalSource {
    id:               Option<i32>,
    slug:             String, // anilist, mal, imdb, vndb, igdb, isbndb, musicbrainz, tmdb, bgg, openlibrary, discogs
    name:             String,
    base_url:         Option<String>,
    url_template:     Option<String>, // e.g. 'https://anilist.co/anime/{id}'
    is_authoritative: bool
}

#[allow(dead_code)]
pub trait HasExternalSource{
    fn external_source(&self) -> &ExternalSource;

    fn id(&self) -> Option<&i32>;
    fn slug(&self) -> &str;
    fn name(&self) -> &str;
    fn base_url(&self) -> Option<&str>;
    fn url_template(&self) -> Option<&str>;
    fn is_authoritative(&self) -> &bool;
}

impl HasExternalSource for ExternalSource {
    fn external_source(&self) -> &ExternalSource { self }

    fn id(&self) -> Option<&i32> { self.id.as_ref() }
    fn slug(&self) -> &str { &self.slug }
    fn name(&self) -> &str { &self.name }
    fn base_url(&self) -> Option<&str> { self.base_url.as_deref() }
    fn url_template(&self) -> Option<&str> { self.url_template.as_deref() }
    fn is_authoritative(&self) -> &bool { &self.is_authoritative }
}

impl RootStruct for ExternalSource {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query_scalar!(
        r#"
            INSERT INTO external_source (
                slug, name, base_url, url_template, is_authoritative
                )
                VALUES (
                $1, $2, $3, $4, $5
                )
                RETURNING id AS "id!"
            "#,
            self.slug(), self.name(), self.base_url(), self.url_template(), self.is_authoritative()
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
            r#"DELETE FROM external_source WHERE id = $1"#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

impl Default for ExternalSource {
    fn default() -> Self {
        ExternalSource {
            id: None,
            slug: String::from("anilist"), name: String::from("AniList"),
            base_url: Some(String::from("https://anilist.co")),
            url_template: Some(String::from("https://anilist.co/anime/{id}")),
            is_authoritative: true
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
        let external_source = ExternalSource::default();
        let id = external_source.insert(&mut tx).await?;
        ExternalSource::delete(id, &mut tx).await?;

        Ok(())
    }
}

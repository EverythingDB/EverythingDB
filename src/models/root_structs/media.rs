use ambassador::delegatable_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::query_scalar;

use crate::models::enums::common::DatePrecision::Day;
use crate::models::enums::common::MediaStatus::Completed;
use crate::models::enums::common::SourceMaterial::Original;
use crate::models::traits::{Insertable, Deletable};
use crate::models::enums::common::{DatePrecision, MediaStatus, SourceMaterial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Media{
    id: Option<i32>,
    slug: Option<String>,

    // naming
    primary_title: String,
    original_title: Option<String>,
    romanized_title: Option<String>,
    sort_title: Option<String>,

    // provenance
    original_language_id: Option<i32>,
    country_of_origin_id: Option<i32>,
    source_material: SourceMaterial,

    // lifecycle
    status: MediaStatus,
    started_on: Option<NaiveDate>,
    ended_on: Option<NaiveDate>,
    date_precision: DatePrecision,
    is_indefinite: bool,

    // description
    tagline: Option<String>,
    synopsis: Option<String>,
    synopsis_language_id: Option<i32>,
    notes: Option<String>,

    // classification flags that apply to every form
    is_adult: bool,
    is_official: bool,
    is_lost_media: bool,
    is_unreleased: bool,

    // denormalized aggregates, maintained by the application
    mean_score: Decimal,
    score_count: i32,
    popularity: i32,
    favorite_count: i32,

    // curation
    /// Between 0 and 100
    data_completeness: i16,
    is_locked: bool,
    verified_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

#[allow(dead_code)]
#[delegatable_trait]
pub trait HasMedia{
    fn id(&self) -> Option<&i32>;
    fn slug(&self) -> Option<&str>;

    fn original_language_id(&self) -> Option<&i32>;
    fn country_of_origin_id(&self) -> Option<&i32>;
    fn source_material(&self) -> &SourceMaterial;

    fn primary_title(&self) -> &str;
    fn original_title(&self) -> Option<&str>;
    fn romanized_title(&self) -> Option<&str>;
    fn sort_title(&self) -> Option<&str>;

    fn status(&self) -> &MediaStatus;
    fn started_on(&self) -> Option<&NaiveDate>;
    fn ended_on(&self) -> Option<&NaiveDate>;
    fn date_precision(&self) -> &DatePrecision;
    fn is_indefinite(&self) -> &bool;

    fn tagline(&self) -> Option<&str>;
    fn synopsis(&self) -> Option<&str>;
    fn synopsis_language_id(&self) -> Option<&i32>;
    fn notes(&self) -> Option<&str>;

    fn is_adult(&self) -> &bool;
    fn is_official(&self) -> &bool;
    fn is_lost_media(&self) -> &bool;
    fn is_unreleased(&self) -> &bool;

    fn mean_score(&self) -> &Decimal;
    fn score_count(&self) -> &i32;
    fn popularity(&self) -> &i32;
    fn favorite_count(&self) -> &i32;

    fn data_completeness(&self) -> &i16;
    fn is_locked(&self) -> &bool;
    fn verified_at(&self) -> Option<&DateTime<Utc>>;
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;
}

impl HasMedia for Media {
    fn id(&self) -> Option<&i32> {self.id.as_ref()}
    fn slug(&self) -> Option<&str> {self.slug.as_deref()}

    fn original_language_id(&self) -> Option<&i32> {self.original_language_id.as_ref()}
    fn country_of_origin_id(&self) -> Option<&i32> {self.country_of_origin_id.as_ref()}
    fn source_material(&self) -> &SourceMaterial {&self.source_material}

    fn primary_title(&self) -> &str {&self.primary_title}
    fn original_title(&self) -> Option<&str> {self.original_title.as_deref()}
    fn romanized_title(&self) -> Option<&str> {self.romanized_title.as_deref()}
    fn sort_title(&self) -> Option<&str> {self.sort_title.as_deref()}

    fn status(&self) -> &MediaStatus {&self.status}
    fn started_on(&self) -> Option<&NaiveDate> {self.started_on.as_ref()}
    fn ended_on(&self) -> Option<&NaiveDate> {self.ended_on.as_ref()}
    fn date_precision(&self) -> &DatePrecision {&self.date_precision}
    fn is_indefinite(&self) -> &bool {&self.is_indefinite}

    fn tagline(&self) -> Option<&str> {self.tagline.as_deref()}
    fn synopsis(&self) -> Option<&str> {self.synopsis.as_deref()}
    fn synopsis_language_id(&self) -> Option<&i32> {self.synopsis_language_id.as_ref()}
    fn notes(&self) -> Option<&str> {self.notes.as_deref()}

    fn is_adult(&self) -> &bool {&self.is_adult}
    fn is_official(&self) -> &bool {&self.is_official}
    fn is_lost_media(&self) -> &bool {&self.is_lost_media}
    fn is_unreleased(&self) -> &bool {&self.is_unreleased}

    fn mean_score(&self) -> &Decimal {&self.mean_score}
    fn score_count(&self) -> &i32 {&self.score_count}
    fn popularity(&self) -> &i32 {&self.popularity}
    fn favorite_count(&self) -> &i32 {&self.favorite_count}

    fn data_completeness(&self) -> &i16 {&self.data_completeness}
    fn is_locked(&self) -> &bool {&self.is_locked}
    fn verified_at(&self) -> Option<&DateTime<Utc>> {self.verified_at.as_ref()}
    fn created_at(&self) -> &DateTime<Utc> {&self.created_at}
    fn updated_at(&self) -> &DateTime<Utc> {&self.updated_at}
}

impl Insertable for Media {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<i32, sqlx::Error>
    {
        let id = query_scalar!(
        r#"
            INSERT INTO media (
                slug,
                primary_title, original_title, romanized_title, sort_title,
                original_language_id, country_of_origin_id, source_material,
                status, started_on, ended_on, date_precision, is_indefinite,
                tagline, synopsis, synopsis_language_id, notes,
                is_adult, is_official, is_lost_media, is_unreleased,
                mean_score, score_count, popularity, favorite_count,
                data_completeness, is_locked, verified_at
                )
                VALUES (
                $1,
                $2, $3, $4, $5,
                $6, $7, $8::source_material,
                $9::media_status, $10, $11, $12::date_precision, $13,
                $14, $15, $16, $17,
                $18, $19, $20, $21,
                $22, $23, $24, $25,
                $26, $27, $28
                )
                RETURNING id AS "id!"
            "#,
            self.slug(),
            self.primary_title(), self.original_title(), self.romanized_title(), self.sort_title(),
            self.original_language_id(), self.country_of_origin_id(), self.source_material(),
            self.status(), self.started_on(), self.ended_on(), self.date_precision(), self.is_indefinite(),
            self.tagline(), self.synopsis(), self.synopsis_language_id(), self.notes(),
            self.is_adult(), self.is_official(), self.is_lost_media(), self.is_unreleased(),
            self.mean_score(), self.score_count(), self.popularity(), self.favorite_count(),
            self.data_completeness(), self.is_locked(), self.verified_at()
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }
}

impl Deletable for Media {
    async fn delete(
        id: i32,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error>
    {
        todo!()
    }
}

impl Default for Media {
    /// First season of Tenchi Muyo, nothing special about it, not even my favorite anime, just old enough, good enough, and iconic enough to garner the id of 1 imo
    fn default() -> Self {
        let synopsis: String = String::from("Tenchi Masaki was a normal 17-year-old boy until the day he accidentally releases the space pirate, Ryoko from a cave she was sealed in 700 years ago as the people thought she was a demon. In a series of events, four other alien girls show up at the Masaki household as Tenchi learns much of his heritage he never knew about and deal with five alien girls who each have some sort of romantic interest in him.");
        let notes: String = String::from("First anime in the DB, not because it's special, but just cuz it's old enough, good (imo), and iconic to some degree");
        let verified: Option<DateTime<Utc>> = DateTime::from_timestamp_secs(1787676872);
        let created: Option<DateTime<Utc>> = DateTime::from_timestamp_secs(1787680205);

        Media { 
            id: Some(1), slug: Some(String::from("hi")),
            primary_title: String::from("Tenchi Muyo! Ryo-Ohki"), original_title: Some(String::from("天地無用! 魎皇鬼 第1期")), romanized_title: Some(String::from("Tenchi Muyou! Ryououki Dai iki")), sort_title: Some(String::from("Tenchi Muyo! 1")), original_language_id: None,
            country_of_origin_id: None, source_material: Original, status: Completed,
            started_on: NaiveDate::from_ymd_opt(1992, 9, 25), ended_on: NaiveDate::from_ymd_opt(1993,3,25), date_precision: Day, is_indefinite: false,
            tagline: None, synopsis: Some(synopsis), synopsis_language_id: None, notes: Some(notes),
            is_adult: false, is_official: true, is_lost_media: false, is_unreleased: false,
            mean_score: Decimal::from(73), score_count: 4148, popularity: 12431, favorite_count: 246,
            data_completeness: 40, is_locked: false,
            verified_at: verified, created_at: created.expect("if this somehow panics... idk what you did"), updated_at: created.expect("if this somehow panics... idk what you did")
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
    async fn test_insert() -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = pool().await.begin().await?;
        let tenchi = Media::default();
        tenchi.insert(&mut tx).await?;

        Ok(())
    }
}
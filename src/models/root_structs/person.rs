use crate::models::{enums::{common::DatePrecision, people_orgs::Gender}, traits::RootStruct};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    id:                  Option<i32>,
    slug:                Option<String>,

    // Naming
    primary_name:        String,
    native_name:         Option<String>,
    romanized_name:      Option<String>,
    sort_name:           Option<String>,
    given_name:          Option<String>,
    family_name:         Option<String>,

    // General Data
    gender:              Gender,
    pronouns:            Option<String>,
    birth_date:          Option<NaiveDate>,
    birth_precision:     DatePrecision,
    death_date:          Option<NaiveDate>,
    birth_country_id:    Option<i32>,
    hometown:            Option<String>,
    height_cm:           Option<i16>,
    blood_type:          Option<String>, //todo: replace with an Enum

    // Group info
    is_group:            bool,
    active_from:         Option<NaiveDate>,
    active_until:        Option<NaiveDate>,
    primary_language_id: Option<i32>,

    // Special Data
    biography:           Option<String>,
    website:             Option<String>,
    created_at:          DateTime<Utc>,
    updated_at:          DateTime<Utc>
}

#[allow(dead_code)]
pub trait HasPerson{
    fn person(&self) -> &Person;

    fn id(&self) -> Option<&i32>;
    fn slug(&self) -> Option<&str>;

    // Naming
    fn primary_name(&self) -> &str;
    fn native_name(&self) -> Option<&str>;
    fn romanized_name(&self) -> Option<&str>;
    fn sort_name(&self) -> Option<&str>;
    fn given_name(&self) -> Option<&str>;
    fn family_name(&self) ->  Option<&str>;

    // General Data
    fn gender(&self) -> &Gender;
    fn pronouns(&self) -> Option<&str>;
    fn birth_date(&self) -> Option<&NaiveDate>;
    fn birth_precision(&self) -> &DatePrecision;
    fn death_date(&self) -> Option<&NaiveDate>;
    fn birth_country_id(&self) -> Option<&i32>;
    fn hometown(&self) -> Option<&str>;
    fn height_cm(&self) -> Option<&i16>;
    fn blood_type(&self) -> Option<&str>;

    // Group info
    fn is_group(&self) -> &bool;
    fn active_from(&self) -> Option<&NaiveDate>;
    fn active_until(&self) -> Option<&NaiveDate>;
    fn primary_language_id(&self) -> Option<&i32>;

    // Special Data
    fn biography(&self) -> Option<&str>;
    fn website(&self) -> Option<&str>;
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;
}

impl HasPerson for Person {
    fn person(&self) -> &Person { self }

    fn id(&self) -> Option< &i32>  { self.id.as_ref() }
    fn slug(&self) -> Option< &str>  { self.slug.as_deref() }

    // Naming
    fn primary_name(&self) ->  &str { self.primary_name.as_ref() }
    fn native_name(&self) -> Option< &str>  { self.native_name.as_deref() }
    fn romanized_name(&self) -> Option< &str>  { self.romanized_name.as_deref() }
    fn sort_name(&self) -> Option< &str>  { self.sort_name.as_deref() }
    fn given_name(&self) -> Option< &str>  { self.given_name.as_deref() }
    fn family_name(&self) -> Option< &str>  { self.family_name.as_deref() }

    // General Data
    fn gender(&self) ->  &Gender { &self.gender }
    fn pronouns(&self) -> Option<&str>  { self.pronouns.as_deref() }
    fn birth_date(&self) -> Option<&NaiveDate>  { self.birth_date.as_ref() }
    fn birth_precision(&self) ->  &DatePrecision { &self.birth_precision }
    fn death_date(&self) -> Option< &NaiveDate>  { self.death_date.as_ref() }
    fn birth_country_id(&self) -> Option<&i32>  { self.birth_country_id.as_ref() }
    fn hometown(&self) -> Option< &str>  { self.hometown.as_deref() }
    fn height_cm(&self) -> Option< &i16>  { self.height_cm.as_ref() }
    fn blood_type(&self) -> Option< &str>  { self.blood_type.as_deref() }

    // Group info
    fn is_group(&self) ->  &bool { &self.is_group }
    fn active_from(&self) -> Option< &NaiveDate>  { self.active_from.as_ref() }
    fn active_until(&self) -> Option< &NaiveDate>  { self.active_until.as_ref() }
    fn primary_language_id(&self) -> Option< &i32>  { self.primary_language_id.as_ref() }

    // Special Data
    fn biography(&self) -> Option< &str>  { self.biography.as_deref() }
    fn website(&self) -> Option< &str>  { self.website.as_deref() }
    fn created_at(&self) ->  &DateTime<Utc>  { &self.created_at }
    fn updated_at(&self) ->  &DateTime<Utc>  { &self.updated_at }
}

impl RootStruct for Person {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query_scalar!(
        r#"
            INSERT INTO person (
                slug,
                primary_name, native_name, romanized_name, sort_name, given_name, family_name,
                gender, pronouns, birth_date, birth_precision, death_date, birth_country_id, hometown, height_cm, blood_type,
                is_group, active_from, active_until, primary_language_id,
                biography, website
                )
                VALUES (
                $1,
                $2, $3, $4, $5, $6, $7,
                $8::gender, $9, $10, $11::date_precision, $12, $13, $14, $15, $16,
                $17, $18, $19, $20,
                $21, $22
                )
                RETURNING id AS "id!"
            "#,
            self.slug(),
            self.primary_name(), self.native_name(), self.romanized_name(), self.sort_name(), self.given_name(), self.family_name(),
            self.gender() as &Gender, self.pronouns(), self.birth_date(), self.birth_precision() as &DatePrecision, self.death_date(), self.birth_country_id(), self.hometown(), self.height_cm(), self.blood_type(),
            self.is_group(), self.active_from(), self.active_until(), self.primary_language_id(),
            self.biography(), self.website()
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
            r#"DELETE FROM person WHERE id = $1"#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

impl Default for Person {
    fn default() -> Self {
        let created: DateTime<Utc> = DateTime::from_timestamp_secs(1787680205)
            .expect("if this somehow panics... idk what you did");

        Person {
            id: None, slug: Some(String::from("test-person")),
            primary_name: String::from("Test Person"), native_name: None, romanized_name: None, sort_name: None, given_name: None, family_name: None,
            gender: Gender::Unspecified, pronouns: None, birth_date: None, birth_precision: DatePrecision::Day, death_date: None, birth_country_id: None, hometown: None, height_cm: None, blood_type: None,
            is_group: false, active_from: None, active_until: None, primary_language_id: None,
            biography: None, website: None, created_at: created, updated_at: created
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
        let person = Person::default();
        let id = person.insert(&mut tx).await?;
        Person::delete(id, &mut tx).await?;

        Ok(())
    }
}

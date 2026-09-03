use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::models::traits::RootStruct;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    id:           Option<i32>,
    slug:         String,
    name:         String,
    namespace:    String, // genre | theme | setting | content_warning | demographic | technique | origin | franchise_trait | publishing_trait | mood | misc
    description:  Option<String>,
    parent_id:    Option<i32>,
    is_adult:     bool,
    is_spoiler:   bool,
    is_moderated: bool,
    usage_count:  i32,
    created_at:   DateTime<Utc>
}

#[allow(dead_code)]
pub trait HasTag{
    fn tag(&self) -> &Tag;

    fn id(&self) -> Option<&i32>;
    fn slug(&self) -> &str;
    fn name(&self) -> &str;
    fn namespace(&self) -> &str;
    fn description(&self) -> Option<&str>;
    fn parent_id(&self) -> Option<&i32>;
    fn is_adult(&self) -> &bool;
    fn is_spoiler(&self) -> &bool;
    fn is_moderated(&self) -> &bool;
    fn usage_count(&self) -> &i32;
    fn created_at(&self) -> &DateTime<Utc>;
}

impl HasTag for Tag {
    fn tag(&self) -> &Tag { self }

    fn id(&self) -> Option<&i32> { self.id.as_ref() }
    fn slug(&self) -> &str { &self.slug }
    fn name(&self) -> &str { &self.name }
    fn namespace(&self) -> &str { &self.namespace }
    fn description(&self) -> Option<&str> { self.description.as_deref() }
    fn parent_id(&self) -> Option<&i32> { self.parent_id.as_ref() }
    fn is_adult(&self) -> &bool { &self.is_adult }
    fn is_spoiler(&self) -> &bool { &self.is_spoiler }
    fn is_moderated(&self) -> &bool { &self.is_moderated }
    fn usage_count(&self) -> &i32 { &self.usage_count }
    fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
}

impl RootStruct for Tag {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query_scalar!(
        r#"
            INSERT INTO tag (
                slug, name, namespace, description, parent_id,
                is_adult, is_spoiler, is_moderated, usage_count
                )
                VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9
                )
                RETURNING id AS "id!"
            "#,
            self.slug(), self.name(), self.namespace(), self.description(), self.parent_id(),
            self.is_adult(), self.is_spoiler(), self.is_moderated(), self.usage_count()
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
            r#"DELETE FROM tag WHERE id = $1"#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

impl Default for Tag {
    fn default() -> Self {
        let created: DateTime<Utc> = DateTime::from_timestamp_secs(1787680205)
            .expect("if this somehow panics... idk what you did");

        Tag {
            id: None,
            slug: String::from("isekai"), name: String::from("Isekai"),
            namespace: String::from("setting"), description: Some(String::from("Protagonist transported to another world")), parent_id: None,
            is_adult: false, is_spoiler: false, is_moderated: true, usage_count: 0, created_at: created
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
        let tag = Tag::default();
        let id = tag.insert(&mut tx).await?;
        Tag::delete(id, &mut tx).await?;

        Ok(())
    }
}

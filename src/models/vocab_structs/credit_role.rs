use serde::{Serialize, Deserialize};

use crate::models::{enums::common::CreditDepartment, traits::RootStruct};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreditRole {
    id:                Option<i32>,
    slug:              String,
    name:              String,
    department:        CreditDepartment,
    description:       Option<String>,
    is_primary_credit: bool
}

#[allow(dead_code)]
pub trait HasCreditRole{
    fn credit_role(&self) -> &CreditRole;

    fn id(&self) -> Option<&i32>;
    fn slug(&self) -> &str;
    fn name(&self) -> &str;
    fn department(&self) -> &CreditDepartment;
    fn description(&self) -> Option<&str>;
    fn is_primary_credit(&self) -> &bool;
}

impl HasCreditRole for CreditRole {
    fn credit_role(&self) -> &CreditRole { self }

    fn id(&self) -> Option<&i32> { self.id.as_ref() }
    fn slug(&self) -> &str { &self.slug }
    fn name(&self) -> &str { &self.name }
    fn department(&self) -> &CreditDepartment { &self.department }
    fn description(&self) -> Option<&str> { self.description.as_deref() }
    fn is_primary_credit(&self) -> &bool { &self.is_primary_credit }
}

impl RootStruct for CreditRole {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query_scalar!(
        r#"
            INSERT INTO credit_role (
                slug, name, department, description, is_primary_credit
                )
                VALUES (
                $1, $2, $3::credit_department, $4, $5
                )
                RETURNING id AS "id!"
            "#,
            self.slug(), self.name(), self.department() as &CreditDepartment, self.description(), self.is_primary_credit()
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
            r#"DELETE FROM credit_role WHERE id = $1"#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

impl Default for CreditRole {
    fn default() -> Self {
        CreditRole {
            id: None,
            slug: String::from("director"), name: String::from("Director"),
            department: CreditDepartment::Direction, description: Some(String::from("Oversees the overall creative direction")),
            is_primary_credit: true
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
        let credit_role = CreditRole::default();
        let id = credit_role.insert(&mut tx).await?;
        CreditRole::delete(id, &mut tx).await?;

        Ok(())
    }
}

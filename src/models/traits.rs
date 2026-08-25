use sqlx::{Postgres, Transaction};

// Action traits
pub trait Insertable {
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<i32, sqlx::Error>;
}

pub trait Deletable {
    async fn delete(
        id: i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error>;
}

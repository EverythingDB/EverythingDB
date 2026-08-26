use sqlx::{Postgres, Transaction};

// Root Structs action traits
pub trait RootStruct {
    /// Inserts a root scuct into the DB and returns its assigned ID
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<i32, sqlx::Error>;

    /// Delete a root (and cascade to further FK rows) by ID 
    async fn delete(
        id: i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error>;
}

// Non-root structs action traits
pub trait NonRootStruct {
    /// Insert a non-root struct into the DB.
    /// Must have an ID already present in &self
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error>;
}
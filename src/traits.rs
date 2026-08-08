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

// Has* traits
pub trait HasID {
    fn id(&self) -> Option<i32>;
}
pub trait HasTitle {
    fn title(&self) -> &str;
}
pub trait HasSynopsis {
    fn synopsis(&self) -> &str;
}
pub trait HasISBN {
    fn isbn(&self) -> &str;
}
pub trait HasAuthor {
    fn author(&self) -> &str;
}
pub trait HasPageCount {
    fn page_count(&self) -> i32;
}

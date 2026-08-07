use sqlx::{PgPool, Row};


pub trait PublicTable {
    async fn insert(&self, pool: &PgPool) -> Result<u32, sqlx::Error>;
    async fn delete(&self, pool: &PgPool);
}

pub trait HasTitle {
    fn title(&self) -> &str;
}
pub trait HasID {
    fn id(&self) -> Option<u32>;
}
pub trait HasISBN {
    fn isbn(&self) -> &str;
}
pub trait HasAuthor {
    fn author(&self) -> &str;
}
pub trait HasPageCount {
    fn page_count(&self) -> u32;
}

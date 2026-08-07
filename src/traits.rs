pub trait Insertable {
    async fn insert<'e, E>(&self, executor: E) -> Result<i32, sqlx::Error>
    where E: sqlx::Executor<'e, Database = sqlx::Postgres>;
}
pub trait Deletable {
    async fn delete<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where E: sqlx::Executor<'e, Database = sqlx::Postgres>;
}

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

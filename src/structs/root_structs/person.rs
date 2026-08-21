use sqlx::{query, query_scalar, Postgres, Transaction};

use crate::traits::{Deletable, Insertable};

pub struct Person{
    id: Option<i32>,
    
}
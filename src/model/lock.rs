use sqlx::FromRow;
use chrono::{NaiveDateTime};

#[derive(Debug)]
pub struct LockAdd {
    pub hash: String
}

#[derive(Debug, FromRow)]
pub struct Lock {
    pub id: i64,
    pub hash: String,
    pub timestamp: NaiveDateTime,
}

impl Lock {
    pub fn new(id: i64, hash: String, timestamp: NaiveDateTime) -> Self {
        Lock { id, hash, timestamp }
    }
}
use chrono::{Timelike, Utc};

pub fn get_timestamp() -> chrono::DateTime<chrono::Utc> {
    Utc::now().with_nanosecond((Utc::now().nanosecond() / 1_000_000) * 1_000_000).unwrap()
}
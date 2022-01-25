use chrono::{NaiveDateTime, Utc};

pub struct TimeUtils {}

impl TimeUtils {
    pub fn get_current_utc_time() -> NaiveDateTime {
        Utc::now().naive_utc()
    }
}

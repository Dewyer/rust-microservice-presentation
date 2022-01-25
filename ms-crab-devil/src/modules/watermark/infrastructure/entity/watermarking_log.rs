use crate::schema::watermarking_log;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable, AsChangeset)]
#[table_name = "watermarking_log"]
#[changeset_options(treat_none_as_null = "true")]
pub struct WatermarkingLog {
    pub id: Uuid,
    pub full_name: String,
    pub file_bucket: String,
    pub file_key: String,
    pub watermarked_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "watermarking_log"]
pub struct NewWatermarkingLog<'a> {
    pub full_name: &'a str,
    pub file_bucket: &'a str,
    pub file_key: &'a str,
    pub watermarked_at: NaiveDateTime,
}

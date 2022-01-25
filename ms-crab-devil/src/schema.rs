table! {
    watermarking_log (id) {
        id -> Uuid,
        full_name -> Varchar,
        file_bucket -> Varchar,
        file_key -> Varchar,
        watermarked_at -> Timestamp,
    }
}

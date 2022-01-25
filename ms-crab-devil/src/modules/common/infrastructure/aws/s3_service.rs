use crate::modules::common::errors::integration::S3Error;
use rusoto_core::Region;
use rusoto_s3::{
    GetObjectOutput, GetObjectRequest, PutObjectOutput, PutObjectRequest, S3Client, StreamingBody,
    S3,
};

pub struct S3Service {}

impl S3Service {
    fn get_client() -> S3Client {
        S3Client::new(Region::EuCentral1)
    }

    pub async fn get_object(bucket: &str, key: &str) -> Result<GetObjectOutput, S3Error> {
        Self::get_client()
            .get_object(GetObjectRequest {
                bucket: bucket.to_string(),
                key: key.to_string(),
                ..Default::default()
            })
            .await
            .map_err(|_| S3Error::ObjectNotFound(bucket.to_string(), key.to_string()))
    }

    pub async fn put_object(
        bucket: &str,
        key: &str,
        body: StreamingBody,
    ) -> Result<PutObjectOutput, S3Error> {
        Self::get_client()
            .put_object(PutObjectRequest {
                bucket: bucket.to_string(),
                key: key.to_string(),
                body: Some(body),
                ..Default::default()
            })
            .await
            .map_err(|_| S3Error::FailedToPutObject(bucket.to_string(), key.to_string()))
    }
}

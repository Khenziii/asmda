pub mod utils;

use crate::api_wrappers::APIWrapper;
use crate::environment;
use crate::logger::logger;
use crate::utils::constants::APIWrapperIdentificator;
use crate::utils::exit::exit;
use s3::{Bucket, Region, creds::Credentials};

pub struct S3Client {
    bucket: Bucket,
}

impl APIWrapper for S3Client {
    fn get_identificator(&self) -> APIWrapperIdentificator {
        APIWrapperIdentificator::S3
    }
}

async fn get_main_bucket() -> Bucket {
    let config = environment::environment();

    let region = Region::Custom {
        region: config.s3.region.clone(),
        endpoint: config.s3.url.clone(),
    };
    let credentials = Credentials::new(
        Some(&config.s3.access_key),
        Some(&config.s3.secret_key),
        None,
        None,
        None,
    )
    .unwrap();

    let bucket = Bucket::new(&config.s3.bucket_name, region.clone(), credentials.clone()).unwrap();

    if let Err(error) = bucket.list("".to_string(), None).await {
        let error_message = format!(
            "Can't access storage bucket! No data at all will be stored. Please correct stored credentials. Details: {}",
            error
        );
        logger().error(&error_message);
        exit();
    }

    *bucket
}

impl S3Client {
    pub async fn new() -> Self {
        let bucket = get_main_bucket().await;
        S3Client { bucket }
    }

    pub async fn upload(&self, app_name: &str, filename: &str, data: Vec<u8>) {
        let object_path = format!("{}/{}", app_name, filename);
        self.bucket
            .put_object(object_path, &data)
            .await
            .expect("Failed to upload the file to S3!");
    }
}

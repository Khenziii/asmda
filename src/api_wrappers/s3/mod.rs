use crate::api_wrappers::APIWrapper;
use crate::environment;
use crate::utils::constants::APIWrapperIdentificator;
use s3::{Bucket, BucketConfiguration, Region, creds::Credentials};

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
    let bucket_config = BucketConfiguration::default();

    let bucket = Bucket::new(&config.s3.bucket_name, region.clone(), credentials.clone()).unwrap();

    let bucket_exists = bucket
        .exists()
        .await
        .expect("Failed to check whether a bucket exists! Are the S3 credentials surely correct?");
    if !bucket_exists {
        Bucket::create_with_path_style(
            &config.s3.bucket_name,
            region.clone(),
            credentials.clone(),
            bucket_config.clone(),
        )
        .await
        .unwrap_or_else(|_| panic!("Failed to create the `{}` bucket!", &config.s3.bucket_name,));
    }

    *bucket
}

impl S3Client {
    pub async fn new() -> Self {
        let bucket = get_main_bucket().await;
        S3Client { bucket }
    }

    pub async fn upload(&self, app_name: &str, filename: &str, data: Vec<u8>) {
        let object_path = format!("{}/{}/{}", self.bucket.name, app_name, filename);
        self.bucket
            .put_object(object_path, &data)
            .await
            .expect("Failed to upload the file to S3!");
    }
}

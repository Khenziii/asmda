use s3::{Region, Bucket, BucketConfiguration, creds::Credentials};
use crate::api_wrappers::APIWrapper;
use crate::environment;

pub struct S3Client {
    bucket: Bucket,
}

impl APIWrapper for S3Client {
    fn get_name(&self) -> &str {
        "s3"
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
    ).unwrap();
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
        .expect(&format!("Failed to create the `{}` bucket!", &config.s3.bucket_name));
    }

    *bucket
}

impl S3Client {
    pub async fn new() -> Self {
        let bucket = get_main_bucket().await;
        let this = S3Client { bucket };

        this
    }

    pub async fn upload(&self, data: Vec<u8>) {
        // TODO: when trying to upload a file (Vec<u8>), request app name. Once you have it, check
        // if the directory `<app_name>` exists in the bucket. If it does, create a folder in that
        // directory with the name of current date. Upload the binary data there.
    }
}

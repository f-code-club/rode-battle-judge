use std::env;

use aws_sdk_s3::Client;

const BUCKET_ENV: &str = "s3_bucket";

pub struct Storage {
    pub bucket: String,
    pub client: Client,
}

impl Storage {
    pub async fn new() -> color_eyre::Result<Self> {
        let bucket = env::var(BUCKET_ENV)?;
        let client = Client::new(&aws_config::load_from_env().await);

        Ok(Storage { bucket, client })
    }

    pub async fn download(&self, path: &str) -> color_eyre::Result<Vec<u8>> {
        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(path)
            .send()
            .await?;
        let data = response.body.collect().await?.into_bytes().to_vec();
        Ok(data)
    }
}

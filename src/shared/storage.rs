use std::{env, sync::Arc};

use aws_sdk_s3::Client;
use moka::future::Cache;

const BUCKET_ENV: &str = "s3_bucket";
const CACHE_SIZE: u64 = 1024 * 1024 * 1024; // 1GB

pub struct Storage {
    pub bucket: String,
    pub client: Client,
    pub cache: Cache<String, Arc<Vec<u8>>>,
}

impl Storage {
    pub async fn new() -> color_eyre::Result<Self> {
        let bucket = env::var(BUCKET_ENV)?;
        let client = Client::new(&aws_config::load_from_env().await);
        let cache = Cache::builder()
            .weigher(|_, value: &Arc<Vec<u8>>| -> u32 {
                value.len().try_into().unwrap_or(u32::MAX)
            })
            .max_capacity(CACHE_SIZE)
            .build();

        Ok(Storage {
            bucket,
            client,
            cache,
        })
    }

    async fn _download(&self, path: &str) -> color_eyre::Result<Vec<u8>> {
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

    pub async fn download(&self, path: String) -> color_eyre::Result<Arc<Vec<u8>>> {
        if !self.cache.contains_key(&path) {}

        match self.cache.get(&path).await {
            Some(data) => Ok(data),
            None => {
                let data = Arc::new(self._download(&path).await?);
                self.cache.insert(path.clone(), data.clone()).await;

                Ok(data)
            }
        }
    }
}

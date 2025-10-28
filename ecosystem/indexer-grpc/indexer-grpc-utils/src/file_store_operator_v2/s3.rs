// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::file_store_operator_v2::common::{IFileStoreReader, IFileStoreWriter};
use anyhow::{bail, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use std::path::PathBuf;
use tokio::time::Duration;
use tracing::{info, trace};

pub struct S3FileStore {
    client: Client,
    bucket_name: String,
    bucket_sub_dir: Option<PathBuf>,
}

impl S3FileStore {
    pub async fn new(
        bucket_name: String,
        bucket_sub_dir: Option<PathBuf>,
        region: String,
        endpoint: Option<String>,
        access_key: String,
        secret_key: String,
    ) -> Self {
        info!(
            bucket_name = bucket_name,
            region = region,
            endpoint = endpoint,
            "Initializing S3FileStore."
        );

        let credentials = Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "s3-filestore",
        );

        let region_provider = RegionProviderChain::first_try(Region::new(region));

        let mut config_builder = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(region_provider)
            .credentials_provider(credentials);

        // Configure custom endpoint if provided (for Digital Ocean Spaces, MinIO, etc.)
        if let Some(endpoint_url) = endpoint {
            config_builder = config_builder.endpoint_url(endpoint_url);
        }

        let config = config_builder.load().await;
        let client = Client::new(&config);

        // Verify bucket exists
        match client.head_bucket().bucket(&bucket_name).send().await {
            Ok(_) => info!(
                bucket_name = bucket_name,
                "Bucket exists, S3FileStore is created."
            ),
            Err(e) => panic!(
                "Failed to access bucket {}: {:?}. Please verify bucket name, credentials, and permissions.",
                bucket_name, e
            ),
        }

        Self {
            client,
            bucket_name,
            bucket_sub_dir,
        }
    }

    fn get_path(&self, file_path: PathBuf) -> String {
        if let Some(sub_dir) = &self.bucket_sub_dir {
            let mut path = sub_dir.clone();
            path.push(file_path);
            path.to_string_lossy().into_owned()
        } else {
            file_path.to_string_lossy().into_owned()
        }
    }
}

#[async_trait::async_trait]
impl IFileStoreReader for S3FileStore {
    fn tag(&self) -> &str {
        "S3"
    }

    async fn is_initialized(&self) -> bool {
        let prefix = self
            .bucket_sub_dir
            .clone()
            .map(|p| p.to_string_lossy().into_owned());

        let mut request = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket_name)
            .max_keys(1);

        if let Some(prefix) = prefix {
            request = request.prefix(prefix);
        }

        match request.send().await {
            Ok(response) => {
                !response.contents().is_empty() || !response.common_prefixes().is_empty()
            },
            Err(e) => {
                panic!(
                    "Failed to list bucket. Bucket name: {}, sub_dir: {:?}, error: {:?}",
                    self.bucket_name, self.bucket_sub_dir, e
                );
            },
        }
    }

    async fn get_raw_file(&self, file_path: PathBuf) -> Result<Option<Vec<u8>>> {
        let path = self.get_path(file_path);
        trace!(
            "Downloading object from s3://{}/{}",
            self.bucket_name,
            path
        );

        match self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(&path)
            .send()
            .await
        {
            Ok(response) => {
                let data = response.body.collect().await?.to_vec();
                Ok(Some(data))
            },
            Err(err) => {
                let error_str = format!("{:?}", err);
                if error_str.contains("NoSuchKey") || error_str.contains("NotFound") {
                    Ok(None)
                } else {
                    bail!(
                        "[Indexer File] Error happens when downloading file at {}: {}",
                        path,
                        err
                    );
                }
            },
        }
    }
}

#[async_trait::async_trait]
impl IFileStoreWriter for S3FileStore {
    async fn save_raw_file(&self, file_path: PathBuf, data: Vec<u8>) -> Result<()> {
        let path = self.get_path(file_path);
        trace!(
            "Uploading object to s3://{}/{}",
            self.bucket_name,
            path
        );

        let body = ByteStream::from(data);

        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&path)
            .content_type("application/json")
            .body(body)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to upload file to S3: {}", e))?;

        Ok(())
    }

    fn max_update_frequency(&self) -> Duration {
        // S3 can handle high request rates
        Duration::from_millis(100)
    }
}

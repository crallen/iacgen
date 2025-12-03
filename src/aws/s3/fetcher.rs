use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::{
    aws::s3::{
        BucketEncryption, BucketLogging, BucketVersioning,
        model::{Bucket, BucketBuilder, BucketPublicAccessBlock},
    },
    core::ResourceFetcher,
};

#[derive(Clone)]
pub struct S3Fetcher {
    client: aws_sdk_s3::Client,
}

impl S3Fetcher {
    pub fn new(config: aws_config::SdkConfig) -> Self {
        Self {
            client: aws_sdk_s3::Client::new(&config),
        }
    }

    async fn enrich_bucket(&self, bucket_name: String) -> Bucket {
        let mut builder = BucketBuilder::new(bucket_name.clone());

        let (policy, public_access_block, encryption, versioning, logging) = tokio::join!(
            self.fetch_policy(&bucket_name),
            self.fetch_public_access_block(&bucket_name),
            self.fetch_encryption(&bucket_name),
            self.fetch_versioning(&bucket_name),
            self.fetch_logging(&bucket_name),
        );

        if let Some(policy) = policy {
            builder = builder.with_policy(policy);
        }

        if let Some(public_access_block) = public_access_block {
            builder = builder.with_public_access_block(public_access_block);
        }

        if let Some(encryption) = encryption {
            builder = builder.with_encryption(encryption);
        }

        if let Some(versioning) = versioning {
            builder = builder.with_versioning(versioning);
        }

        if let Some(logging) = logging {
            builder = builder.with_logging(logging);
        }

        builder.build()
    }

    async fn fetch_policy(&self, bucket_name: &str) -> Option<String> {
        self.client
            .get_bucket_policy()
            .bucket(bucket_name)
            .send()
            .await
            .ok()
            .and_then(|output| output.policy().map(|p| p.to_string()))
    }

    async fn fetch_public_access_block(
        &self,
        bucket_name: &str,
    ) -> Option<BucketPublicAccessBlock> {
        self.client
            .get_public_access_block()
            .bucket(bucket_name)
            .send()
            .await
            .ok()
            .and_then(|output| {
                output
                    .public_access_block_configuration()
                    .map(|cfg| BucketPublicAccessBlock {
                        block_public_acls: cfg.block_public_acls().unwrap_or(true),
                        block_public_policy: cfg.block_public_policy().unwrap_or(true),
                        ignore_public_acls: cfg.ignore_public_acls().unwrap_or(true),
                        restrict_public_buckets: cfg.restrict_public_buckets().unwrap_or(true),
                    })
            })
    }

    async fn fetch_encryption(&self, bucket_name: &str) -> Option<BucketEncryption> {
        self.client
            .get_bucket_encryption()
            .bucket(bucket_name)
            .send()
            .await
            .ok()
            .and_then(|output| {
                output
                    .server_side_encryption_configuration()
                    .and_then(|cfg| BucketEncryption::from_aws_rules(cfg.rules()))
            })
    }

    async fn fetch_versioning(&self, bucket_name: &str) -> Option<BucketVersioning> {
        self.client
            .get_bucket_versioning()
            .bucket(bucket_name)
            .send()
            .await
            .ok()
            .and_then(|output| {
                output.status().map(|status| BucketVersioning {
                    status: status.to_string(),
                })
            })
    }

    async fn fetch_logging(&self, bucket_name: &str) -> Option<BucketLogging> {
        self.client
            .get_bucket_logging()
            .bucket(bucket_name)
            .send()
            .await
            .ok()
            .and_then(|output| {
                output.logging_enabled().map(|enabled| BucketLogging {
                    target_bucket: enabled.target_bucket().to_string(),
                    target_prefix: enabled.target_prefix().to_string(),
                })
            })
    }
}

#[async_trait]
impl ResourceFetcher for S3Fetcher {
    type Resource = Bucket;

    async fn fetch(&self) -> Result<Vec<Bucket>> {
        let buckets = self.client.list_buckets().send().await?;
        let bucket_names: Vec<String> = buckets
            .buckets()
            .iter()
            .filter_map(|b| b.name().map(|n| n.to_string()))
            .collect();

        let semaphore = Arc::new(Semaphore::new(5));
        let mut tasks = Vec::new();

        for bucket_name in bucket_names {
            let semaphore = Arc::clone(&semaphore);
            let fetcher = self.clone();

            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                fetcher.enrich_bucket(bucket_name).await
            });

            tasks.push(task);
        }

        let mut results = Vec::new();
        for task in tasks {
            let result = task.await?;
            results.push(result);
        }

        Ok(results)
    }
}

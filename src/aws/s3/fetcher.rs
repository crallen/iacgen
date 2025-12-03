use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::{
    aws::s3::model::{Bucket, BucketBuilder, BucketPublicAccessBlock},
    core::ResourceFetcher,
};

pub struct S3Fetcher {
    client: aws_sdk_s3::Client,
}

impl S3Fetcher {
    pub fn new(config: aws_config::SdkConfig) -> Self {
        Self {
            client: aws_sdk_s3::Client::new(&config),
        }
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
            let bucket_name = bucket_name.clone();
            let semaphore = Arc::clone(&semaphore);
            let client = self.client.clone();

            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();

                let mut builder = BucketBuilder::new(bucket_name.clone());

                let (policy_result, public_access_block_result) = tokio::join!(
                    client.get_bucket_policy().bucket(&bucket_name).send(),
                    client.get_public_access_block().bucket(&bucket_name).send(),
                );

                if let Ok(policy_output) = policy_result {
                    if let Some(policy) = policy_output.policy() {
                        builder = builder.with_policy(policy.to_string());
                    }
                }

                if let Ok(public_access_block_output) = public_access_block_result {
                    if let Some(cfg) =
                        public_access_block_output.public_access_block_configuration()
                    {
                        builder = builder.with_public_access_block(BucketPublicAccessBlock {
                            block_public_acls: cfg.block_public_acls().unwrap_or(true),
                            block_public_policy: cfg.block_public_policy().unwrap_or(true),
                            ignore_public_acls: cfg.ignore_public_acls().unwrap_or(true),
                            restrict_public_buckets: cfg.restrict_public_buckets().unwrap_or(true),
                        });
                    }
                }

                builder.build()
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

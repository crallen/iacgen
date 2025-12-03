use hcl::{
    Block, Body,
    expr::{Traversal, Variable},
};

use crate::{core::TerraformGenerator, terraform::normalize_resource_name};

pub struct Bucket {
    name: String,
    policy: Option<String>,
    public_access_block: Option<BucketPublicAccessBlock>,
    encryption: Option<BucketEncryption>,
    versioning: Option<BucketVersioning>,
    logging: Option<BucketLogging>,
}

impl From<crate::aws::s3::Bucket> for Bucket {
    fn from(value: crate::aws::s3::Bucket) -> Self {
        Self {
            name: value.name,
            policy: value.policy,
            public_access_block: value
                .public_access_block
                .map(|pab| BucketPublicAccessBlock::from(pab)),
            encryption: value.encryption.map(|e| BucketEncryption::from(e)),
            versioning: value.versioning.map(|v| BucketVersioning::from(v)),
            logging: value.logging.map(|l| BucketLogging::from(l)),
        }
    }
}

impl TerraformGenerator for Bucket {
    fn to_hcl(&self) -> String {
        let resource_name = normalize_resource_name(&self.name);

        let mut body = Body::builder().add_block(
            Block::builder("resource")
                .add_label("aws_s3_bucket")
                .add_label(resource_name.clone())
                .add_attribute(("bucket", self.name.clone()))
                .build(),
        );

        let bucket_traversal = Traversal::builder(Variable::new("aws_s3_bucket").unwrap())
            .attr(resource_name.clone())
            .attr("bucket")
            .build();

        if let Some(public_access_block) = &self.public_access_block {
            let pab_block = Block::builder("resource")
                .add_label("aws_s3_bucket_public_access_block")
                .add_label(resource_name.clone())
                .add_attribute(("bucket", bucket_traversal.clone()))
                .add_attribute(("block_public_acls", public_access_block.block_public_acls))
                .add_attribute((
                    "block_public_policy",
                    public_access_block.block_public_policy,
                ))
                .add_attribute(("ignore_public_acls", public_access_block.ignore_public_acls))
                .add_attribute((
                    "restrict_public_buckets",
                    public_access_block.restrict_public_buckets,
                ))
                .build();

            body = body.add_block(pab_block);
        }

        if let Some(encryption) = &self.encryption {
            let mut sse_by_default_block_builder =
                Block::builder("apply_server_side_encryption_by_default")
                    .add_attribute(("sse_algorithm", encryption.sse_algorithm.clone()));

            if let Some(kms_master_key_id) = &encryption.kms_master_key_id {
                sse_by_default_block_builder = sse_by_default_block_builder
                    .add_attribute(("kms_master_key_id", kms_master_key_id.to_string()));
            }

            let mut rule_block_builder =
                Block::builder("rule").add_block(sse_by_default_block_builder.build());

            if encryption.bucket_key_enabled {
                rule_block_builder = rule_block_builder.add_attribute(("bucket_key_enabled", true));
            }

            let encryption_block = Block::builder("resource")
                .add_label("aws_s3_bucket_server_side_encryption_configuration")
                .add_label(resource_name.clone())
                .add_attribute(("bucket", bucket_traversal.clone()))
                .add_block(rule_block_builder.build())
                .build();

            body = body.add_block(encryption_block);
        }

        if let Some(versioning) = &self.versioning {
            let versioning_block = Block::builder("resource")
                .add_label("aws_s3_bucket_versioning")
                .add_label(resource_name.clone())
                .add_attribute(("bucket", bucket_traversal.clone()))
                .add_block(
                    Block::builder("versioning_configuration")
                        .add_attribute(("status", versioning.status.clone()))
                        .build(),
                )
                .build();

            body = body.add_block(versioning_block);
        }

        if let Some(logging) = &self.logging {
            let logging_block = Block::builder("resource")
                .add_label("aws_s3_bucket_logging")
                .add_label(resource_name.clone())
                .add_attribute(("bucket", bucket_traversal.clone()))
                .add_attribute(("target_bucket", logging.target_bucket.clone()))
                .add_attribute(("target_prefix", logging.target_prefix.clone()))
                .build();

            body = body.add_block(logging_block);
        }

        let mut output = hcl::format::to_string(&body.build()).unwrap();

        if let Some(policy) = &self.policy {
            let policy_json: serde_json::Value = serde_json::from_str(policy).unwrap();
            let formatted_json = serde_json::to_string_pretty(&policy_json).unwrap();

            let policy_block = format!(
                r#"
resource "aws_s3_bucket_policy" "{}" {{
  bucket = aws_s3_bucket.{}.bucket
  policy = <<POLICY
{}
POLICY
}}
"#,
                resource_name, resource_name, formatted_json
            );

            output.push_str(&policy_block);
        }

        output
    }
}

pub struct BucketPublicAccessBlock {
    block_public_acls: bool,
    block_public_policy: bool,
    ignore_public_acls: bool,
    restrict_public_buckets: bool,
}

impl From<crate::aws::s3::BucketPublicAccessBlock> for BucketPublicAccessBlock {
    fn from(value: crate::aws::s3::BucketPublicAccessBlock) -> Self {
        Self {
            block_public_acls: value.block_public_acls,
            block_public_policy: value.block_public_policy,
            ignore_public_acls: value.ignore_public_acls,
            restrict_public_buckets: value.restrict_public_buckets,
        }
    }
}

pub struct BucketEncryption {
    sse_algorithm: String,
    kms_master_key_id: Option<String>,
    bucket_key_enabled: bool,
}

impl From<crate::aws::s3::BucketEncryption> for BucketEncryption {
    fn from(value: crate::aws::s3::BucketEncryption) -> Self {
        Self {
            sse_algorithm: value.sse_algorithm,
            kms_master_key_id: value.kms_master_key_id,
            bucket_key_enabled: value.bucket_key_enabled,
        }
    }
}

pub struct BucketVersioning {
    status: String,
}

impl From<crate::aws::s3::BucketVersioning> for BucketVersioning {
    fn from(value: crate::aws::s3::BucketVersioning) -> Self {
        Self {
            status: value.status,
        }
    }
}

pub struct BucketLogging {
    target_bucket: String,
    target_prefix: String,
}

impl From<crate::aws::s3::BucketLogging> for BucketLogging {
    fn from(value: crate::aws::s3::BucketLogging) -> Self {
        Self {
            target_bucket: value.target_bucket,
            target_prefix: value.target_prefix,
        }
    }
}

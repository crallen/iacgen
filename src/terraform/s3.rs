use hcl::{
    Block, Body,
    expr::{Traversal, Variable},
};

use crate::{core::TerraformGenerator, terraform::normalize_resource_name};

pub struct Bucket {
    name: String,
    policy: Option<String>,
    public_access_block: Option<BucketPublicAccessBlock>,
}

impl From<crate::aws::s3::Bucket> for Bucket {
    fn from(value: crate::aws::s3::Bucket) -> Self {
        Self {
            name: value.name,
            policy: value.policy,
            public_access_block: value
                .public_access_block
                .map(|pab| BucketPublicAccessBlock::from(pab)),
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

        if let Some(public_access_block) = &self.public_access_block {
            let pab_block = Block::builder("resource")
                .add_label("aws_s3_bucket_public_access_block")
                .add_label(resource_name.clone())
                .add_attribute((
                    "bucket",
                    Traversal::builder(Variable::new("aws_s3_bucket").unwrap())
                        .attr(resource_name.clone())
                        .attr("bucket")
                        .build(),
                ))
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

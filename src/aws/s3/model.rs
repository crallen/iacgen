use crate::core::IntoTerraform;

pub struct Bucket {
    pub name: String,
    pub policy: Option<String>,
    pub public_access_block: Option<BucketPublicAccessBlock>,
    pub encryption: Option<BucketEncryption>,
    pub versioning: Option<BucketVersioning>,
    pub logging: Option<BucketLogging>,
}

impl IntoTerraform for Bucket {
    type TerraformResource = crate::terraform::s3::Bucket;

    fn into_terraform(self) -> Self::TerraformResource {
        self.into()
    }
}

pub struct BucketPublicAccessBlock {
    pub block_public_acls: bool,
    pub block_public_policy: bool,
    pub ignore_public_acls: bool,
    pub restrict_public_buckets: bool,
}

pub struct BucketEncryption {
    pub sse_algorithm: String,
    pub kms_master_key_id: Option<String>,
    pub bucket_key_enabled: bool,
}

impl BucketEncryption {
    pub fn from_aws_rules(rules: &[aws_sdk_s3::types::ServerSideEncryptionRule]) -> Option<Self> {
        let rule = rules.first()?;
        let default_encryption = rule.apply_server_side_encryption_by_default()?;

        Some(Self {
            sse_algorithm: default_encryption.sse_algorithm().to_string(),
            kms_master_key_id: default_encryption
                .kms_master_key_id()
                .map(|id| id.to_string()),
            bucket_key_enabled: rule.bucket_key_enabled().unwrap_or(false),
        })
    }
}

pub struct BucketVersioning {
    pub status: String,
}

pub struct BucketLogging {
    pub target_bucket: String,
    pub target_prefix: String,
}

pub struct BucketBuilder {
    name: String,
    policy: Option<String>,
    public_access_block: Option<BucketPublicAccessBlock>,
    encryption: Option<BucketEncryption>,
    versioning: Option<BucketVersioning>,
    logging: Option<BucketLogging>,
}

impl BucketBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            policy: None,
            public_access_block: None,
            encryption: None,
            versioning: None,
            logging: None,
        }
    }

    pub fn with_policy(mut self, policy: String) -> Self {
        self.policy = Some(policy);
        self
    }

    pub fn with_public_access_block(
        mut self,
        public_access_block: BucketPublicAccessBlock,
    ) -> Self {
        self.public_access_block = Some(public_access_block);
        self
    }

    pub fn with_encryption(mut self, encryption: BucketEncryption) -> Self {
        self.encryption = Some(encryption);
        self
    }

    pub fn with_versioning(mut self, versioning: BucketVersioning) -> Self {
        self.versioning = Some(versioning);
        self
    }

    pub fn with_logging(mut self, logging: BucketLogging) -> Self {
        self.logging = Some(logging);
        self
    }

    pub fn build(self) -> Bucket {
        Bucket {
            name: self.name,
            policy: self.policy,
            public_access_block: self.public_access_block,
            encryption: self.encryption,
            versioning: self.versioning,
            logging: self.logging,
        }
    }
}

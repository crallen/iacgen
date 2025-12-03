use crate::core::IntoTerraform;

pub struct Bucket {
    pub name: String,
    pub policy: Option<String>,
    pub public_access_block: Option<BucketPublicAccessBlock>,
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

pub struct BucketBuilder {
    name: String,
    policy: Option<String>,
    public_access_block: Option<BucketPublicAccessBlock>,
}

impl BucketBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            policy: None,
            public_access_block: None,
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

    pub fn build(self) -> Bucket {
        Bucket {
            name: self.name,
            policy: self.policy,
            public_access_block: self.public_access_block,
        }
    }
}

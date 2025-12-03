use anyhow::Result;

use crate::core::traits::{IntoTerraform, OutputWriter, ResourceFetcher, TerraformGenerator};

pub struct Generator<F>
where
    F: ResourceFetcher,
{
    fetcher: F,
}

impl<F> Generator<F>
where
    F: ResourceFetcher,
{
    pub fn new(fetcher: F) -> Self {
        Self { fetcher }
    }

    pub async fn generate(&self, writer: &mut Box<dyn OutputWriter>) -> Result<()> {
        let aws_resources = self.fetcher.fetch().await?;

        for aws_resource in aws_resources {
            let tf_resource = aws_resource.into_terraform();
            let hcl = tf_resource.to_hcl();
            writer.write(&hcl)?;
            writer.write("\n")?;
        }

        Ok(())
    }
}

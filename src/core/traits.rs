use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ResourceFetcher {
    type Resource: IntoTerraform;
    async fn fetch(&self) -> Result<Vec<Self::Resource>>;
}

pub trait IntoTerraform {
    type TerraformResource: TerraformGenerator;
    fn into_terraform(self) -> Self::TerraformResource;
}

pub trait TerraformGenerator {
    fn to_hcl(&self) -> String;
}

pub trait OutputWriter {
    fn write(&mut self, content: &str) -> Result<()>;
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

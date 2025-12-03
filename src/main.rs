mod aws;
mod cli;
mod core;
mod logging;
mod output;
mod terraform;

use aws_config::BehaviorVersion;
use tracing::{error, info};

use crate::{
    aws::s3::S3Fetcher,
    cli::Command,
    core::{Generator, OutputWriter},
    output::{FileWriter, StdoutWriter},
};

#[tokio::main]
async fn main() {
    let args = cli::parse();

    logging::init(&args);

    let mut config_loader = aws_config::defaults(BehaviorVersion::latest());

    if let Some(profile) = args.profile {
        config_loader = config_loader.profile_name(profile);
    }

    let config = config_loader.load().await;

    let mut writer: Box<dyn OutputWriter> = if let Some(output_path) = args.output {
        info!(
            "Writing Terraform configuration to {}",
            output_path.display()
        );
        Box::new(FileWriter::new(output_path))
    } else {
        info!("Writing Terraform configuration to stdout");
        Box::new(StdoutWriter)
    };

    match args.command {
        Command::S3 => {
            let s3_fetcher = S3Fetcher::new(config);
            let generator = Generator::new(s3_fetcher);
            if let Err(e) = generator.generate(&mut writer).await {
                error!("Failed to generate Terraform configuration: {}", e);
            }
            writer.flush().unwrap();
        }
    }
}

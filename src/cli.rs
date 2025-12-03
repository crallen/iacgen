use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    /// Enable debug logging
    #[clap(short, long, global = true)]
    pub debug: bool,

    /// AWS profile to use
    #[clap(short, long, global = true)]
    pub profile: Option<String>,

    /// Output path to write Terraform configuration to (default: stdout)
    #[clap(short, long, global = true)]
    pub output: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate Terraform configuration for S3 buckets
    S3,
}

pub fn parse() -> Args {
    Args::parse()
}

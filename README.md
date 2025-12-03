# iacgen

**� Proof of Concept**: This project was built as a proof-of-concept for generating Terraform configurations from existing AWS infrastructure. It will not be actively maintained and may not receive updates.

## Overview

`iacgen` scans your AWS account and generates Terraform (HCL) configuration files for your existing resources. This can be useful for importing existing infrastructure into Infrastructure as Code or understanding your current AWS setup.

## Features

Currently supports:
- **S3 Buckets**: Generates `aws_s3_bucket`, `aws_s3_bucket_policy`, and `aws_s3_bucket_public_access_block` resources

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/iacgen`.

## Usage

### Prerequisites

- AWS credentials configured (via `~/.aws/credentials`, environment variables, or IAM role)
- Appropriate IAM permissions to describe resources
- **OpenSSL**: The AWS SDK requires SSL/TLS support

#### macOS SSL Configuration

On macOS, the AWS SDK requires SSL certificate paths to be explicitly set. Before running `iacgen`, export these environment variables:

```bash
export SSL_CERT_FILE=/etc/ssl/cert.pem
export SSL_CERT_DIR=/etc/ssl/certs
```

Alternatively, if you have OpenSSL installed via Homebrew:

```bash
export SSL_CERT_FILE=/opt/homebrew/etc/openssl@3/cert.pem
export SSL_CERT_DIR=/opt/homebrew/etc/openssl@3/certs
```

You can add these to your shell profile (`~/.zshrc`, `~/.bashrc`, etc.) to make them permanent.

### Basic Commands

Generate Terraform configuration for S3 buckets:

```bash
# Output to stdout
iacgen s3

# Write to file
iacgen s3 --output s3.tf

# Use specific AWS profile
iacgen s3 --profile production

# Enable debug logging
iacgen s3 --debug
```

### Options

- `-o, --output <PATH>` - Write output to file instead of stdout
- `-p, --profile <NAME>` - AWS profile to use
- `-d, --debug` - Enable debug logging

## Architecture

The project follows a layered architecture:

```
core/       - Core traits and orchestration logic
aws/        - AWS resource fetching (implements ResourceFetcher)
terraform/  - Terraform HCL generation (implements TerraformGenerator)
output/     - Output handling (stdout, file)
```

This separation allows easy extension to support additional AWS services or IaC formats.

## Limitations

- **Limited Resource Coverage**: Only S3 is currently supported
- **No State Management**: Does not generate or manage Terraform state
- **Basic Error Handling**: May not gracefully handle all AWS API errors
- **No Import Blocks**: Generates configuration only, not `terraform import` commands
- **Read-Only**: Does not modify any AWS resources

## License

MIT - See [LICENSE](LICENSE) file for details

## Contributing

As this is a proof-of-concept project, contributions are not actively being accepted. Feel free to fork the repository for your own use.

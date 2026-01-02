# MinerU SDK for Rust

[![DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/longcipher/mineru-sdk-rs)
[![Context7](https://img.shields.io/badge/Website-context7.com-blue)](https://context7.com/longcipher/mineru-sdk-rs)
[![crates.io](https://img.shields.io/crates/v/mineru-sdk.svg)](https://crates.io/crates/mineru-sdk)
[![docs.rs](https://docs.rs/mineru-sdk/badge.svg)](https://docs.rs/mineru-sdk)

![mineru-sdk-rs](https://socialify.git.ci/longcipher/mineru-sdk-rs/image?font=Source+Code+Pro&language=1&name=1&owner=1&pattern=Circuit+Board&theme=Auto)

A Rust SDK for interacting with the MinerU API, which provides document extraction services for PDFs, DOCX, PPTX, and other formats.

## Features

- **Single File Extraction**: Create extraction tasks for individual files via URL
- **Batch Processing**: Support for batch file uploads and URL-based batch processing
- **Task Management**: Query task status and retrieve extraction results
- **CLI Tool**: Command-line interface for easy testing and integration
- **Async Support**: Built with Tokio for asynchronous operations

## Installation

Add the SDK to your Cargo.toml:

```toml
[dependencies]
mineru-sdk = "0.1.0"
```

Or clone the repository and build locally:

```bash
git clone https://github.com/longcipher/mineru-sdk-rs.git
cd mineru-sdk-rs
cargo build --release
```

## Usage

### SDK Usage

```rust
use mineru_sdk::{MineruClient, ExtractTaskRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MineruClient::new("your-api-token".to_string());

    let request = ExtractTaskRequest {
        url: "https://example.com/document.pdf".to_string(),
        ..Default::default()
    };

    let response = client.create_extract_task(request).await?;
    println!("Task ID: {}", response.data.task_id);

    Ok(())
}
```

### CLI Usage

Set your API token as an environment variable:

```bash
export MINERU_TOKEN="your-api-token"
```

Or pass it as an argument:

```bash
cargo run --bin mineru-cli -- --token "your-api-token" extract --url "https://example.com/document.pdf"
```

Available commands:

- `extract`: Create a single extraction task
- `get-task`: Get the result of a task
- `batch-file`: Get upload URLs for batch file processing
- `batch-url`: Create batch tasks from URLs
- `get-batch`: Get results for batch processing

Example:

```bash
# Create extraction task
mineru-cli extract --url "https://example.com/document.pdf"

# Get task result
mineru-cli get-task --task-id "task-uuid"

# Batch processing
mineru-cli batch-url --url "https://example.com/document.pdf"
mineru-cli get-batch --batch-id "batch-uuid"
```

## API Documentation

For detailed API documentation, visit: [https://mineru.net/apiManage/docs](https://mineru.net/apiManage/docs)

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

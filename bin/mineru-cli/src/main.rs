use clap::{Parser, Subcommand};
use eyre::Result;
use mineru_sdk::{
    BatchFileItem, BatchFileRequest, BatchUrlItem, BatchUrlRequest, ExtractTaskRequest,
    MineruClient,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, env = "MINERU_TOKEN")]
    token: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Extract {
        #[arg(short, long)]
        url: String,
    },
    GetTask {
        #[arg(short, long)]
        task_id: String,
    },
    BatchFile {
        #[arg(short, long)]
        name: String,
    },
    BatchUrl {
        #[arg(short, long)]
        url: String,
    },
    GetBatch {
        #[arg(short, long)]
        batch_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = MineruClient::new(cli.token);

    match cli.command {
        Commands::Extract { url } => {
            let req = ExtractTaskRequest {
                url,
                is_ocr: false,
                enable_formula: true,
                enable_table: true,
                language: "ch".to_string(),
                data_id: None,
                callback: None,
                seed: None,
                extra_formats: None,
                page_ranges: None,
                model_version: "pipeline".to_string(),
            };
            let res = client.create_extract_task(req).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        Commands::GetTask { task_id } => {
            let res = client.get_task_result(&task_id).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        Commands::BatchFile { name } => {
            let req = BatchFileRequest {
                files: vec![BatchFileItem {
                    name,
                    data_id: None,
                }],
                model_version: "pipeline".to_string(),
            };
            let res = client.batch_file_upload_urls(req).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        Commands::BatchUrl { url } => {
            let req = BatchUrlRequest {
                files: vec![BatchUrlItem { url, data_id: None }],
                model_version: "pipeline".to_string(),
            };
            let res = client.batch_url_upload(req).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        Commands::GetBatch { batch_id } => {
            let res = client.get_batch_results(&batch_id).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
    }
    Ok(())
}

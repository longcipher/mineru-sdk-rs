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
    #[command(about = "Create a single extraction task")]
    Extract {
        #[arg(short, long)]
        url: String,
        #[arg(long, default_value = "pipeline")]
        model_version: String,
        #[arg(long)]
        is_ocr: bool,
        #[arg(long, default_value_t = true)]
        enable_formula: bool,
        #[arg(long, default_value_t = true)]
        enable_table: bool,
        #[arg(long, default_value = "ch")]
        language: String,
        #[arg(long)]
        data_id: Option<String>,
        #[arg(long)]
        callback: Option<String>,
        #[arg(long)]
        seed: Option<String>,
        #[arg(long)]
        extra_formats: Option<Vec<String>>,
        #[arg(long)]
        page_ranges: Option<String>,
    },
    #[command(about = "Get the result of a task")]
    GetTask {
        #[arg(short, long)]
        task_id: String,
    },
    #[command(about = "Get upload URLs for batch file processing")]
    BatchFile {
        #[arg(short, long)]
        name: String,
        #[arg(long, default_value = "pipeline")]
        model_version: String,
    },
    #[command(about = "Create batch tasks from URLs")]
    BatchUrl {
        #[arg(short, long)]
        url: String,
        #[arg(long, default_value = "pipeline")]
        model_version: String,
    },
    #[command(about = "Get results for batch processing")]
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
        Commands::Extract {
            url,
            model_version,
            is_ocr,
            enable_formula,
            enable_table,
            language,
            data_id,
            callback,
            seed,
            extra_formats,
            page_ranges,
        } => {
            let req = ExtractTaskRequest {
                url,
                is_ocr,
                enable_formula,
                enable_table,
                language,
                data_id,
                callback,
                seed,
                extra_formats,
                page_ranges,
                model_version,
            };
            let res = client.create_extract_task(req).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        Commands::GetTask { task_id } => {
            let res = client.get_task_result(&task_id).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        Commands::BatchFile {
            name,
            model_version,
        } => {
            let req = BatchFileRequest {
                files: vec![BatchFileItem {
                    name,
                    data_id: None,
                }],
                model_version,
            };
            let res = client.batch_file_upload_urls(req).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        Commands::BatchUrl { url, model_version } => {
            let req = BatchUrlRequest {
                files: vec![BatchUrlItem { url, data_id: None }],
                model_version,
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

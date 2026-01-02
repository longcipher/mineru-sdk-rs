use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractTaskRequest {
    pub url: String,
    #[serde(default)]
    pub is_ocr: bool,
    #[serde(default = "default_true")]
    pub enable_formula: bool,
    #[serde(default = "default_true")]
    pub enable_table: bool,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_formats: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_ranges: Option<String>,
    #[serde(default = "default_model_version")]
    pub model_version: String,
}

fn default_true() -> bool {
    true
}

fn default_language() -> String {
    "ch".to_string()
}

fn default_model_version() -> String {
    "pipeline".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractTaskResponse {
    pub code: i32,
    pub msg: String,
    pub trace_id: String,
    pub data: ExtractTaskData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractTaskData {
    pub task_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResultResponse {
    pub code: i32,
    pub msg: String,
    pub trace_id: String,
    pub data: TaskResultData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResultData {
    pub task_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_zip_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_progress: Option<ExtractProgress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractProgress {
    pub extracted_pages: i32,
    pub total_pages: i32,
    pub start_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchFileRequest {
    pub files: Vec<BatchFileItem>,
    #[serde(default = "default_model_version")]
    pub model_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchFileItem {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchFileResponse {
    pub code: i32,
    pub msg: String,
    pub trace_id: String,
    pub data: BatchFileData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchFileData {
    pub batch_id: String,
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUrlRequest {
    pub files: Vec<BatchUrlItem>,
    #[serde(default = "default_model_version")]
    pub model_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUrlItem {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUrlResponse {
    pub code: i32,
    pub msg: String,
    pub trace_id: String,
    pub data: BatchUrlData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUrlData {
    pub batch_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResultResponse {
    pub code: i32,
    pub msg: String,
    pub trace_id: String,
    pub data: BatchResultData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResultData {
    pub batch_id: String,
    pub extract_result: Vec<BatchExtractResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchExtractResult {
    pub file_name: Option<String>,
    pub state: String,
    pub full_zip_url: Option<String>,
    pub err_msg: Option<String>,
    pub data_id: Option<String>,
    pub extract_progress: Option<ExtractProgress>,
}

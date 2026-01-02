use std::fmt;

use crate::{error::MineruError, types::*};

#[derive(Clone)]
pub struct MineruClient {
    base_url: String,
    token: String,
    client: hpx::Client,
}

impl fmt::Debug for MineruClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MineruClient")
            .field("base_url", &self.base_url)
            .field("token", &"***")
            .finish()
    }
}

impl MineruClient {
    pub fn new(token: String) -> Self {
        Self {
            base_url: "https://mineru.net".to_string(),
            token,
            client: hpx::Client::new(),
        }
    }

    pub async fn create_extract_task(
        &self,
        request: ExtractTaskRequest,
    ) -> Result<ExtractTaskResponse, MineruError> {
        let url = format!("{}/api/v4/extract/task", self.base_url);
        let body = serde_json::to_vec(&request)?;
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        let bytes = resp.bytes().await?;
        let result: ExtractTaskResponse = serde_json::from_slice(&bytes)?;
        Ok(result)
    }

    pub async fn get_task_result(&self, task_id: &str) -> Result<TaskResultResponse, MineruError> {
        let url = format!("{}/api/v4/extract/task/{}", self.base_url, task_id);
        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        let bytes = resp.bytes().await?;
        let result: TaskResultResponse = serde_json::from_slice(&bytes)?;
        Ok(result)
    }

    pub async fn batch_file_upload_urls(
        &self,
        request: BatchFileRequest,
    ) -> Result<BatchFileResponse, MineruError> {
        let url = format!("{}/api/v4/file-urls/batch", self.base_url);
        let body = serde_json::to_vec(&request)?;
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        let bytes = resp.bytes().await?;
        let result: BatchFileResponse = serde_json::from_slice(&bytes)?;
        Ok(result)
    }

    pub async fn batch_url_upload(
        &self,
        request: BatchUrlRequest,
    ) -> Result<BatchUrlResponse, MineruError> {
        let url = format!("{}/api/v4/extract/task/batch", self.base_url);
        let body = serde_json::to_vec(&request)?;
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        let bytes = resp.bytes().await?;
        let result: BatchUrlResponse = serde_json::from_slice(&bytes)?;
        Ok(result)
    }

    pub async fn get_batch_results(
        &self,
        batch_id: &str,
    ) -> Result<BatchResultResponse, MineruError> {
        let url = format!(
            "{}/api/v4/extract-results/batch/{}",
            self.base_url, batch_id
        );
        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        let bytes = resp.bytes().await?;
        let result: BatchResultResponse = serde_json::from_slice(&bytes)?;
        Ok(result)
    }
}

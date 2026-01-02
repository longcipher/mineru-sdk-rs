pub mod client;
pub mod error;
pub mod types;

pub use client::MineruClient;
pub use error::MineruError;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_serialization() {
        let req = ExtractTaskRequest {
            url: "http://example.com".to_string(),
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
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("http://example.com"));
    }
}

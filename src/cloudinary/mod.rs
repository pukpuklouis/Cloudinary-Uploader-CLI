use anyhow::{Context, Result};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use mime_guess::from_path;

use crate::config::CloudinaryConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub public_id: String,
    pub version: u64,
    pub signature: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub format: String,
    pub resource_type: String,
    pub created_at: String,
    pub tags: Option<Vec<String>>,
    pub bytes: u64,
    pub url: String,
    pub secure_url: String,
}

#[derive(Clone)]
pub struct CloudinaryUploader {
    config: CloudinaryConfig,
    client: reqwest::Client,
}

impl CloudinaryUploader {
    pub fn new(config: CloudinaryConfig) -> Self {
        CloudinaryUploader {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn upload_file(
        &self,
        file_path: &Path,
        folder: Option<&str>,
        transformation: Option<&str>,
    ) -> Result<UploadResponse> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_secs()
            .to_string();

        let mut file = File::open(file_path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .context("Invalid file name")?;

        let resource_type = self.determine_resource_type(file_path);
        
        // Build the form and collect parameters for signature
        let mut form = Form::new()
            .text("api_key", self.config.api_key.clone())
            .text("timestamp", timestamp.clone());
            
        // Create a map of parameters for signature generation
        use std::collections::HashMap;
        let mut params = HashMap::new();
        params.insert("timestamp".to_string(), timestamp.clone());

        // Add folder if specified
        if let Some(folder_name) = folder.or(Some(&self.config.default_folder)) {
            if !folder_name.is_empty() {
                form = form.text("folder", folder_name.to_string());
                params.insert("folder".to_string(), folder_name.to_string());
            }
        }

        // Add transformation if specified
        if let Some(transform) = transformation {
            match transform {
                "webp" => {
                    form = form.text("format", "webp");
                    params.insert("format".to_string(), "webp".to_string());
                },
                "avif" => {
                    form = form.text("format", "avif");
                    params.insert("format".to_string(), "avif".to_string());
                },
                _ => {}
            }
        }

        // Generate signature - sort parameters alphabetically as required by Cloudinary
        let mut sorted_params: Vec<(String, String)> = params.into_iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(&b.0));
        
        // Build the string to sign
        let string_to_sign = sorted_params
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&");
        
        // Generate the signature using SHA-1
        use sha1::{Digest, Sha1};
        let mut hasher = Sha1::new();
        hasher.update(string_to_sign.as_bytes());
        hasher.update(self.config.api_secret.as_bytes());
        let signature = format!("{:x}", hasher.finalize());

        form = form.text("signature", signature);

        // Add file part
        let mime_type = from_path(file_path).first_or_octet_stream();
        let part = Part::bytes(buffer)
            .file_name(file_name.to_string())
            .mime_str(mime_type.as_ref())?;

        form = form.part("file", part);

        // Send request
        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/{}/upload",
            self.config.cloud_name, resource_type
        );

        let response = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .context("Failed to send upload request")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Upload failed: {}", error_text);
        }

        let upload_response = response
            .json::<UploadResponse>()
            .await
            .context("Failed to parse upload response")?;

        Ok(upload_response)
    }

    fn determine_resource_type(&self, file_path: &Path) -> &'static str {
        let mime = from_path(file_path).first_or_octet_stream();
        let mime_type = mime.type_().as_str();

        match mime_type {
            "image" => "image",
            "video" => "video",
            "audio" => "raw",
            _ => "auto",
        }
    }

    pub fn get_url(&self, public_id: &str, resource_type: &str, transformation: Option<&str>) -> String {
        let base_url = format!(
            "https://res.cloudinary.com/{}/{}/upload",
            self.config.cloud_name, resource_type
        );

        match transformation {
            Some("webp") => format!("{}/f_webp/{}", base_url, public_id),
            Some("avif") => format!("{}/f_avif/{}", base_url, public_id),
            _ => format!("{}/{}", base_url, public_id),
        }
    }
}

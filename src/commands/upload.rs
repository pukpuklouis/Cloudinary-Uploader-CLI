use anyhow::{Context, Result};
use std::path::PathBuf;
use futures::future::try_join_all;

use crate::cloudinary::CloudinaryUploader;
use crate::config::Config;
use crate::utils::{create_progress_bar, print_error, print_info, print_success, save_urls_to_file};
use crate::utils::fzf::{get_files_in_directory, select_files};

pub async fn execute(
    path: Option<String>,
    folder: Option<String>,
    transform: Option<String>,
    output: Option<String>,
) -> Result<()> {
    // Load configuration
    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            if let Some(env_config) = Config::from_env() {
                env_config
            } else {
                print_error("No configuration found. Please run 'cloudy init' first or set CLOUDINARY_URL environment variable.");
                return Ok(());
            }
        }
    };

    // Create Cloudinary uploader
    let uploader = CloudinaryUploader::new(config.cloudinary.clone());

    // Determine files to upload
    let files = if let Some(path_str) = path {
        let path = PathBuf::from(path_str);
        
        if path.is_dir() {
            get_files_in_directory(&path).context("Failed to get files in directory")?
        } else if path.is_file() {
            vec![path]
        } else {
            print_error(&format!("Path not found: {}", path.display()));
            return Ok(());
        }
    } else {
        // Interactive selection using fzf
        print_info("Select files to upload (use Tab to select multiple files):");
        select_files().context("Failed to select files")?
    };

    if files.is_empty() {
        print_error("No files selected for upload.");
        return Ok(());
    }

    print_info(&format!("Uploading {} files to Cloudinary...", files.len()));
    
    // Create progress bar
    let pb = create_progress_bar(files.len() as u64);
    
    // Upload files in parallel
    let mut upload_tasks = Vec::new();
    
    for file in files {
        let uploader = uploader.clone();
        let pb = pb.clone();
        let folder_clone = folder.clone();
        let transform_clone = transform.clone();
        
        let task = tokio::spawn(async move {
            let folder_ref = folder_clone.as_deref();
            let transform_ref = transform_clone.as_deref();
            
            let result = uploader.upload_file(&file, folder_ref, transform_ref).await;
            pb.inc(1);
            
            match result {
                Ok(response) => {
                    pb.println(format!("Uploaded: {} -> {}", file.display(), response.secure_url));
                    Ok(response.secure_url)
                }
                Err(err) => {
                    pb.println(format!("Failed to upload {}: {}", file.display(), err));
                    Err(err)
                }
            }
        });
        
        upload_tasks.push(task);
    }
    
    // Wait for all uploads to complete
    let results: Vec<Result<String, _>> = try_join_all(upload_tasks)
        .await?
        .into_iter()
        .collect();
    
    pb.finish_with_message("Upload completed");
    
    // Collect successful uploads
    let successful_urls: Vec<String> = results
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();
    
    print_success(&format!("Successfully uploaded {} files.", successful_urls.len()));
    
    // Save URLs to file if requested
    if let Some(output_path) = output {
        let output_path = PathBuf::from(output_path);
        save_urls_to_file(&successful_urls, &output_path)
            .context("Failed to save URLs to file")?;
        
        print_success(&format!("URLs saved to: {}", output_path.display()));
    }
    
    Ok(())
}

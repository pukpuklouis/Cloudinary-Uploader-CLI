use anyhow::Result;
use clap::{Parser, Subcommand};

mod cloudinary;
mod commands;
mod config;
mod utils;

#[derive(Parser)]
#[command(name = "cld-upload-helper")]
#[command(author = "pukpuklouis <pukpuk.tw@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Cloudinary Uploader CLI - A tool for uploading media to Cloudinary", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the Cloudinary Uploader CLI configuration
    Init,
    
    /// Upload files to Cloudinary
    Upload {
        /// Path to a file or directory to upload (optional, defaults to interactive selection)
        #[arg(value_name = "PATH")]
        path: Option<String>,
        
        /// Specify Cloudinary destination folder
        #[arg(short, long, value_name = "FOLDER")]
        folder: Option<String>,
        
        /// Apply transformation (e.g., webp, avif)
        #[arg(short, long, value_name = "FORMAT")]
        transform: Option<String>,
        
        /// Save URLs to a specified file
        #[arg(short, long, value_name = "FILE")]
        output: Option<String>,
    },
    
    /// View or edit current configuration
    Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            commands::init::execute()?;
        }
        Commands::Upload { path, folder, transform, output } => {
            commands::upload::execute(path.clone(), folder.clone(), transform.clone(), output.clone()).await?;
        }
        Commands::Config => {
            commands::config::execute()?;
        }
    }

    Ok(())
}

pub mod fzf;

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

pub fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}

pub fn save_urls_to_file(urls: &[String], output_file: &Path) -> anyhow::Result<()> {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(output_file)?;
    for url in urls {
        writeln!(file, "{}", url)?;
    }
    
    Ok(())
}

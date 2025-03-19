use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

/// Check if fzf is installed on the system
pub fn is_fzf_available() -> bool {
    Command::new("which")
        .arg("fzf")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Select files using fzf
pub fn select_files() -> Result<Vec<PathBuf>> {
    if !is_fzf_available() {
        anyhow::bail!("fzf is not installed. Please install it first or specify file paths directly.");
    }

    // Run find command to get all files in the current directory
    let mut find_cmd = Command::new("find")
        .arg(".")
        .arg("-type")
        .arg("f")
        .arg("-not")
        .arg("-path")
        .arg("*/\\.*")  // Exclude hidden files
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run find command")?;

    let find_stdout = find_cmd.stdout.take()
        .context("Failed to get find command stdout")?;

    // Pipe find output to fzf
    let mut fzf_cmd = Command::new("fzf")
        .arg("--multi")
        .arg("--preview")
        .arg("file -b {}")
        .stdin(find_stdout)
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run fzf command")?;

    // Read selected files from fzf output
    let fzf_stdout = fzf_cmd.stdout.take()
        .context("Failed to get fzf output")?;
    let reader = BufReader::new(fzf_stdout);
    
    let mut selected_files = Vec::new();
    for line in reader.lines() {
        let line = line.context("Failed to read line from fzf output")?;
        let path = PathBuf::from(line);
        selected_files.push(path);
    }

    // Wait for the commands to finish
    find_cmd.wait().context("Failed to wait for find command")?;
    fzf_cmd.wait().context("Failed to wait for fzf command")?;

    if selected_files.is_empty() {
        anyhow::bail!("No files selected");
    }

    Ok(selected_files)
}

/// Select a directory using fzf
pub fn select_directory() -> Result<PathBuf> {
    if !is_fzf_available() {
        anyhow::bail!("fzf is not installed. Please install it first or specify directory path directly.");
    }

    // Run find command to get all directories in the current directory
    let mut find_cmd = Command::new("find")
        .arg(".")
        .arg("-type")
        .arg("d")
        .arg("-not")
        .arg("-path")
        .arg("*/\\.*")  // Exclude hidden directories
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run find command")?;

    let find_stdout = find_cmd.stdout.take()
        .context("Failed to get find command stdout")?;

    // Pipe find output to fzf
    let mut fzf_cmd = Command::new("fzf")
        .arg("--preview")
        .arg("ls -la {}")
        .stdin(find_stdout)
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run fzf command")?;

    // Read selected directory from fzf output
    let fzf_stdout = fzf_cmd.stdout.take()
        .context("Failed to get fzf output")?;
    let reader = BufReader::new(fzf_stdout);
    
    let lines: Vec<String> = reader.lines()
        .collect::<std::io::Result<Vec<String>>>()
        .context("Failed to read lines from fzf output")?;

    // Wait for the commands to finish
    find_cmd.wait().context("Failed to wait for find command")?;
    fzf_cmd.wait().context("Failed to wait for fzf command")?;

    if lines.is_empty() {
        anyhow::bail!("No directory selected");
    }

    Ok(PathBuf::from(&lines[0]))
}

/// Get all files in a directory (recursively)
pub fn get_files_in_directory(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        files.push(entry.path().to_path_buf());
    }
    
    Ok(files)
}

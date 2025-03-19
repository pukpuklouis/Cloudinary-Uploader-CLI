Below is a Product Requirements Document (PRD) draft for your Rust-based CLI tool that helps users upload media to Cloudinary via its API. This PRD outlines the purpose, features, and actionable plan for development. Feel free to adjust it based on your specific needs!

---

# Product Requirements Document (PRD)  
## Project Name: Cloudinary Uploader CLI  
**Version:** 1.0  
**Date:** March 18, 2025  
**Author:** [Your Name]  

---

## 1. Overview  
### 1.1 Purpose  
The Cloudinary Uploader CLI is a lightweight, Rust-based command-line tool designed to simplify uploading media files (images, videos, etc.) to Cloudinary, a cloud-based media management platform. It provides an intuitive interface for selecting files or folders using `fzf` (a fuzzy finder), uploading them to user-defined remote folders on Cloudinary, retrieving public URLs, and applying basic transformations (e.g., converting to WebP or AVIF). The tool aims to streamline workflows for developers and content creators integrating Cloudinary into web development projects.

### 1.2 Goals  
- Enable users to upload single files, multiple files, or entire folders to Cloudinary via a CLI.  
- Provide a simple configuration system for setting Cloudinary API credentials and preferences.  
- Allow users to select files/folders interactively using `fzf`.  
- Support basic media transformations (e.g., format conversion to WebP or AVIF).  
- Return Cloudinary URLs for use in web development.  
- Ensure a fast, reliable, and secure experience leveraging Rust’s performance and safety features.  

### 1.3 Target Audience  
- Web developers integrating Cloudinary into their projects.  
- Content creators managing media assets in the cloud.  
- CLI enthusiasts who prefer terminal-based workflows.  

---

## 2. Features  

### 2.1 Core Functionality  
1. **File/Folder Selection with fzf**  
   - Users can select a single file, multiple files, or an entire folder to upload using `fzf` for an interactive, fuzzy-search experience.  
   - Command: `cloudy upload` (default behavior prompts file/folder selection).  

2. **Cloudinary API Integration**  
   - Upload selected media to Cloudinary using its REST API.  
   - Support for images, videos, and other Cloudinary-compatible file types.  

3. **Configuration Management**  
   - Users set Cloudinary API credentials (API key, API secret, cloud name) via a configuration file (e.g., `.cloudyrc` in JSON or TOML format).  
   - Optional environment variable support for credentials (e.g., `CLOUDINARY_URL`).  

4. **Remote Folder Specification**  
   - Users can specify a remote folder in Cloudinary where files will be uploaded (e.g., `cloudy upload --folder "my-project/assets"`).  
   - Default folder: root directory if unspecified.  

5. **URL Retrieval**  
   - After upload, return the public Cloudinary URL for each uploaded file.  
   - URLs are displayed in the terminal and optionally saved to a file (e.g., `urls.txt`).  

6. **Basic Transformations**  
   - Allow users to apply transformations during upload, such as converting images to WebP or AVIF.  
   - Example: `cloudy upload --transform webp`.  
   - Return transformed URLs alongside original URLs.  

### 2.2 CLI Commands  
- `cloudy init`: Generate a default configuration file.  
- `cloudy upload [path]`: Upload a specific file or folder (optional path; defaults to `fzf` selection).  
  - Flags:  
    - `--folder <remote-folder>`: Specify Cloudinary destination folder.  
    - `--transform <format>`: Apply transformation (e.g., `webp`, `avif`).  
    - `--output <file>`: Save URLs to a specified file.  
- `cloudy config`: View or edit current configuration.  

### 2.3 Non-Functional Requirements  
- **Performance:** Leverage Rust’s concurrency for parallel uploads of multiple files.  
- **Security:** Securely handle API credentials and avoid exposing them in logs or terminal output.  
- **Portability:** Cross-platform support (Windows, macOS, Linux).  

---

## 3. Technical Requirements  

### 3.1 Tech Stack  
- **Language:** Rust  
- **Dependencies:**  
  - `reqwest`: For HTTP requests to Cloudinary API.  
  - `serde`: For configuration file parsing (JSON/TOML).  
  - `fzf`: Integration via Rust bindings or subprocess for file selection.  
  - `clap`: For CLI argument parsing.  
  - `tokio`: For asynchronous operations (e.g., parallel uploads).  
  - `cloudinary`: Your Rust library for Cloudinary operations.  
  - `ratatui`: For building a CLI interface.  

### 3.2 Cloudinary Integration  
- Use Cloudinary's Upload API (`POST /v1_1/<cloud_name>/<resource_type>/upload`) for uploading from local files, remote files, and data URLs.  
- Support signed uploads for security using API secret.  
- Handle transformation parameters in the API payload for resizing, cropping, and padding.  
- Include asset management functions like destroying assets by public ID and listing assets by tags.  

### 3.3 Configuration File  
- Default location: `~/.cloudyrc`.  
- Example structure (TOML):  
  ```toml
  [cloudinary]
  cloud_name = "my-cloud"
  api_key = "1234567890"
  api_secret = "abc123xyz"
  default_folder = "uploads"
  ```

---
### Minimum Rust Version
The minimum supported Rust version for this project is 1.65.

## 4. Actionable Plan  

### 4.1 Phase 1: Setup & Core Upload (Weeks 1-2)  
- [ ] Set up Rust project with `cargo init`.  
- [ ] Implement basic CLI structure with `clap`.  
- [ ] Add Cloudinary API client using `reqwest`.  
- [ ] Build single-file upload functionality with hardcoded credentials.  
- [ ] Test upload with sample image/video and print returned URL.  

### 4.2 Phase 2: Configuration & File Selection (Weeks 3-4)  
- [ ] Add configuration file parsing with `serde`.  
- [ ] Support environment variable fallback for credentials.  
- [ ] Integrate `fzf` for interactive file/folder selection.  
- [ ] Enable multi-file/folder uploads with progress feedback.  

### 4.3 Phase 3: Advanced Features (Weeks 5-6)  
- [ ] Add `--folder` flag for remote folder specification.  
- [ ] Implement basic transformations (WebP, AVIF) via API parameters.  
- [ ] Return transformed URLs and save to file with `--output`.  
- [ ] Optimize uploads with `tokio` for concurrency.  

### 4.4 Phase 4: Testing & Polish (Weeks 7-8)  
- [ ] Write unit tests for API calls, config parsing, and file selection.  
- [ ] Test across platforms (Linux, macOS, Windows).  
- [ ] Add error handling (e.g., invalid credentials, file not found).  
- [ ] Document usage in `README.md` with examples.  

### 4.5 Phase 5: Release (Week 9)  
- [ ] Package CLI with `cargo build --release`.  
- [ ] Publish to crates.io or distribute via GitHub releases.  
- [ ] Gather user feedback for v1.1 features (e.g., more transformations).  

---

## 5. Success Metrics  
- Uploads complete successfully for 95% of valid files within 5 seconds (single file).  
- CLI installs and runs without errors on all supported platforms.  
- Positive feedback from at least 10 early adopters on GitHub.  

---

## 6. Risks & Mitigations  
- **Risk:** Cloudinary API rate limits.  
  - **Mitigation:** Implement retry logic and inform users of limits.  
- **Risk:** `fzf` dependency issues on some systems.  
  - **Mitigation:** Fall back to basic file path input if `fzf` fails.  
- **Risk:** Credential exposure.  
  - **Mitigation:** Use Rust’s secure string handling and avoid logging sensitive data.  

---

This PRD provides a clear roadmap for your Cloudinary Uploader CLI. Let me know if you’d like to refine any section, add more details, or adjust the timeline! Ready to start coding?

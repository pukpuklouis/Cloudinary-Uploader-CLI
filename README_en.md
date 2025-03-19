# Cloudinary Uploader CLI

A lightweight, Rust-based command-line tool designed to simplify uploading media files (images, videos, etc.) to Cloudinary, a cloud-based media management platform. It provides an intuitive interface for selecting files or folders using `fzf` (a fuzzy finder), uploading them to user-defined remote folders on Cloudinary, retrieving public URLs, and applying basic transformations.
[中文版README](README.md)

## Features

- Upload single files, multiple files, or entire folders to Cloudinary
- Interactive file selection using `fzf`
- Specify remote folders for uploads
- Apply basic transformations (WebP, AVIF)
- Get public URLs for uploaded media
- Save URLs to a file for easy reference

## Installation

### Prerequisites

- Rust 1.65 or later
- `fzf` for interactive file selection

### Building from Source

```bash
git clone https://github.com/yourusername/cld-upload-helper.git
cd cld-upload-helper
cargo build --release
```

The binary will be available at `./target/release/cld-upload-helper`.

## Usage

### Initialize Configuration

Before using the tool, you need to set up your Cloudinary credentials:

```bash
cld-upload-helper init
```

This will prompt you to enter your Cloudinary cloud name, API key, and API secret. Alternatively, you can set the `CLOUDINARY_URL` environment variable in the format `cloudinary://<api_key>:<api_secret>@<cloud_name>`.

### Upload Files

Upload files interactively using `fzf`:

```bash
cld-upload-helper upload
```

Upload a specific file or directory:

```bash
cld-upload-helper upload path/to/file.jpg
cld-upload-helper upload path/to/directory
```

Upload with options:

```bash
# Upload to a specific folder in Cloudinary
cld-upload-helper upload --folder my-project/assets

# Convert images to WebP during upload
cld-upload-helper upload --transform webp

# Save URLs to a file
cld-upload-helper upload --output urls.txt

# Combine options
cld-upload-helper upload path/to/directory --folder my-project/assets --transform webp --output urls.txt
```

### View Configuration

View your current configuration:

```bash
cld-upload-helper config
```

## Configuration

The configuration is stored in `~/.cloudyrc` in TOML format:

```toml
[cloudinary]
cloud_name = "your-cloud-name"
api_key = "your-api-key"
api_secret = "your-api-secret"
default_folder = "optional-default-folder"
```

## Development

Check the [DEVELOPER_NOTES.md](DEVELOPER_NOTES.md) file for information about the current state of development, what's been implemented, and what's planned for future releases.

## License

MIT

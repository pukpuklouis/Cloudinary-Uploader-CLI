# Cloudinary Uploader CLI
Cloudinary 上傳用的工具指令

一個輕量級的、基於 Rust 的命令行工具，旨在簡化將媒體文件（圖像、視頻等）上傳到 Cloudinary（一個基於雲的媒體管理平台）的過程。它提供了一個直觀的界面，使用 `fzf`（模糊查找器）選擇文件或文件夾，將它們上傳到 Cloudinary 上的用戶定義的遠程文件夾，檢索公共 URL，並應用基本轉換。

 [English README](README_en.md)


## 功能特點

- 將單個文件、多個文件或整個文件夾上傳到 Cloudinary
- 使用 `fzf` 進行互動式文件選擇
- 指定上傳的遠程文件夾
- 應用基本轉換（WebP、AVIF）
- 獲取已上傳媒體的公共 URL
- 將 URL 保存到文件中以便於參考

## 安裝

### 前置要求

- Rust 1.65 或更高版本
- 用於互動式文件選擇的 `fzf`

### 要用源程式碼開發

```bash
git clone https://github.com/yourusername/cld-upload-helper.git
cd cld-upload-helper
cargo build --release
```

build後，執行文件將位於 `./target/release/cld-upload-helper`。

## 使用方法

### 初始化配置

在使用工具之前，您需要設置 Cloudinary 憑證：

```bash
cld-upload-helper init
```

這將提示您輸入 Cloudinary 雲名稱、API 密鑰和 API 密碼。或者，您可以設置 `CLOUDINARY_URL` 環境變量，格式為 `cloudinary://<api_key>:<api_secret>@<cloud_name>`。

### 上傳文件

使用 `fzf` 互動式上傳文件：

```bash
cld-upload-helper upload
```

上傳特定文件或目錄：

```bash
cld-upload-helper upload path/to/file.jpg
cld-upload-helper upload path/to/directory
```

使用選項上傳：

```bash
# 上傳到 Cloudinary 中的特定文件夾
cld-upload-helper upload --folder my-project/assets

# 在上傳過程中將圖像轉換為 WebP
cld-upload-helper upload --transform webp

# 將 URL 保存到文件
cld-upload-helper upload --output urls.txt

# 組合選項
cld-upload-helper upload path/to/directory --folder my-project/assets --transform webp --output urls.txt
```

### 查看配置

查看您當前的配置：

```bash
cld-upload-helper config
```

## 配置

配置以 TOML 格式存儲在 `~/.cloudyrc` 中：

```toml
[cloudinary]
cloud_name = "your-cloud-name"
api_key = "your-api-key"
api_secret = "your-api-secret"
default_folder = "optional-default-folder"
```

## 開發

查看 [DEVELOPER_NOTES.md](DEVELOPER_NOTES.md) 文件，了解有關當前開發狀態、已實現的功能以及計劃在未來版本中實現的功能的信息。

## 許可證

MIT

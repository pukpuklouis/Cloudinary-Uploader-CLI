# Cloudinary Uploader CLI
Cloudinary 上傳用的工具指令

一個用 Rust 寫的輕量級命令行小工具，專為那些懶得手動搞亂七八糟上傳流程的人設計，目標是把圖片、影片之類的媒體檔案丟到 Cloudinary（那個超方便的雲端媒體管理平台）變得超簡單。它有個超直覺的介面，靠著 fzf（模糊搜尋神器）讓你輕鬆挑選檔案或資料夾，然後一鍵上傳到你在 Cloudinary 自訂的遠端資料夾，還能馬上拿到公開 URL，甚至順手搞點基本轉換。簡單說，就是把麻煩的事變得輕鬆。

 [English README](README_en.md)


## 功能特色
- 支援單一檔案、多個檔案或整個資料夾上傳至 Cloudinary。
- 整合 `fzf` 提供互動式檔案選擇，操作更便捷。
- 可指定上傳至 Cloudinary 的遠端資料夾。
- 提供基本轉換功能（如 WebP、AVIF），滿足簡單編輯需求。
- 上傳完成後即可取得公開 URL，方便分享與使用。
- 可將 URL 儲存至檔案，方便後續參考與管理。

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
./cld-upload-helper init
```

這將提示您輸入 Cloudinary cloud name、API 密鑰和 API 密碼。或者，您可以設置 `CLOUDINARY_URL` 環境變量，格式為 `cloudinary://<api_key>:<api_secret>@<cloud_name>`。

### 上傳文件

使用 `fzf` 互動式上傳文件：

```bash
./cld-upload-helper upload
```

上傳特定文件或目錄：

```bash
./cld-upload-helper upload path/to/file.jpg
./cld-upload-helper upload path/to/directory
```

使用選項上傳：

```bash
# 上傳到 Cloudinary 中的特定文件夾
./cld-upload-helper upload --folder my-project/assets

# 在上傳過程中將圖像轉換為 WebP
./cld-upload-helper upload --transform webp

# 將 URL 保存到文件
./cld-upload-helper upload --output urls.txt

# 組合選項
./cld-upload-helper upload path/to/directory --folder my-project/assets --transform webp --output urls.txt
```

### 查看配置

查看您當前的配置：

```bash
./cld-upload-helper config
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

MIT 授權

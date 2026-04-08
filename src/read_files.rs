use base64::Engine;
use std::path::Path;

/// A file entry ready for the deployment/commit API.
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileEntry {
    pub path: String,
    pub content: String,
    pub encoding: &'static str,
}

/// Read all files from a local directory, returning base64-encoded entries
/// suitable for the deployment or commit APIs.
///
/// Ignores `node_modules` directories. Hidden files (dotfiles) are included.
pub async fn read_files(directory: impl AsRef<Path>) -> std::io::Result<Vec<FileEntry>> {
    let directory = directory.as_ref().to_path_buf();
    let mut files = Vec::new();
    collect_files(&directory, &directory, &mut files).await?;
    Ok(files)
}

#[async_recursion::async_recursion]
async fn collect_files(root: &Path, dir: &Path, files: &mut Vec<FileEntry>) -> std::io::Result<()> {
    let mut entries = tokio::fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        // Skip node_modules
        if name == "node_modules" {
            continue;
        }

        let file_type = entry.file_type().await?;
        if file_type.is_dir() {
            collect_files(root, &path, files).await?;
        } else if file_type.is_file() {
            let relative = path.strip_prefix(root).map_err(std::io::Error::other)?;
            // Use forward slashes for consistency
            let relative_str = relative
                .components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/");

            let content = tokio::fs::read(&path).await?;
            let encoded = base64::engine::general_purpose::STANDARD.encode(&content);

            files.push(FileEntry {
                path: relative_str,
                content: encoded,
                encoding: "base64",
            });
        }
    }
    Ok(())
}

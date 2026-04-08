use crate::openapi::types;
use crate::vm::VmHandle;

/// File/directory stat information.
#[derive(Debug, Clone)]
pub struct FileStat {
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
    pub is_symlink: bool,
    pub permissions: String,
    pub owner: String,
    pub group: String,
    pub modified: String,
}

/// Filesystem operations on a VM.
///
/// Wraps the raw file API endpoints and exec-based helpers.
pub struct FileSystem {
    vm: VmHandle,
}

impl FileSystem {
    pub(crate) fn new(vm: VmHandle) -> Self {
        Self { vm }
    }

    fn vm_id_str(&self) -> String {
        self.vm.vm_id.to_string()
    }

    /// Read a file as raw bytes (base64-decoded).
    pub async fn read_file(&self, filepath: &str) -> Result<Vec<u8>, FsError> {
        use base64::Engine;

        let resp = self
            .vm
            .client
            .get_file(&self.vm_id_str(), filepath)
            .await
            .map_err(|e| FsError(format!("get_file: {e}")))?;

        match resp.into_inner() {
            types::FileSystemResponse::Variant0 {
                content, encoding, ..
            } => {
                if encoding.as_ref() == Some(&types::FileEncoding::Base64) {
                    base64::engine::general_purpose::STANDARD
                        .decode(&content)
                        .map_err(|e| FsError(format!("base64 decode: {e}")))
                } else {
                    Ok(content.into_bytes())
                }
            }
            types::FileSystemResponse::Variant1 { .. } => {
                Err(FsError("path is a directory, not a file".into()))
            }
        }
    }

    /// Read a text file as a UTF-8 string.
    pub async fn read_text_file(&self, filepath: &str) -> Result<String, FsError> {
        let bytes = self.read_file(filepath).await?;
        String::from_utf8(bytes).map_err(|e| FsError(format!("UTF-8 error: {e}")))
    }

    /// Write raw bytes to a file (base64-encoded for the API).
    pub async fn write_file(&self, filepath: &str, content: &[u8]) -> Result<(), FsError> {
        use base64::Engine;

        let encoded = base64::engine::general_purpose::STANDARD.encode(content);
        let body = types::WriteFileRequest {
            content: encoded,
            encoding: Some(types::FileEncoding::Base64),
        };
        self.vm
            .client
            .put_file(&self.vm_id_str(), filepath, &body)
            .await
            .map_err(|e| FsError(format!("put_file: {e}")))?;
        Ok(())
    }

    /// Write a UTF-8 string to a file.
    pub async fn write_text_file(&self, filepath: &str, content: &str) -> Result<(), FsError> {
        self.write_file(filepath, content.as_bytes()).await
    }

    /// Read a directory listing.
    pub async fn read_dir(&self, path: &str) -> Result<Vec<types::FileInfo>, FsError> {
        let resp = self
            .vm
            .client
            .get_file(&self.vm_id_str(), path)
            .await
            .map_err(|e| FsError(format!("get_file: {e}")))?;

        match resp.into_inner() {
            types::FileSystemResponse::Variant1 { files } => Ok(files),
            types::FileSystemResponse::Variant0 { .. } => {
                Err(FsError("path is a file, not a directory".into()))
            }
        }
    }

    /// Create a directory. If `recursive` is true, creates parent directories.
    pub async fn mkdir(&self, path: &str, recursive: bool) -> Result<(), FsError> {
        let flag = if recursive { "-p " } else { "" };
        let result = self
            .vm
            .exec(&format!("mkdir {flag}\"{path}\""))
            .await
            .map_err(|e| FsError(format!("exec: {e}")))?;
        let inner = result.into_inner();
        if inner.status_code != Some(0) {
            return Err(FsError(format!(
                "mkdir failed: {}",
                inner.stderr.as_deref().unwrap_or("unknown error")
            )));
        }
        Ok(())
    }

    /// Remove a file or directory. If `recursive` is true, removes directories recursively.
    pub async fn remove(&self, path: &str, recursive: bool) -> Result<(), FsError> {
        let flag = if recursive { "-rf" } else { "-f" };
        let result = self
            .vm
            .exec(&format!("rm {flag} \"{path}\""))
            .await
            .map_err(|e| FsError(format!("exec: {e}")))?;
        let inner = result.into_inner();
        if inner.status_code != Some(0) {
            return Err(FsError(format!(
                "remove failed: {}",
                inner.stderr.as_deref().unwrap_or("unknown error")
            )));
        }
        Ok(())
    }

    /// Check if a path exists.
    pub async fn exists(&self, path: &str) -> Result<bool, FsError> {
        let result = self
            .vm
            .exec(&format!("test -e \"{path}\""))
            .await
            .map_err(|e| FsError(format!("exec: {e}")))?;
        Ok(result.into_inner().status_code == Some(0))
    }

    /// Get file/directory stats.
    pub async fn stat(&self, path: &str) -> Result<FileStat, FsError> {
        let cmd = format!(
            "stat -c '%s|%F|%a|%U|%G|%y' \"{path}\" 2>/dev/null || \
             stat -f '%z|%HT|%p|%Su|%Sg|%Sm' \"{path}\""
        );
        let result = self
            .vm
            .exec(&cmd)
            .await
            .map_err(|e| FsError(format!("exec: {e}")))?;
        let inner = result.into_inner();
        if inner.status_code != Some(0) {
            return Err(FsError(format!(
                "stat failed: {}",
                inner.stderr.as_deref().unwrap_or("unknown error")
            )));
        }
        let stdout = inner.stdout.as_deref().unwrap_or("");
        let parts: Vec<&str> = stdout.trim().split('|').collect();
        if parts.len() < 6 {
            return Err(FsError(format!("invalid stat output: {stdout}")));
        }
        Ok(FileStat {
            size: parts[0].parse().unwrap_or(0),
            is_file: parts[1].contains("regular file") || parts[1].contains("Regular File"),
            is_directory: parts[1].contains("directory") || parts[1].contains("Directory"),
            is_symlink: parts[1].contains("symbolic link") || parts[1].contains("Symbolic Link"),
            permissions: parts[2].to_string(),
            owner: parts[3].to_string(),
            group: parts[4].to_string(),
            modified: parts[5].to_string(),
        })
    }
}

/// Error from filesystem operations.
#[derive(Debug)]
pub struct FsError(pub String);

impl std::fmt::Display for FsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for FsError {}

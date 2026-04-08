use crate::openapi::Client;
use futures_core::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A file change event from the VM's watch-files endpoint.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct FileChangeEvent(pub serde_json::Value);

/// Error from the watch-files stream.
#[derive(Debug)]
pub enum WatchError {
    Request(reqwest::Error),
    Http(reqwest::StatusCode),
    Json(serde_json::Error),
}

impl std::fmt::Display for WatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(e) => write!(f, "watch request failed: {e}"),
            Self::Http(status) => write!(f, "watch returned {status}"),
            Self::Json(e) => write!(f, "watch JSON error: {e}"),
        }
    }
}

impl std::error::Error for WatchError {}

/// Async stream of file change events.
pub struct FileChangeStream {
    inner: Pin<Box<dyn Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send>>,
    buffer: String,
}

impl Stream for FileChangeStream {
    type Item = Result<FileChangeEvent, WatchError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            // Drain any complete lines from the buffer.
            if let Some(newline_idx) = self.buffer.find('\n') {
                let line = self.buffer[..newline_idx].trim().to_string();
                self.buffer = self.buffer[newline_idx + 1..].to_string();
                if !line.is_empty() {
                    return Poll::Ready(Some(
                        serde_json::from_str::<serde_json::Value>(&line)
                            .map(FileChangeEvent)
                            .map_err(WatchError::Json),
                    ));
                }
                continue;
            }

            // No complete line — poll for more data.
            match self.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    if let Ok(text) = std::str::from_utf8(&chunk) {
                        self.buffer.push_str(text);
                    }
                }
                Poll::Ready(Some(Err(e))) => {
                    return Poll::Ready(Some(Err(WatchError::Request(e))));
                }
                Poll::Ready(None) => {
                    // Stream ended — flush remaining buffer.
                    let remaining = std::mem::take(&mut self.buffer);
                    let trimmed = remaining.trim();
                    if !trimmed.is_empty() {
                        return Poll::Ready(Some(
                            serde_json::from_str::<serde_json::Value>(trimmed)
                                .map(FileChangeEvent)
                                .map_err(WatchError::Json),
                        ));
                    }
                    return Poll::Ready(None);
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

/// Start watching for file changes in a VM.
pub(crate) async fn watch_files(
    client: &Client,
    vm_id: &str,
    linux_username: Option<&str>,
) -> Result<FileChangeStream, WatchError> {
    let url = format!("{}/v1/vms/{vm_id}/watch-files", client.baseurl);
    let mut req = client.client.post(&url);
    if let Some(user) = linux_username {
        req = req.header("X-Freestyle-Vm-Linux-User-Id", user);
    }
    let response = req.send().await.map_err(WatchError::Request)?;
    if !response.status().is_success() {
        return Err(WatchError::Http(response.status()));
    }
    Ok(FileChangeStream {
        inner: Box::pin(response.bytes_stream()),
        buffer: String::new(),
    })
}

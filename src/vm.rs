use crate::openapi::{Client, Error, ResponseValue, types};

/// Handle for a specific VM, providing convenience methods.
#[derive(Clone)]
pub struct VmHandle {
    pub(crate) client: Client,
    pub(crate) vm_id: String,
    pub(crate) linux_username: Option<String>,
}

impl VmHandle {
    pub(crate) fn new(client: Client, vm_id: impl Into<String>) -> Self {
        Self {
            client,
            vm_id: vm_id.into(),
            linux_username: None,
        }
    }

    /// Return a new handle that executes commands as the given Linux user.
    pub fn user(&self, username: impl Into<String>) -> Self {
        Self {
            client: self.client.clone(),
            vm_id: self.vm_id.clone(),
            linux_username: Some(username.into()),
        }
    }

    /// Execute a command in the VM and wait for completion.
    pub async fn exec(
        &self,
        command: &str,
    ) -> Result<ResponseValue<types::ExecAwaitVmResponse>, Error<types::ExecAwaitResponse>> {
        let body = types::ExecAwaitRequest {
            command: command.to_string(),
            terminal: None,
            timeout_ms: None,
        };
        self.client.exec_await(&self.vm_id, &body).await
    }

    /// Execute a command with a timeout in milliseconds.
    pub async fn exec_with_timeout(
        &self,
        command: &str,
        timeout_ms: i64,
    ) -> Result<ResponseValue<types::ExecAwaitVmResponse>, Error<types::ExecAwaitResponse>> {
        let body = types::ExecAwaitRequest {
            command: command.to_string(),
            terminal: None,
            timeout_ms: Some(timeout_ms),
        };
        self.client.exec_await(&self.vm_id, &body).await
    }

    /// Get a filesystem handle for this VM.
    pub fn fs(&self) -> crate::FileSystem {
        crate::FileSystem::new(self.clone())
    }

    /// Get a systemd service handle for a specific service.
    pub fn systemd_service(&self, service_id: impl Into<String>) -> crate::SystemdServiceHandle {
        crate::SystemdServiceHandle::new(self.clone(), service_id.into())
    }

    /// Get VM info.
    pub async fn info(
        &self,
    ) -> Result<ResponseValue<types::GetVmResponse>, Error<types::GetVmResponse>> {
        self.client.get_vm(&self.vm_id).await
    }

    /// Start the VM.
    pub async fn start(&self) -> Result<ResponseValue<types::StartedVmResponse>, Error<()>> {
        let body = types::StartVmRequest {
            activity_threshold_bytes: None,
            idle_timeout_seconds: None,
            ready_signal_timeout_seconds: None,
            wait_for_ready_signal: None,
        };
        self.client.start_vm(&self.vm_id, &body).await
    }

    /// Stop the VM.
    pub async fn stop(
        &self,
    ) -> Result<ResponseValue<types::StopVmResponse>, Error<types::StopVmResponse>> {
        self.client.stop_vm(&self.vm_id).await
    }

    /// Kill the VM immediately.
    pub async fn kill(
        &self,
    ) -> Result<ResponseValue<types::KillVmResponse>, Error<types::KillVmResponse>> {
        self.client.kill_vm(&self.vm_id).await
    }

    /// Suspend the VM to disk.
    pub async fn suspend(
        &self,
    ) -> Result<ResponseValue<types::SuspendVmResponse>, Error<types::SuspendVmResponse>> {
        self.client.suspend_vm(&self.vm_id).await
    }

    /// Wait for the VM to exit.
    pub async fn wait_for_exit(
        &self,
    ) -> Result<ResponseValue<types::WaitVmResponse>, Error<types::WaitVmResponse>> {
        self.client.wait_vm(&self.vm_id).await
    }

    /// Create a snapshot of the VM.
    pub async fn snapshot(&self) -> Result<ResponseValue<types::SnapshotVmResponse>, Error<()>> {
        let body = types::SnapshotVmRequest {
            name: None,
            persistence: None,
        };
        self.client.snapshot_vm(&self.vm_id, &body).await
    }

    /// Fork (clone) the VM.
    pub async fn fork(
        &self,
    ) -> Result<ResponseValue<types::ForkVmResponse>, Error<types::ForkVmResponse>> {
        let body = types::ForkVmRequest { count: None };
        self.client.fork_vm(&self.vm_id, &body).await
    }

    /// Optimize the VM (compact memory/disk).
    pub async fn optimize(
        &self,
    ) -> Result<ResponseValue<types::OptimizeVmResponse>, Error<types::OptimizeVmResponse>> {
        self.client.optimize_vm(&self.vm_id).await
    }

    /// Delete this VM.
    pub async fn delete(
        &self,
    ) -> Result<ResponseValue<types::DeleteVmResponses>, Error<types::DeleteVmResponse>> {
        self.client.delete_vm(&self.vm_id).await
    }

    /// Watch for file changes. Returns an async stream of events.
    pub async fn watch_files(
        &self,
    ) -> Result<crate::watch::FileChangeStream, crate::watch::WatchError> {
        crate::watch::watch_files(&self.client, &self.vm_id, self.linux_username.as_deref()).await
    }
}

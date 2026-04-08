use crate::openapi::{Error, ResponseValue, types};
use crate::vm::VmHandle;

/// Handle for a specific systemd service in a VM.
pub struct SystemdServiceHandle {
    vm: VmHandle,
    service_id: String,
}

impl SystemdServiceHandle {
    pub(crate) fn new(vm: VmHandle, service_id: String) -> Self {
        Self { vm, service_id }
    }

    fn vm_id_str(&self) -> String {
        self.vm.vm_id.to_string()
    }

    fn batch_body(&self) -> types::BatchServiceRequest {
        types::BatchServiceRequest {
            services: vec![types::ServiceIdItem {
                id: self.service_id.clone(),
            }],
        }
    }

    /// Get the status of this service.
    pub async fn status(
        &self,
    ) -> Result<ResponseValue<types::SystemdServiceStatus>, Error<types::GetServiceStatusResponse>>
    {
        self.vm
            .client
            .get_service_status(&self.vm_id_str(), &self.service_id)
            .await
    }

    /// Get logs from this service.
    pub async fn logs(
        &self,
    ) -> Result<ResponseValue<types::JournaldLogsResponse>, Error<types::GetServiceLogsResponse>>
    {
        self.vm
            .client
            .get_service_logs(&self.vm_id_str(), &self.service_id, None, None)
            .await
    }

    /// Get logs with options.
    pub async fn logs_with_options(
        &self,
        lines: Option<i32>,
        since: Option<&str>,
    ) -> Result<ResponseValue<types::JournaldLogsResponse>, Error<types::GetServiceLogsResponse>>
    {
        self.vm
            .client
            .get_service_logs(&self.vm_id_str(), &self.service_id, lines, since)
            .await
    }

    /// Start this service.
    pub async fn start(
        &self,
    ) -> Result<ResponseValue<types::BatchServiceResponse>, Error<types::BatchStartServicesResponse>>
    {
        self.vm
            .client
            .batch_start_services(&self.vm_id_str(), &self.batch_body())
            .await
    }

    /// Stop this service.
    pub async fn stop(
        &self,
    ) -> Result<ResponseValue<types::BatchServiceResponse>, Error<types::BatchStopServicesResponse>>
    {
        self.vm
            .client
            .batch_stop_services(&self.vm_id_str(), &self.batch_body())
            .await
    }

    /// Restart this service.
    pub async fn restart(
        &self,
    ) -> Result<
        ResponseValue<types::BatchServiceResponse>,
        Error<types::BatchRestartServicesResponse>,
    > {
        self.vm
            .client
            .batch_restart_services(&self.vm_id_str(), &self.batch_body())
            .await
    }

    /// Delete this service.
    pub async fn delete(
        &self,
    ) -> Result<
        ResponseValue<types::SystemdDeleteServiceResponse>,
        Error<types::DeleteServiceResponse>,
    > {
        self.vm
            .client
            .delete_service(&self.vm_id_str(), &self.service_id)
            .await
    }
}

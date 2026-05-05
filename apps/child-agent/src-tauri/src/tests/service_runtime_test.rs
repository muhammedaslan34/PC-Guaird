use crate::ipc::{PipeRequest, PipeResponse, TrayPipe};
use crate::service::{ServiceConnectionState, ServiceRuntime};

#[test]
fn service_runtime_reports_degraded_and_disconnected_states() {
    let mut runtime = ServiceRuntime::new();
    runtime.set_status(ServiceConnectionState::Degraded);
    runtime.set_last_error("backend reconnect pending");

    let mut pipe = TrayPipe::connect(runtime);

    match pipe.request(PipeRequest::GetStatus) {
        PipeResponse::Status(snapshot) => {
            assert_eq!(snapshot.service_status, ServiceConnectionState::Degraded);
            assert_eq!(
                snapshot.last_error.as_deref(),
                Some("backend reconnect pending")
            );
        }
        other => panic!("expected status response, got {other:?}"),
    }

    let mut runtime = ServiceRuntime::new();
    runtime.set_status(ServiceConnectionState::Disconnected);
    let mut pipe = TrayPipe::connect(runtime);

    match pipe.request(PipeRequest::GetStatus) {
        PipeResponse::Status(snapshot) => {
            assert_eq!(
                snapshot.service_status,
                ServiceConnectionState::Disconnected
            );
        }
        other => panic!("expected status response, got {other:?}"),
    }
}

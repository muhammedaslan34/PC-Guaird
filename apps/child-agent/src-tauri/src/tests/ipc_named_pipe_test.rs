use crate::ipc::{PipeRequest, PipeResponse, TrayPipe};
use crate::service::{ServiceConnectionState, ServiceRuntime};

#[test]
fn tray_pipe_can_read_and_update_service_state() {
    let runtime = ServiceRuntime::new();
    let mut pipe = TrayPipe::connect(runtime);

    match pipe.request(PipeRequest::GetStatus) {
        PipeResponse::Status(snapshot) => {
            assert_eq!(snapshot.service_status, ServiceConnectionState::Starting);
            assert!(snapshot.pairing_code.is_none());
        }
        other => panic!("expected status response, got {other:?}"),
    }

    assert_eq!(
        pipe.request(PipeRequest::SetPairingCode("PAIR-1234".to_string())),
        PipeResponse::Acknowledged
    );

    assert_eq!(
        pipe.request(PipeRequest::ReportDiagnostic(
            "tray connected successfully".to_string()
        )),
        PipeResponse::Acknowledged
    );

    match pipe.request(PipeRequest::GetPairingCode) {
        PipeResponse::PairingCode(pairing_code) => {
            assert_eq!(pairing_code.as_deref(), Some("PAIR-1234"));
        }
        other => panic!("expected pairing code response, got {other:?}"),
    }
}

use crate::ipc::TrayPipe;
use crate::service::{ServiceConnectionState, ServiceRuntime};
use crate::tray::{TrayConnectionLabel, TrayStatusReader};

#[test]
fn tray_status_reader_shows_starting_state_when_service_not_yet_connected() {
    let runtime = ServiceRuntime::new();
    let pipe = TrayPipe::connect(runtime);
    let mut reader = TrayStatusReader::new(pipe);

    let status = reader.read();

    assert_eq!(status.connection_label, TrayConnectionLabel::Starting);
    assert!(status.pairing_code.is_none());
    assert_eq!(status.connection_display(), "Starting\u{2026}");
}

#[test]
fn tray_status_reader_shows_pairing_code_when_set() {
    let mut runtime = ServiceRuntime::new();
    runtime.set_pairing_code("AB12CD");
    let pipe = TrayPipe::connect(runtime);
    let mut reader = TrayStatusReader::new(pipe);

    let status = reader.read();

    assert_eq!(status.pairing_code.as_deref(), Some("AB12CD"));
    assert_eq!(status.pairing_code_display(), "AB12CD");
}

#[test]
fn tray_status_reader_shows_connected_state() {
    let mut runtime = ServiceRuntime::new();
    runtime.set_status(ServiceConnectionState::Connected);
    let pipe = TrayPipe::connect(runtime);
    let mut reader = TrayStatusReader::new(pipe);

    let status = reader.read();

    assert_eq!(status.connection_label, TrayConnectionLabel::Connected);
    assert_eq!(status.connection_display(), "Connected");
}

#[test]
fn tray_status_reader_shows_degraded_state() {
    let mut runtime = ServiceRuntime::new();
    runtime.set_status(ServiceConnectionState::Degraded);
    let pipe = TrayPipe::connect(runtime);
    let mut reader = TrayStatusReader::new(pipe);

    let status = reader.read();

    assert_eq!(status.connection_label, TrayConnectionLabel::Degraded);
    assert_eq!(status.connection_display(), "Connection degraded");
}

#[test]
fn tray_status_reader_shows_error_message_when_present() {
    let mut runtime = ServiceRuntime::new();
    runtime.set_last_error("websocket handshake failed");
    let pipe = TrayPipe::connect(runtime);
    let mut reader = TrayStatusReader::new(pipe);

    let status = reader.read();

    assert_eq!(
        status.error_message.as_deref(),
        Some("websocket handshake failed")
    );
}

#[test]
fn tray_status_no_pairing_code_displays_dash() {
    let runtime = ServiceRuntime::new();
    let pipe = TrayPipe::connect(runtime);
    let mut reader = TrayStatusReader::new(pipe);

    let status = reader.read();

    assert_eq!(status.pairing_code_display(), "\u{2014}");
}

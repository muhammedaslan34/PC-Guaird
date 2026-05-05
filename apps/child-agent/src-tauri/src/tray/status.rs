#![allow(dead_code)]

use crate::ipc::{PipeRequest, PipeResponse, TrayPipe};
use crate::service::ServiceConnectionState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrayConnectionLabel {
    Starting,
    Connected,
    Degraded,
    Disconnected,
    ServiceUnavailable,
}

impl std::fmt::Display for TrayConnectionLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            TrayConnectionLabel::Starting => "Starting\u{2026}",
            TrayConnectionLabel::Connected => "Connected",
            TrayConnectionLabel::Degraded => "Connection degraded",
            TrayConnectionLabel::Disconnected => "Disconnected",
            TrayConnectionLabel::ServiceUnavailable => "Service unavailable",
        };
        f.write_str(label)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrayStatus {
    pub connection_label: TrayConnectionLabel,
    pub pairing_code: Option<String>,
    pub error_message: Option<String>,
}

impl TrayStatus {
    pub fn pairing_code_display(&self) -> String {
        match &self.pairing_code {
            Some(code) => code.clone(),
            None => "\u{2014}".to_string(),
        }
    }

    pub fn connection_display(&self) -> String {
        self.connection_label.to_string()
    }
}

pub struct TrayStatusReader {
    pipe: TrayPipe,
}

impl TrayStatusReader {
    pub fn new(pipe: TrayPipe) -> Self {
        Self { pipe }
    }

    pub fn read(&mut self) -> TrayStatus {
        match self.pipe.request(PipeRequest::GetStatus) {
            PipeResponse::Status(snapshot) => {
                let connection_label = match snapshot.service_status {
                    ServiceConnectionState::Starting => TrayConnectionLabel::Starting,
                    ServiceConnectionState::Connected => TrayConnectionLabel::Connected,
                    ServiceConnectionState::Degraded => TrayConnectionLabel::Degraded,
                    ServiceConnectionState::Disconnected => TrayConnectionLabel::Disconnected,
                };
                TrayStatus {
                    connection_label,
                    pairing_code: snapshot.pairing_code,
                    error_message: snapshot.last_error,
                }
            }
            _ => TrayStatus {
                connection_label: TrayConnectionLabel::ServiceUnavailable,
                pairing_code: None,
                error_message: Some("Could not reach service".to_string()),
            },
        }
    }
}

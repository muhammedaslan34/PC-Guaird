#![allow(dead_code)]

use crate::service::{ServiceConnectionState, ServiceRuntime, ServiceSnapshot};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PipeRequest {
    GetStatus,
    GetPairingCode,
    SetPairingCode(String),
    ReportDiagnostic(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PipeResponse {
    Status(ServiceSnapshot),
    PairingCode(Option<String>),
    Acknowledged,
}

#[derive(Clone, Debug)]
pub struct TrayPipe {
    runtime: ServiceRuntime,
}

impl TrayPipe {
    pub fn connect(runtime: ServiceRuntime) -> Self {
        Self { runtime }
    }

    pub fn request(&mut self, request: PipeRequest) -> PipeResponse {
        match request {
            PipeRequest::GetStatus => PipeResponse::Status(self.runtime.snapshot()),
            PipeRequest::GetPairingCode => {
                PipeResponse::PairingCode(self.runtime.snapshot().pairing_code)
            }
            PipeRequest::SetPairingCode(pairing_code) => {
                self.runtime.set_pairing_code(pairing_code);
                self.runtime.set_status(ServiceConnectionState::Connected);
                PipeResponse::Acknowledged
            }
            PipeRequest::ReportDiagnostic(_) => PipeResponse::Acknowledged,
        }
    }
}

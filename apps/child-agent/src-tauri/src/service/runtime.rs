#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ServiceConnectionState {
    Starting,
    Connected,
    Degraded,
    Disconnected,
}

impl std::fmt::Display for ServiceConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            ServiceConnectionState::Starting => "starting",
            ServiceConnectionState::Connected => "connected",
            ServiceConnectionState::Degraded => "degraded",
            ServiceConnectionState::Disconnected => "disconnected",
        };

        f.write_str(label)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceSnapshot {
    pub service_status: ServiceConnectionState,
    pub pairing_code: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ServiceRuntime {
    snapshot: ServiceSnapshot,
}

impl ServiceRuntime {
    pub fn new() -> Self {
        Self {
            snapshot: ServiceSnapshot {
                service_status: ServiceConnectionState::Starting,
                pairing_code: None,
                last_error: None,
            },
        }
    }

    pub fn snapshot(&self) -> ServiceSnapshot {
        self.snapshot.clone()
    }

    pub fn set_pairing_code(&mut self, pairing_code: impl Into<String>) {
        self.snapshot.pairing_code = Some(pairing_code.into());
    }

    pub fn set_status(&mut self, service_status: ServiceConnectionState) {
        self.snapshot.service_status = service_status;
    }

    pub fn set_last_error(&mut self, last_error: impl Into<String>) {
        self.snapshot.last_error = Some(last_error.into());
    }
}

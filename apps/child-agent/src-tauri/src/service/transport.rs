#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransportState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Degraded,
}

#[derive(Clone, Debug)]
pub enum TransportEvent {
    ConnectRequested,
    WebSocketConnected,
    WebSocketFailed(String),
    WebSocketDisconnected,
    PollSucceeded,
    PollFailed(String),
    ReconnectTimerFired,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransportAction {
    OpenWebSocket,
    StartPolling,
    StopPolling,
    ScheduleReconnect { after_secs: u64 },
}

// Backoff schedule in seconds, indexed by attempt number (clamped at last entry).
const BACKOFF_SECS: &[u64] = &[1, 2, 5, 10, 30];

pub struct TransportMachine {
    state: TransportState,
    reconnect_attempt: u32,
    polling_active: bool,
}

impl TransportMachine {
    pub const MAX_RECONNECT_ATTEMPTS: u32 = BACKOFF_SECS.len() as u32;

    pub fn new() -> Self {
        Self {
            state: TransportState::Disconnected,
            reconnect_attempt: 0,
            polling_active: false,
        }
    }

    pub fn state(&self) -> &TransportState {
        &self.state
    }

    pub fn reconnect_attempt(&self) -> u32 {
        self.reconnect_attempt
    }

    pub fn is_using_fallback(&self) -> bool {
        self.polling_active
    }

    pub fn backoff_secs(attempt: u32) -> u64 {
        let idx = (attempt as usize).min(BACKOFF_SECS.len() - 1);
        BACKOFF_SECS[idx]
    }

    pub fn handle(&mut self, event: TransportEvent) -> Vec<TransportAction> {
        match (&self.state, event) {
            // ── connect request ───────────────────────────────────────────
            (TransportState::Disconnected, TransportEvent::ConnectRequested)
            | (TransportState::Degraded, TransportEvent::ConnectRequested) => {
                self.state = TransportState::Connecting;
                self.reconnect_attempt = 0;
                let mut actions = vec![TransportAction::OpenWebSocket];
                if self.polling_active {
                    actions.push(TransportAction::StopPolling);
                    self.polling_active = false;
                }
                actions
            }

            // ── websocket established ─────────────────────────────────────
            (TransportState::Connecting, TransportEvent::WebSocketConnected) => {
                self.state = TransportState::Connected;
                self.reconnect_attempt = 0;
                self.polling_active = false;
                vec![TransportAction::StopPolling]
            }

            // ── websocket failed while connecting ─────────────────────────
            (TransportState::Connecting, TransportEvent::WebSocketFailed(_)) => {
                self.reconnect_attempt += 1;
                if self.reconnect_attempt >= Self::MAX_RECONNECT_ATTEMPTS {
                    self.state = TransportState::Degraded;
                    let mut actions = vec![];
                    if !self.polling_active {
                        self.polling_active = true;
                        actions.push(TransportAction::StartPolling);
                    }
                    actions
                } else {
                    self.state = TransportState::Reconnecting;
                    self.polling_active = true;
                    vec![
                        TransportAction::StartPolling,
                        TransportAction::ScheduleReconnect {
                            after_secs: Self::backoff_secs(self.reconnect_attempt),
                        },
                    ]
                }
            }

            // ── live connection dropped ───────────────────────────────────
            (TransportState::Connected, TransportEvent::WebSocketDisconnected)
            | (TransportState::Connected, TransportEvent::WebSocketFailed(_)) => {
                self.state = TransportState::Reconnecting;
                self.reconnect_attempt = 0;
                self.polling_active = true;
                vec![
                    TransportAction::StartPolling,
                    TransportAction::ScheduleReconnect {
                        after_secs: Self::backoff_secs(0),
                    },
                ]
            }

            // ── reconnect timer elapsed ───────────────────────────────────
            (TransportState::Reconnecting, TransportEvent::ReconnectTimerFired) => {
                self.state = TransportState::Connecting;
                vec![TransportAction::OpenWebSocket]
            }

            // ── polling events while degraded/reconnecting ─────────────────
            (TransportState::Degraded, TransportEvent::PollSucceeded)
            | (TransportState::Reconnecting, TransportEvent::PollSucceeded)
            | (TransportState::Degraded, TransportEvent::PollFailed(_))
            | (TransportState::Reconnecting, TransportEvent::PollFailed(_)) => {
                // state unchanged; caller records errors elsewhere
                vec![]
            }

            // ── ignore irrelevant events ──────────────────────────────────
            _ => vec![],
        }
    }
}

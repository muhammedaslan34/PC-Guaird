use crate::service::transport::{TransportAction, TransportEvent, TransportMachine, TransportState};

fn has_action(actions: &[TransportAction], target: &TransportAction) -> bool {
    actions.iter().any(|a| a == target)
}

#[test]
fn initial_state_is_disconnected() {
    let m = TransportMachine::new();
    assert_eq!(*m.state(), TransportState::Disconnected);
    assert_eq!(m.reconnect_attempt(), 0);
    assert!(!m.is_using_fallback());
}

#[test]
fn connect_requested_transitions_to_connecting_with_open_websocket_action() {
    let mut m = TransportMachine::new();
    let actions = m.handle(TransportEvent::ConnectRequested);
    assert_eq!(*m.state(), TransportState::Connecting);
    assert!(has_action(&actions, &TransportAction::OpenWebSocket));
}

#[test]
fn websocket_connected_transitions_to_connected_and_stops_polling() {
    let mut m = TransportMachine::new();
    m.handle(TransportEvent::ConnectRequested);
    let actions = m.handle(TransportEvent::WebSocketConnected);
    assert_eq!(*m.state(), TransportState::Connected);
    assert!(has_action(&actions, &TransportAction::StopPolling));
    assert!(!m.is_using_fallback());
}

#[test]
fn websocket_failed_from_connecting_starts_polling_and_schedules_reconnect() {
    let mut m = TransportMachine::new();
    m.handle(TransportEvent::ConnectRequested);
    let actions = m.handle(TransportEvent::WebSocketFailed("timeout".to_string()));
    assert_eq!(*m.state(), TransportState::Reconnecting);
    assert!(has_action(&actions, &TransportAction::StartPolling));
    assert!(actions
        .iter()
        .any(|a| matches!(a, TransportAction::ScheduleReconnect { .. })));
    assert!(m.is_using_fallback());
}

#[test]
fn websocket_disconnected_from_connected_starts_polling_and_schedules_reconnect() {
    let mut m = TransportMachine::new();
    m.handle(TransportEvent::ConnectRequested);
    m.handle(TransportEvent::WebSocketConnected);
    let actions = m.handle(TransportEvent::WebSocketDisconnected);
    assert_eq!(*m.state(), TransportState::Reconnecting);
    assert!(has_action(&actions, &TransportAction::StartPolling));
    assert!(actions
        .iter()
        .any(|a| matches!(a, TransportAction::ScheduleReconnect { .. })));
}

#[test]
fn reconnect_timer_fired_transitions_back_to_connecting() {
    let mut m = TransportMachine::new();
    m.handle(TransportEvent::ConnectRequested);
    m.handle(TransportEvent::WebSocketFailed("err".to_string()));
    let actions = m.handle(TransportEvent::ReconnectTimerFired);
    assert_eq!(*m.state(), TransportState::Connecting);
    assert!(has_action(&actions, &TransportAction::OpenWebSocket));
}

#[test]
fn reconnect_attempt_increments_on_each_websocket_failure() {
    let mut m = TransportMachine::new();
    m.handle(TransportEvent::ConnectRequested);
    m.handle(TransportEvent::WebSocketFailed("e".to_string()));
    assert_eq!(m.reconnect_attempt(), 1);
    m.handle(TransportEvent::ReconnectTimerFired);
    m.handle(TransportEvent::WebSocketFailed("e".to_string()));
    assert_eq!(m.reconnect_attempt(), 2);
}

#[test]
fn websocket_connected_after_reconnect_resets_attempt_count() {
    let mut m = TransportMachine::new();
    m.handle(TransportEvent::ConnectRequested);
    m.handle(TransportEvent::WebSocketFailed("e".to_string()));
    m.handle(TransportEvent::ReconnectTimerFired);
    m.handle(TransportEvent::WebSocketConnected);
    assert_eq!(m.reconnect_attempt(), 0);
    assert!(!m.is_using_fallback());
}

#[test]
fn repeated_failures_eventually_transition_to_degraded() {
    let mut m = TransportMachine::new();
    m.handle(TransportEvent::ConnectRequested);
    // Exhaust all reconnect attempts
    for _ in 0..TransportMachine::MAX_RECONNECT_ATTEMPTS {
        m.handle(TransportEvent::WebSocketFailed("e".to_string()));
        if *m.state() == TransportState::Degraded {
            break;
        }
        m.handle(TransportEvent::ReconnectTimerFired);
    }
    assert_eq!(*m.state(), TransportState::Degraded);
}

#[test]
fn degraded_connect_requested_retries_websocket() {
    let mut m = TransportMachine::new();
    // Drive into degraded
    m.handle(TransportEvent::ConnectRequested);
    for _ in 0..TransportMachine::MAX_RECONNECT_ATTEMPTS {
        m.handle(TransportEvent::WebSocketFailed("e".to_string()));
        if *m.state() == TransportState::Degraded {
            break;
        }
        m.handle(TransportEvent::ReconnectTimerFired);
    }
    let actions = m.handle(TransportEvent::ConnectRequested);
    assert_eq!(*m.state(), TransportState::Connecting);
    assert!(has_action(&actions, &TransportAction::OpenWebSocket));
    assert_eq!(m.reconnect_attempt(), 0);
}

#[test]
fn backoff_increases_with_each_attempt() {
    let a0 = TransportMachine::backoff_secs(0);
    let a1 = TransportMachine::backoff_secs(1);
    let a2 = TransportMachine::backoff_secs(2);
    assert!(a1 > a0);
    assert!(a2 > a1);
}

#[test]
fn poll_succeeded_from_degraded_keeps_degraded_state() {
    let mut m = TransportMachine::new();
    m.handle(TransportEvent::ConnectRequested);
    for _ in 0..TransportMachine::MAX_RECONNECT_ATTEMPTS {
        m.handle(TransportEvent::WebSocketFailed("e".to_string()));
        if *m.state() == TransportState::Degraded {
            break;
        }
        m.handle(TransportEvent::ReconnectTimerFired);
    }
    m.handle(TransportEvent::PollSucceeded);
    assert_eq!(*m.state(), TransportState::Degraded);
    assert!(m.is_using_fallback());
}

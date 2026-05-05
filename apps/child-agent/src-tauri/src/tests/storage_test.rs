use crate::storage::{
    diagnostics::DiagnosticsLog,
    retry_queue::{CommandStatus, NewQueuedCommand, RetryQueue},
    sqlite::{Database, ServiceStateRecord},
};

// ── schema ────────────────────────────────────────────────────────────────────

#[test]
fn schema_creates_all_required_tables() {
    let db = Database::open_in_memory().expect("open in-memory db");
    let tables: Vec<String> = db
        .conn()
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .unwrap()
        .query_map([], |r| r.get::<_, String>(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert!(tables.contains(&"command_queue".to_string()));
    assert!(tables.contains(&"diagnostic_events".to_string()));
    assert!(tables.contains(&"pairing_cache".to_string()));
    assert!(tables.contains(&"service_state".to_string()));
}

// ── service_state ─────────────────────────────────────────────────────────────

#[test]
fn service_state_upsert_then_load_round_trips() {
    let db = Database::open_in_memory().unwrap();
    let record = ServiceStateRecord {
        device_uuid: "uuid-abc".to_string(),
        paired_device_id: None,
        service_status: "starting".to_string(),
        last_connected_at: None,
        last_heartbeat_at: None,
        last_error_code: None,
    };
    db.upsert_service_state(&record).unwrap();
    let loaded = db.load_service_state().unwrap().expect("row should exist");
    assert_eq!(loaded.device_uuid, "uuid-abc");
    assert_eq!(loaded.service_status, "starting");
    assert!(loaded.paired_device_id.is_none());
}

#[test]
fn service_state_upsert_overwrites_previous_row() {
    let db = Database::open_in_memory().unwrap();
    db.upsert_service_state(&ServiceStateRecord {
        device_uuid: "uuid-abc".to_string(),
        paired_device_id: None,
        service_status: "starting".to_string(),
        last_connected_at: None,
        last_heartbeat_at: None,
        last_error_code: None,
    })
    .unwrap();
    db.upsert_service_state(&ServiceStateRecord {
        device_uuid: "uuid-abc".to_string(),
        paired_device_id: Some("device-42".to_string()),
        service_status: "connected".to_string(),
        last_connected_at: Some(1_000_000),
        last_heartbeat_at: Some(1_000_001),
        last_error_code: None,
    })
    .unwrap();
    let loaded = db.load_service_state().unwrap().unwrap();
    assert_eq!(loaded.service_status, "connected");
    assert_eq!(loaded.paired_device_id.as_deref(), Some("device-42"));
}

// ── pairing_cache ─────────────────────────────────────────────────────────────

#[test]
fn pairing_cache_stores_and_retrieves_active_code() {
    let db = Database::open_in_memory().unwrap();
    let future_ts = 9_999_999_999_i64;
    db.cache_pairing_code("uuid-abc", "AB12CD", future_ts)
        .unwrap();
    let entry = db
        .load_active_pairing_code("uuid-abc")
        .unwrap()
        .expect("entry should exist");
    assert_eq!(entry.pairing_code, "AB12CD");
    assert!(entry.used_at.is_none());
}

#[test]
fn pairing_cache_expired_code_is_not_returned() {
    let db = Database::open_in_memory().unwrap();
    let past_ts = 1_i64; // epoch + 1s — safely in the past
    db.cache_pairing_code("uuid-abc", "OLDCOD", past_ts)
        .unwrap();
    let entry = db.load_active_pairing_code("uuid-abc").unwrap();
    assert!(entry.is_none(), "expired code should not be returned");
}

#[test]
fn pairing_cache_mark_used_hides_code_from_active_query() {
    let db = Database::open_in_memory().unwrap();
    let future_ts = 9_999_999_999_i64;
    db.cache_pairing_code("uuid-abc", "AB12CD", future_ts)
        .unwrap();
    let entry = db.load_active_pairing_code("uuid-abc").unwrap().unwrap();
    db.mark_pairing_code_used(entry.id).unwrap();
    let after = db.load_active_pairing_code("uuid-abc").unwrap();
    assert!(after.is_none());
}

// ── retry_queue ───────────────────────────────────────────────────────────────

#[test]
fn retry_queue_enqueue_adds_command_with_pending_status() {
    let db = Database::open_in_memory().unwrap();
    let q = RetryQueue::new(&db);
    let id = q
        .enqueue(NewQueuedCommand {
            remote_command_id: Some("cmd-1".to_string()),
            device_id: "dev-1".to_string(),
            command_type: "lock".to_string(),
            payload: "{}".to_string(),
        })
        .unwrap();
    assert!(id > 0);
    let pending = q.peek_pending().unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].command_type, "lock");
    assert_eq!(pending[0].status, CommandStatus::Pending);
    assert_eq!(pending[0].attempt_count, 0);
}

#[test]
fn retry_queue_peek_pending_excludes_non_pending_commands() {
    let db = Database::open_in_memory().unwrap();
    let q = RetryQueue::new(&db);
    let id = q
        .enqueue(NewQueuedCommand {
            remote_command_id: None,
            device_id: "dev-1".to_string(),
            command_type: "shutdown".to_string(),
            payload: "{}".to_string(),
        })
        .unwrap();
    q.mark_succeeded(id).unwrap();
    let pending = q.peek_pending().unwrap();
    assert!(pending.is_empty());
}

#[test]
fn retry_queue_mark_failed_stores_error_and_increments_attempt() {
    let db = Database::open_in_memory().unwrap();
    let q = RetryQueue::new(&db);
    let id = q
        .enqueue(NewQueuedCommand {
            remote_command_id: None,
            device_id: "dev-1".to_string(),
            command_type: "restart".to_string(),
            payload: "{}".to_string(),
        })
        .unwrap();
    q.mark_failed(id, "exec error").unwrap();
    let cmd = q.get(id).unwrap().expect("row should exist");
    assert_eq!(cmd.status, CommandStatus::Failed);
    assert_eq!(cmd.last_error_message.as_deref(), Some("exec error"));
    assert_eq!(cmd.attempt_count, 1);
}

#[test]
fn retry_queue_mark_retrying_sets_next_retry_at() {
    let db = Database::open_in_memory().unwrap();
    let q = RetryQueue::new(&db);
    let id = q
        .enqueue(NewQueuedCommand {
            remote_command_id: None,
            device_id: "dev-1".to_string(),
            command_type: "lock".to_string(),
            payload: "{}".to_string(),
        })
        .unwrap();
    q.mark_retrying(id, 1_700_000_000).unwrap();
    let cmd = q.get(id).unwrap().unwrap();
    assert_eq!(cmd.status, CommandStatus::Retrying);
    assert_eq!(cmd.next_retry_at, Some(1_700_000_000));
}

// ── diagnostics ───────────────────────────────────────────────────────────────

#[test]
fn diagnostics_record_stores_event_retrievable_via_recent() {
    let db = Database::open_in_memory().unwrap();
    let log = DiagnosticsLog::new(&db);
    log.record("connection_lost", r#"{"reason":"timeout"}"#)
        .unwrap();
    let events = log.recent(10).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event_type, "connection_lost");
    assert_eq!(events[0].event_payload, r#"{"reason":"timeout"}"#);
}

#[test]
fn diagnostics_recent_returns_newest_first() {
    let db = Database::open_in_memory().unwrap();
    let log = DiagnosticsLog::new(&db);
    log.record("first", "a").unwrap();
    log.record("second", "b").unwrap();
    log.record("third", "c").unwrap();
    let events = log.recent(10).unwrap();
    assert_eq!(events[0].event_type, "third");
    assert_eq!(events[2].event_type, "first");
}

#[test]
fn diagnostics_recent_respects_limit() {
    let db = Database::open_in_memory().unwrap();
    let log = DiagnosticsLog::new(&db);
    for i in 0..5 {
        log.record("evt", &i.to_string()).unwrap();
    }
    let events = log.recent(3).unwrap();
    assert_eq!(events.len(), 3);
}

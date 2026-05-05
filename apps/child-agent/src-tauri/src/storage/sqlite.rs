#![allow(dead_code)]

use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.initialize_schema()?;
        Ok(db)
    }

    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.initialize_schema()?;
        Ok(db)
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    fn initialize_schema(&self) -> Result<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS service_state (
                id                INTEGER PRIMARY KEY,
                device_uuid       TEXT NOT NULL,
                paired_device_id  TEXT,
                service_status    TEXT NOT NULL,
                last_connected_at INTEGER,
                last_heartbeat_at INTEGER,
                last_error_code   TEXT,
                created_at        INTEGER NOT NULL DEFAULT (unixepoch()),
                updated_at        INTEGER NOT NULL DEFAULT (unixepoch())
            );

            CREATE TABLE IF NOT EXISTS pairing_cache (
                id           INTEGER PRIMARY KEY AUTOINCREMENT,
                device_uuid  TEXT NOT NULL,
                pairing_code TEXT NOT NULL,
                expires_at   INTEGER NOT NULL,
                used_at      INTEGER,
                created_at   INTEGER NOT NULL DEFAULT (unixepoch())
            );

            CREATE TABLE IF NOT EXISTS command_queue (
                id                  INTEGER PRIMARY KEY AUTOINCREMENT,
                remote_command_id   TEXT,
                device_id           TEXT NOT NULL,
                command_type        TEXT NOT NULL,
                payload             TEXT NOT NULL DEFAULT '{}',
                status              TEXT NOT NULL DEFAULT 'pending',
                attempt_count       INTEGER NOT NULL DEFAULT 0,
                next_retry_at       INTEGER,
                last_error_message  TEXT,
                created_at          INTEGER NOT NULL DEFAULT (unixepoch()),
                updated_at          INTEGER NOT NULL DEFAULT (unixepoch())
            );

            CREATE TABLE IF NOT EXISTS diagnostic_events (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                event_type    TEXT NOT NULL,
                event_payload TEXT NOT NULL DEFAULT '{}',
                created_at    INTEGER NOT NULL DEFAULT (unixepoch())
            );
        ")
    }
}

// ── service_state ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceStateRecord {
    pub device_uuid: String,
    pub paired_device_id: Option<String>,
    pub service_status: String,
    pub last_connected_at: Option<i64>,
    pub last_heartbeat_at: Option<i64>,
    pub last_error_code: Option<String>,
}

impl Database {
    pub fn upsert_service_state(&self, record: &ServiceStateRecord) -> Result<()> {
        self.conn.execute(
            "INSERT INTO service_state (
                 id, device_uuid, paired_device_id, service_status,
                 last_connected_at, last_heartbeat_at, last_error_code,
                 updated_at
             ) VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, unixepoch())
             ON CONFLICT(id) DO UPDATE SET
                 device_uuid       = excluded.device_uuid,
                 paired_device_id  = excluded.paired_device_id,
                 service_status    = excluded.service_status,
                 last_connected_at = excluded.last_connected_at,
                 last_heartbeat_at = excluded.last_heartbeat_at,
                 last_error_code   = excluded.last_error_code,
                 updated_at        = excluded.updated_at",
            rusqlite::params![
                record.device_uuid,
                record.paired_device_id,
                record.service_status,
                record.last_connected_at,
                record.last_heartbeat_at,
                record.last_error_code,
            ],
        )?;
        Ok(())
    }

    pub fn load_service_state(&self) -> Result<Option<ServiceStateRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT device_uuid, paired_device_id, service_status,
                    last_connected_at, last_heartbeat_at, last_error_code
             FROM service_state WHERE id = 1",
        )?;
        let mut rows = stmt.query_map([], |row| {
            Ok(ServiceStateRecord {
                device_uuid: row.get(0)?,
                paired_device_id: row.get(1)?,
                service_status: row.get(2)?,
                last_connected_at: row.get(3)?,
                last_heartbeat_at: row.get(4)?,
                last_error_code: row.get(5)?,
            })
        })?;
        rows.next().transpose()
    }
}

// ── pairing_cache ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PairingCacheEntry {
    pub id: i64,
    pub device_uuid: String,
    pub pairing_code: String,
    pub expires_at: i64,
    pub used_at: Option<i64>,
}

impl Database {
    pub fn cache_pairing_code(
        &self,
        device_uuid: &str,
        pairing_code: &str,
        expires_at: i64,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO pairing_cache (device_uuid, pairing_code, expires_at)
             VALUES (?1, ?2, ?3)",
            rusqlite::params![device_uuid, pairing_code, expires_at],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn load_active_pairing_code(
        &self,
        device_uuid: &str,
    ) -> Result<Option<PairingCacheEntry>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let mut stmt = self.conn.prepare(
            "SELECT id, device_uuid, pairing_code, expires_at, used_at
             FROM pairing_cache
             WHERE device_uuid = ?1 AND expires_at > ?2 AND used_at IS NULL
             ORDER BY id DESC
             LIMIT 1",
        )?;
        let mut rows = stmt.query_map(rusqlite::params![device_uuid, now], |row| {
            Ok(PairingCacheEntry {
                id: row.get(0)?,
                device_uuid: row.get(1)?,
                pairing_code: row.get(2)?,
                expires_at: row.get(3)?,
                used_at: row.get(4)?,
            })
        })?;
        rows.next().transpose()
    }

    pub fn mark_pairing_code_used(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE pairing_cache SET used_at = unixepoch() WHERE id = ?1",
            rusqlite::params![id],
        )?;
        Ok(())
    }
}

#![allow(dead_code)]

use crate::storage::sqlite::Database;
use rusqlite::Result;

#[derive(Clone, Debug)]
pub struct DiagnosticEvent {
    pub id: i64,
    pub event_type: String,
    pub event_payload: String,
    pub created_at: i64,
}

pub struct DiagnosticsLog<'a> {
    db: &'a Database,
}

impl<'a> DiagnosticsLog<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn record(&self, event_type: &str, payload: &str) -> Result<()> {
        self.db.conn().execute(
            "INSERT INTO diagnostic_events (event_type, event_payload)
             VALUES (?1, ?2)",
            rusqlite::params![event_type, payload],
        )?;
        Ok(())
    }

    pub fn recent(&self, limit: usize) -> Result<Vec<DiagnosticEvent>> {
        let mut stmt = self.db.conn().prepare(
            "SELECT id, event_type, event_payload, created_at
             FROM diagnostic_events
             ORDER BY id DESC
             LIMIT ?1",
        )?;
        let rows: Result<Vec<DiagnosticEvent>> =
            stmt.query_map(rusqlite::params![limit as i64], |row| {
                Ok(DiagnosticEvent {
                    id: row.get(0)?,
                    event_type: row.get(1)?,
                    event_payload: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })?
            .collect();
        rows
    }
}

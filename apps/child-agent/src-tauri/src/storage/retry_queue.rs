#![allow(dead_code)]

use crate::storage::sqlite::Database;
use rusqlite::Result;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandStatus {
    Pending,
    Dispatched,
    Acknowledged,
    Succeeded,
    Failed,
    Retrying,
    Rejected,
}

impl CommandStatus {
    fn as_str(&self) -> &'static str {
        match self {
            CommandStatus::Pending => "pending",
            CommandStatus::Dispatched => "dispatched",
            CommandStatus::Acknowledged => "acknowledged",
            CommandStatus::Succeeded => "succeeded",
            CommandStatus::Failed => "failed",
            CommandStatus::Retrying => "retrying",
            CommandStatus::Rejected => "rejected",
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "dispatched" => CommandStatus::Dispatched,
            "acknowledged" => CommandStatus::Acknowledged,
            "succeeded" => CommandStatus::Succeeded,
            "failed" => CommandStatus::Failed,
            "retrying" => CommandStatus::Retrying,
            "rejected" => CommandStatus::Rejected,
            _ => CommandStatus::Pending,
        }
    }
}

pub struct NewQueuedCommand {
    pub remote_command_id: Option<String>,
    pub device_id: String,
    pub command_type: String,
    pub payload: String,
}

#[derive(Clone, Debug)]
pub struct QueuedCommand {
    pub id: i64,
    pub remote_command_id: Option<String>,
    pub device_id: String,
    pub command_type: String,
    pub payload: String,
    pub status: CommandStatus,
    pub attempt_count: i32,
    pub next_retry_at: Option<i64>,
    pub last_error_message: Option<String>,
}

pub struct RetryQueue<'a> {
    db: &'a Database,
}

impl<'a> RetryQueue<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn enqueue(&self, cmd: NewQueuedCommand) -> Result<i64> {
        self.db.conn().execute(
            "INSERT INTO command_queue
                 (remote_command_id, device_id, command_type, payload, status)
             VALUES (?1, ?2, ?3, ?4, 'pending')",
            rusqlite::params![
                cmd.remote_command_id,
                cmd.device_id,
                cmd.command_type,
                cmd.payload,
            ],
        )?;
        Ok(self.db.conn().last_insert_rowid())
    }

    pub fn peek_pending(&self) -> Result<Vec<QueuedCommand>> {
        self.query_by_status("pending")
    }

    pub fn get(&self, id: i64) -> Result<Option<QueuedCommand>> {
        let mut stmt = self.db.conn().prepare(
            "SELECT id, remote_command_id, device_id, command_type, payload,
                    status, attempt_count, next_retry_at, last_error_message
             FROM command_queue WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(rusqlite::params![id], Self::map_row)?;
        rows.next().transpose()
    }

    pub fn mark_succeeded(&self, id: i64) -> Result<()> {
        self.db.conn().execute(
            "UPDATE command_queue
             SET status = 'succeeded', updated_at = unixepoch()
             WHERE id = ?1",
            rusqlite::params![id],
        )?;
        Ok(())
    }

    pub fn mark_failed(&self, id: i64, error: &str) -> Result<()> {
        self.db.conn().execute(
            "UPDATE command_queue
             SET status = 'failed',
                 last_error_message = ?2,
                 attempt_count = attempt_count + 1,
                 updated_at = unixepoch()
             WHERE id = ?1",
            rusqlite::params![id, error],
        )?;
        Ok(())
    }

    pub fn mark_retrying(&self, id: i64, next_retry_at: i64) -> Result<()> {
        self.db.conn().execute(
            "UPDATE command_queue
             SET status = 'retrying', next_retry_at = ?2, updated_at = unixepoch()
             WHERE id = ?1",
            rusqlite::params![id, next_retry_at],
        )?;
        Ok(())
    }

    fn query_by_status(&self, status: &str) -> Result<Vec<QueuedCommand>> {
        let mut stmt = self.db.conn().prepare(
            "SELECT id, remote_command_id, device_id, command_type, payload,
                    status, attempt_count, next_retry_at, last_error_message
             FROM command_queue WHERE status = ?1 ORDER BY id",
        )?;
        let rows: Result<Vec<QueuedCommand>> =
            stmt.query_map(rusqlite::params![status], Self::map_row)?.collect();
        rows
    }

    fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<QueuedCommand> {
        let status_str: String = row.get(5)?;
        Ok(QueuedCommand {
            id: row.get(0)?,
            remote_command_id: row.get(1)?,
            device_id: row.get(2)?,
            command_type: row.get(3)?,
            payload: row.get(4)?,
            status: CommandStatus::from_str(&status_str),
            attempt_count: row.get(6)?,
            next_retry_at: row.get(7)?,
            last_error_message: row.get(8)?,
        })
    }
}

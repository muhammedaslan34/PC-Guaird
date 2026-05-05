#![allow(dead_code)]

pub mod lock;
pub mod restart;
pub mod shutdown;

const SUPPORTED: &[&str] = &["lock", "shutdown", "restart"];

pub struct CommandType;

impl CommandType {
    pub fn is_supported(name: &str) -> bool {
        SUPPORTED.contains(&name)
    }
}

pub struct CommandRequest {
    pub command_type: String,
    pub payload: String,
}

#[derive(Debug)]
pub struct CommandResult {
    pub command_type: String,
    pub dry_run: bool,
}

pub struct CommandHandler {
    dry_run: bool,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self { dry_run: false }
    }

    pub fn new_dry_run() -> Self {
        Self { dry_run: true }
    }

    pub fn execute(&self, req: CommandRequest) -> Result<CommandResult, String> {
        if !CommandType::is_supported(&req.command_type) {
            return Err(format!(
                "unsupported command: '{}' — rejected before execution",
                req.command_type
            ));
        }
        match req.command_type.as_str() {
            "lock" => lock::execute_lock(self.dry_run)?,
            "shutdown" => shutdown::execute_shutdown(self.dry_run)?,
            "restart" => restart::execute_restart(self.dry_run)?,
            _ => unreachable!(),
        }
        Ok(CommandResult {
            command_type: req.command_type,
            dry_run: self.dry_run,
        })
    }
}

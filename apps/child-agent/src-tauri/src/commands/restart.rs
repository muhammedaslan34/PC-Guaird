#![allow(dead_code)]

pub fn execute_restart(dry_run: bool) -> Result<(), String> {
    if dry_run {
        return Ok(());
    }
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("shutdown")
            .args(["/r", "/t", "0"])
            .spawn()
            .map_err(|e| format!("restart failed: {e}"))?;
    }
    Ok(())
}

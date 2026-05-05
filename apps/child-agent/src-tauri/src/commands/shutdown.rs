#![allow(dead_code)]

pub fn execute_shutdown(dry_run: bool) -> Result<(), String> {
    if dry_run {
        return Ok(());
    }
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("shutdown")
            .args(["/s", "/t", "0"])
            .spawn()
            .map_err(|e| format!("shutdown failed: {e}"))?;
    }
    Ok(())
}

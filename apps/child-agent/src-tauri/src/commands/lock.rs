#![allow(dead_code)]

pub fn execute_lock(dry_run: bool) -> Result<(), String> {
    if dry_run {
        return Ok(());
    }
    // Windows: LockWorkStation()
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("rundll32.exe")
            .args(["user32.dll,LockWorkStation"])
            .spawn()
            .map_err(|e| format!("lock failed: {e}"))?;
    }
    Ok(())
}

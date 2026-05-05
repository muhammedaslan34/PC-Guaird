use crate::commands::{CommandHandler, CommandRequest, CommandType};

#[test]
fn lock_command_is_in_supported_set() {
    assert!(CommandType::is_supported("lock"));
}

#[test]
fn shutdown_command_is_in_supported_set() {
    assert!(CommandType::is_supported("shutdown"));
}

#[test]
fn restart_command_is_in_supported_set() {
    assert!(CommandType::is_supported("restart"));
}

#[test]
fn unknown_command_is_not_supported() {
    assert!(!CommandType::is_supported("delete_files"));
    assert!(!CommandType::is_supported(""));
    assert!(!CommandType::is_supported("LOCK")); // case-sensitive
}

#[test]
fn unsupported_command_is_rejected_before_execution() {
    let handler = CommandHandler::new_dry_run();
    let result = handler.execute(CommandRequest {
        command_type: "format_disk".to_string(),
        payload: "{}".to_string(),
    });
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.contains("unsupported"),
        "error should mention unsupported: {err}"
    );
}

#[test]
fn lock_command_executes_in_dry_run_mode() {
    let handler = CommandHandler::new_dry_run();
    let result = handler.execute(CommandRequest {
        command_type: "lock".to_string(),
        payload: "{}".to_string(),
    });
    assert!(
        result.is_ok(),
        "lock should succeed in dry-run: {:?}",
        result
    );
    let outcome = result.unwrap();
    assert_eq!(outcome.command_type, "lock");
    assert!(outcome.dry_run);
}

#[test]
fn shutdown_command_executes_in_dry_run_mode() {
    let handler = CommandHandler::new_dry_run();
    let result = handler.execute(CommandRequest {
        command_type: "shutdown".to_string(),
        payload: "{}".to_string(),
    });
    assert!(result.is_ok());
    assert!(result.unwrap().dry_run);
}

#[test]
fn restart_command_executes_in_dry_run_mode() {
    let handler = CommandHandler::new_dry_run();
    let result = handler.execute(CommandRequest {
        command_type: "restart".to_string(),
        payload: "{}".to_string(),
    });
    assert!(result.is_ok());
    assert!(result.unwrap().dry_run);
}

#[test]
fn command_result_carries_command_type() {
    let handler = CommandHandler::new_dry_run();
    let result = handler
        .execute(CommandRequest {
            command_type: "restart".to_string(),
            payload: "{}".to_string(),
        })
        .unwrap();
    assert_eq!(result.command_type, "restart");
}

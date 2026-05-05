use std::fs;
use std::path::Path;

#[test]
fn installer_assets_and_docs_exist_for_windows_service_distribution() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let app_root = crate_root.parent().expect("app root");

    let wix_path = app_root.join("windows/child-agent-service.wxs");
    let tauri_config_path = crate_root.join("tauri.conf.json");
    let service_bin_path = crate_root.join("src/bin/child-agent-service.rs");
    let tray_bin_path = crate_root.join("src/bin/child-agent-tray.rs");
    let env_example_path = app_root.join(".env.example");
    let readme_path = app_root.join("README.md");
    let app_package_path = app_root.join("package.json");
    let root_package_path = app_root
        .parent()
        .and_then(Path::parent)
        .expect("repo root")
        .join("package.json");

    assert!(wix_path.exists(), "expected WiX installer definition");
    assert!(tauri_config_path.exists(), "expected Tauri config");
    assert!(
        service_bin_path.exists(),
        "expected dedicated service binary entrypoint"
    );
    assert!(
        tray_bin_path.exists(),
        "expected dedicated tray binary entrypoint"
    );
    assert!(
        env_example_path.exists(),
        "expected child-agent .env.example"
    );
    assert!(readme_path.exists(), "expected child-agent README");

    let wix_definition = fs::read_to_string(&wix_path).expect("read WiX definition");
    assert!(
        wix_definition.contains("ServiceInstall"),
        "installer must register the Windows service"
    );
    assert!(
        wix_definition.contains("ServiceControl"),
        "installer must control service lifecycle"
    );
    assert!(
        wix_definition.contains("child-agent-tray.exe"),
        "installer must package the tray companion"
    );

    let tauri_config = fs::read_to_string(&tauri_config_path).expect("read tauri config");
    assert!(
        tauri_config.contains("\"productName\": \"Child Agent\""),
        "tauri config should define the product name"
    );

    let env_example = fs::read_to_string(&env_example_path).expect("read env example");
    assert!(
        env_example.contains("CHILD_AGENT_API_BASE_URL="),
        "env example should document backend URL configuration"
    );
    assert!(
        env_example.contains("CHILD_AGENT_POLL_INTERVAL_SECS="),
        "env example should document fallback polling configuration"
    );

    let child_readme = fs::read_to_string(&readme_path).expect("read child-agent README");
    assert!(
        child_readme.contains("Windows Service"),
        "README should explain the Windows Service runtime"
    );
    assert!(
        child_readme.contains("pnpm --filter child-agent test"),
        "README should document the child-agent test command"
    );
    assert!(
        child_readme.contains("administrator"),
        "README should mention administrator privileges for installation"
    );

    let app_package = fs::read_to_string(&app_package_path).expect("read child-agent package");
    assert!(
        app_package.contains("\"build:installer\""),
        "child-agent package.json should expose an installer build script"
    );
    assert!(
        app_package.contains("--bin child-agent-service"),
        "installer build flow should build the service binary"
    );
    assert!(
        app_package.contains("--bin child-agent-tray"),
        "installer build flow should build the tray binary"
    );

    let root_package = fs::read_to_string(&root_package_path).expect("read root package");
    assert!(
        root_package.contains("\"package:child-agent\""),
        "root package.json should expose a child-agent packaging script"
    );
}

use std::path::Path;

#[test]
fn child_agent_workspace_starts_with_app_manifest_and_frontend_entrypoint() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    assert!(
        crate_root.join("../package.json").exists(),
        "expected apps/child-agent/package.json to exist"
    );

    assert!(
        crate_root.join("../src/main.ts").exists(),
        "expected apps/child-agent/src/main.ts to exist"
    );
}

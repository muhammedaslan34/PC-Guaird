# Child Agent Windows Service Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Windows-only child agent in `apps/child-agent` that installs as a Windows Service plus tray companion, pairs with a backend-issued code, keeps running in the background, and executes a minimal safe command set.

**Architecture:** The child agent is split into a Windows Service and a separate always-available tray companion. The service owns backend connectivity, pairing state, command execution, local retry/cache, and diagnostics storage in SQLite; the tray only displays pairing code, status, and diagnostics over a named pipe IPC channel. A single installer package requires administrator privileges and registers both processes together.

**Tech Stack:** Tauri v2, Rust, Windows Service APIs, SQLite, named pipes, installer tooling, pnpm, Turborepo, Vitest

---

## File Structure

- `apps/child-agent/package.json` - child-agent scripts and dependencies
- `apps/child-agent/tauri.conf.json` or `apps/child-agent/src-tauri/tauri.conf.json` - Tauri configuration for the child app
- `apps/child-agent/src-tauri/Cargo.toml` - Rust crate dependencies for the service and tray runtime
- `apps/child-agent/src-tauri/src/main.rs` - runtime entrypoint and platform bootstrap
- `apps/child-agent/src-tauri/src/app.rs` - shared application wiring
- `apps/child-agent/src-tauri/src/service/*.rs` - service runtime, transport, command execution, retry logic
- `apps/child-agent/src-tauri/src/tray/*.rs` - tray UI process and IPC client
- `apps/child-agent/src-tauri/src/ipc/*.rs` - named pipe server/client helpers
- `apps/child-agent/src-tauri/src/storage/*.rs` - SQLite cache, retry queue, diagnostics storage
- `apps/child-agent/src-tauri/src/pairing/*.rs` - pairing code generation and secure credential storage
- `apps/child-agent/src-tauri/src/commands/*.rs` - supported command handlers
- `apps/child-agent/src-tauri/src/tests/*.rs` - Rust unit and integration tests
- `apps/child-agent/src-tauri/windows/*.wxs` or installer scripts - Windows installer definition
- `apps/child-agent/src/` - tray UI frontend if the shell uses a web UI
- `apps/child-agent/tests/**` - frontend or integration tests if needed
- `apps/child-agent/.env.example` - local development configuration
- `apps/child-agent/README.md` - setup, run, install, and test instructions
- `package.json` - root workspace scripts for child-agent development and testing

## Chunk 1: Scaffold The Child Agent Workspace

### Task 1: Create The Child Agent Tauri Workspace And Root Wiring

**Files:**
- Create: `apps/child-agent/package.json`
- Create: `apps/child-agent/src-tauri/Cargo.toml`
- Create: `apps/child-agent/src-tauri/src/main.rs`
- Create: `apps/child-agent/src-tauri/src/app.rs`
- Create: `apps/child-agent/src/`
- Modify: `package.json`
- Modify: `pnpm-workspace.yaml`
- Modify: `turbo.json`
- Test: `apps/child-agent/src-tauri/src/tests/scaffold_test.rs`

- [ ] **Step 1: Write a failing workspace smoke test that asserts the child-agent scripts or Rust entrypoint exist**
- [ ] **Step 2: Run the test to confirm the scaffold is missing**
- [ ] **Step 3: Create the minimal Tauri workspace and root scripts needed for the new app**
- [ ] **Step 4: Run the smoke test and workspace script to confirm the scaffold is wired**
- [ ] **Step 5: Commit**

## Chunk 2: Service, Tray, And IPC Foundation

### Task 2: Add The Windows Service Entry Point And Named Pipe Contract

**Files:**
- Create: `apps/child-agent/src-tauri/src/service/mod.rs`
- Create: `apps/child-agent/src-tauri/src/service/runtime.rs`
- Create: `apps/child-agent/src-tauri/src/ipc/mod.rs`
- Create: `apps/child-agent/src-tauri/src/ipc/named_pipe.rs`
- Create: `apps/child-agent/src-tauri/src/tests/ipc_named_pipe_test.rs`

- [ ] **Step 1: Write a failing test for service startup and named-pipe message exchange**
- [ ] **Step 2: Run the test to verify the IPC and service code is missing**
- [ ] **Step 3: Implement the minimal service runtime and named-pipe helpers**
- [ ] **Step 4: Run the test to verify the IPC contract passes**
- [ ] **Step 5: Commit**

### Task 3: Add The Tray Companion And Status Display Contract

**Files:**
- Create: `apps/child-agent/src/tray/`
- Create: `apps/child-agent/src-tauri/src/tray/mod.rs`
- Create: `apps/child-agent/src-tauri/src/tray/status.rs`
- Create: `apps/child-agent/src-tauri/src/tests/tray_status_test.rs`

- [ ] **Step 1: Write a failing tray-status test for pairing code and connection display**
- [ ] **Step 2: Run the test to verify the tray UI contract is missing**
- [ ] **Step 3: Implement the minimal tray companion surface**
- [ ] **Step 4: Run the test to verify the tray status rendering passes**
- [ ] **Step 5: Commit**

## Chunk 3: Pairing And Persistence

### Task 4: Implement Pairing Code Generation And Credential Storage

**Files:**
- Create: `apps/child-agent/src-tauri/src/pairing/mod.rs`
- Create: `apps/child-agent/src-tauri/src/pairing/code.rs`
- Create: `apps/child-agent/src-tauri/src/pairing/credentials.rs`
- Create: `apps/child-agent/src-tauri/src/tests/pairing_test.rs`

- [ ] **Step 1: Write failing tests for pairing-code generation and secure credential persistence**
- [ ] **Step 2: Run the tests to confirm the pairing implementation is absent**
- [ ] **Step 3: Implement the pairing helpers and local secret handling**
- [ ] **Step 4: Run the tests to verify pairing behavior passes**
- [ ] **Step 5: Commit**

### Task 5: Implement SQLite Cache, Retry Queue, And Diagnostics

**Files:**
- Create: `apps/child-agent/src-tauri/src/storage/mod.rs`
- Create: `apps/child-agent/src-tauri/src/storage/sqlite.rs`
- Create: `apps/child-agent/src-tauri/src/storage/retry_queue.rs`
- Create: `apps/child-agent/src-tauri/src/storage/diagnostics.rs`
- Create: `apps/child-agent/src-tauri/src/tests/storage_test.rs`

- [ ] **Step 1: Write failing tests for SQLite-backed service state, retry state, and diagnostics logging**
- [ ] **Step 2: Run the tests to confirm the local storage layer is missing**
- [ ] **Step 3: Implement the SQLite schema and repository helpers**
- [ ] **Step 4: Run the tests to verify the cache and retry state pass**
- [ ] **Step 5: Commit**

## Chunk 4: Transport And Commands

### Task 6: Implement Hybrid Transport And Backend Reconnect Logic

**Files:**
- Create: `apps/child-agent/src-tauri/src/service/transport.rs`
- Create: `apps/child-agent/src-tauri/src/tests/transport_test.rs`

- [ ] **Step 1: Write failing tests for websocket-primary and polling-fallback transport behavior**
- [ ] **Step 2: Run the tests to confirm the transport code is missing**
- [ ] **Step 3: Implement the transport state machine and reconnect loop**
- [ ] **Step 4: Run the tests to verify transport behavior passes**
- [ ] **Step 5: Commit**

### Task 7: Implement The Minimal Safe Command Set

**Files:**
- Create: `apps/child-agent/src-tauri/src/commands/mod.rs`
- Create: `apps/child-agent/src-tauri/src/commands/lock.rs`
- Create: `apps/child-agent/src-tauri/src/commands/shutdown.rs`
- Create: `apps/child-agent/src-tauri/src/commands/restart.rs`
- Create: `apps/child-agent/src-tauri/src/tests/commands_test.rs`

- [ ] **Step 1: Write failing tests for lock, shutdown, and restart command handling**
- [ ] **Step 2: Run the tests to confirm the command handlers are missing**
- [ ] **Step 3: Implement the minimal safe command handlers**
- [ ] **Step 4: Run the tests to verify command behavior passes**
- [ ] **Step 5: Commit**

## Chunk 5: Installer And Docs

### Task 8: Add The Windows Installer And Operational Documentation

**Files:**
- Create: `apps/child-agent/windows/*.wxs` or installer scripts
- Create: `apps/child-agent/.env.example`
- Create: `apps/child-agent/README.md`
- Modify: `README.md`
- Modify: `package.json`
- Test: `apps/child-agent/src-tauri/src/tests/installer_test.rs`

- [ ] **Step 1: Write a failing installer test for admin-required service registration**
- [ ] **Step 2: Run the test to confirm installer support is missing**
- [ ] **Step 3: Implement the installer definition and package scripts**
- [ ] **Step 4: Update the root and child-agent docs for install, run, and test workflows**
- [ ] **Step 5: Run the child-agent test suite and workspace verification**
- [ ] **Step 6: Commit**

# Child Agent Windows Service Design

**Date:** 2026-05-05
**Project:** Parent Remote Control System
**Scope:** Child agent phase inside the Turborepo monorepo

## Goal

Define the first implementation target for the child machine agent as a Windows-only package that runs as a background service, pairs through a tray UI, stays connected to the backend, and executes a minimal safe command set without exposing control to the tray app.

## Context

The approved product sequence is:

1. backend platform
2. parent dashboard
3. child agent

The child agent must fit the backend contracts already established in the backend spec and dashboard implementation:

- Laravel remains authoritative for device state, command lifecycle, and audit history
- the child agent authenticates as a device, not a user
- the parent dashboard initiates pairing and command requests

The child agent must be hard to accidentally stop from the tray UI. Closing the tray must not stop the background service.

## Architecture

The recommended architecture is a Windows Service plus a separate always-available tray companion inside `apps/child-agent`.

### Runtime Split

- `service` - background runtime that starts on boot, maintains backend connectivity, receives commands, executes allowed system actions, stores local cache/retry state, and reports status
- `tray UI` - separate always-available process that shows pairing code, connection status, and diagnostics only
- `named pipe` - local IPC boundary between tray UI and service
- `backend` - source of truth for device identity, pairing, commands, and audit records
- `SQLite` - local cache and retry state only, not authoritative control state

### Why This Approach

This design matches the admin-install requirement and keeps the service alive even when the tray UI is closed. A named pipe gives Windows-native local communication without depending on a localhost HTTP surface. SQLite is still useful for local retry and diagnostics, but the backend remains the only source of truth.

### Package Model

Phase 1 should ship as one installer package that installs both the service and the tray companion together.

### Child Agent Domain Boundaries

- `Installer` - requires administrator privileges and registers the service
- `Pairing` - child-generated pairing code, device credential issuance, secure local storage
- `ServiceRuntime` - startup, persistence, heartbeat, reconnect, retry queue
- `Transport` - Reverb/WebSocket primary with polling/heartbeat fallback
- `Commands` - command intake, execution, results, retries
- `Diagnostics` - local logs, connection state, service health
- `Tray` - pairing display, status display, diagnostics display, IPC client

## Technology Decisions

- Platform: `Windows only`
- App shell: `Tauri v2`
- Service model: `Windows Service`
- Installer: `single package` with administrator privileges required
- IPC: `named pipe`
- Local storage: `SQLite`
- Local state model: backend authoritative, SQLite cache + retry state
- Transport: hybrid transport with Reverb/WebSocket primary and polling/heartbeat fallback
- Command scope: minimal safe set

## Data Model

The child agent should keep only the local state it needs to stay resilient between reconnects.

### Local SQLite Tables

#### `service_state`

Stores device-local service metadata.

Suggested fields:

- `id`
- `device_uuid`
- `paired_device_id` nullable
- `service_status`
- `last_connected_at`
- `last_heartbeat_at`
- `last_error_code` nullable
- timestamps

#### `pairing_cache`

Stores temporary pairing display state and the latest child-generated code.

Suggested fields:

- `id`
- `device_uuid`
- `pairing_code`
- `expires_at`
- `used_at` nullable
- timestamps

#### `command_queue`

Stores locally cached command intake and retry state.

Suggested fields:

- `id`
- `remote_command_id` nullable
- `device_id`
- `command_type`
- `payload`
- `status`
- `attempt_count`
- `next_retry_at` nullable
- `last_error_message` nullable
- timestamps

#### `diagnostic_events`

Stores local diagnostics for tray display and support.

Suggested fields:

- `id`
- `event_type`
- `event_payload`
- `created_at`

### Device Identity

Phase 1 should use a hybrid identity model:

- a generated app UUID is the primary stable local identifier
- machine metadata is stored only for diagnostics and duplicate detection
- the backend-issued device credential remains the authority after pairing

### Command Statuses

Recommended local statuses:

- `pending`
- `dispatched`
- `acknowledged`
- `succeeded`
- `failed`
- `retrying`
- `rejected`

The local queue can retry work, but it must never override backend authority about final command outcome.

## Runtime Flows

### Installer Flow

1. User runs the installer with administrator privileges.
2. Installer registers the Windows Service.
3. Installer installs the tray companion in the same package.
4. Service is configured to auto-start on boot.
5. Tray companion starts with Windows but does not own the service lifecycle.

### Pairing Flow

1. Unpaired service starts and requests or generates a pairing code.
2. Tray UI reads the current pairing code from the service through the named pipe.
3. Tray UI displays the code to the child user.
4. Parent enters the code in the dashboard.
5. Backend validates the code and issues a device credential.
6. Service stores the credential securely and marks the device paired locally.
7. Tray UI shows successful pairing and current connection state.

### Connection Flow

1. Service loads the stored device credential.
2. Service opens the primary Reverb/WebSocket connection.
3. Service reports heartbeat and status updates to the backend.
4. If the live channel fails, service falls back to polling or reconnect logic.
5. Service updates SQLite cache and exposes status to the tray through the named pipe.

### Command Flow

1. Backend accepts a command for the paired device.
2. Service receives the command through the realtime transport or fallback polling.
3. Service validates that the command is in the supported safe set.
4. Service executes the system action.
5. Service records the result locally and reports it back to the backend.
6. Backend remains the authoritative source for command status and audit history.

## Security Model

Phase 1 security should be strict and operationally simple.

- Installer requires administrator privileges.
- Child devices authenticate with long-lived device credentials, not user credentials.
- Tray UI must not be able to stop the service.
- Named pipe communication should be limited to local machine traffic only.
- SQLite stores only cache and retry state, not authoritative secrets beyond the local credential reference.
- Command execution is limited to the approved safe set.
- Unsupported commands must be rejected locally and reported back as failures.

## Error Handling

The service should fail closed.

- If pairing fails, tray UI should show the backend or service error reason.
- If the backend is unreachable, the service should remain running and retry.
- If the live transport disconnects, the service should fall back to polling or reconnect logic.
- If the tray UI cannot reach the service over the named pipe, it should show a local service-unavailable state.
- If a command is outside the supported safe set, it should be rejected before execution.
- If the service detects repeated failures, it should log diagnostics locally for the tray to surface.

## Testing

Phase 1 testing should cover service behavior, IPC behavior, and installer behavior.

- installer registration and admin-required flow
- service startup and auto-start configuration
- named pipe communication between tray and service
- pairing code generation and display
- device credential storage after pairing
- reconnect and fallback behavior
- command rejection for unsupported actions
- command execution for lock, shutdown, and restart
- SQLite cache and retry state updates
- tray status and diagnostics rendering

## Success Criteria

The child agent phase is complete when:

1. The installer registers a Windows Service and tray companion together.
2. The service starts on boot and stays running independently of the tray UI.
3. The tray UI can display the child-generated pairing code.
4. The parent dashboard can pair the device successfully.
5. The service maintains backend connectivity with hybrid transport.
6. The service executes the minimal safe command set.
7. SQLite stores local cache and retry state without replacing backend authority.
8. Closing the tray UI does not stop the background service.

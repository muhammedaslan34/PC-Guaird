# Child Agent

`apps/child-agent` contains the Windows-only child agent package for the parent remote control system. Phase 1 is split into a background Windows Service plus a separate tray companion so the service stays running even if the tray process is closed.

## Runtime Model

- `child-agent-service` owns pairing, backend transport, command execution, SQLite cache, retry state, and diagnostics.
- `child-agent-tray` displays pairing code, connection status, and diagnostics over local named-pipe IPC.
- Installation requires administrator privileges because the package registers a Windows Service and configures it to start automatically.

## Local Development

1. Create the local environment file:

```bash
copy .env.example .env
```

2. Run the Rust workspace:

```bash
pnpm --filter child-agent dev
```

3. Run the Rust test suite:

```bash
pnpm --filter child-agent test
```

4. Check formatting:

```bash
pnpm --filter child-agent lint
```

## Installer Packaging

Build the release installer assets with:

```bash
pnpm --filter child-agent build:installer
```

The installer definition lives in `windows/child-agent-service.wxs`. The packaging flow expects release binaries for both `child-agent-service.exe` and `child-agent-tray.exe`, then uses WiX to produce a single MSI that installs both binaries together.

## Operational Notes

- The tray companion must never be treated as the service lifecycle owner.
- Unsupported commands are rejected locally before execution.
- The backend remains the authority for device identity, command lifecycle, and audit state.

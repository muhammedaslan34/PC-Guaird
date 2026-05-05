# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Monorepo Overview

This is a Turborepo + pnpm workspace monorepo for a **Parent Remote Control System** — a product that lets parents remotely manage child PCs (lock, shutdown, restart) via a web dashboard and a Windows background service.

Three apps, three phases:
- `apps/backend` — Laravel 13 REST API + WebSocket server (Laravel Reverb) — **Phase 1, complete**
- `apps/dashboard` — Next.js 16 parent-facing web dashboard — **Phase 2, in progress**
- `apps/child-agent` — Tauri v2 + Rust Windows Service + tray companion — **Phase 3, in progress**

## Commands

### Root (runs across apps via Turbo)
```bash
pnpm dev:backend          # Start Laravel API server
pnpm dev:dashboard        # Start Next.js dashboard
pnpm dev:child-agent      # cargo run child agent
pnpm test:backend         # php artisan test
pnpm test:dashboard       # vitest run
pnpm test:child-agent     # cargo test
pnpm lint:backend         # php artisan test --testsuite=Unit
pnpm lint:dashboard       # eslint
pnpm lint:child-agent     # cargo fmt --check
```

### Backend (apps/backend)
```bash
php artisan serve                     # API server on :8000
php artisan reverb:start              # WebSocket server
php artisan queue:work                # Queue worker
php artisan migrate                   # Run migrations
php artisan test                      # Full test suite
php artisan test --filter=TestName    # Single test
php artisan test --testsuite=Feature  # Feature suite only
composer run dev                      # Concurrent: serve + queue + logs + vite
```

Tests use an in-memory SQLite database (configured in `phpunit.xml`).

### Dashboard (apps/dashboard)
```bash
next dev          # Dev server
next build        # Production build
eslint            # Lint
vitest run        # Full test suite
vitest run --reporter=verbose src/tests/path/to/test.test.ts  # Single test file
```

Test pattern: `src/tests/**/*.test.{ts,tsx}` with jsdom environment.

### Child Agent (apps/child-agent)
```bash
cargo run --manifest-path src-tauri/Cargo.toml    # Run
cargo build --manifest-path src-tauri/Cargo.toml  # Build
cargo test --manifest-path src-tauri/Cargo.toml   # All tests
cargo test --manifest-path src-tauri/Cargo.toml test_name  # Single test
cargo fmt --manifest-path src-tauri/Cargo.toml --check     # Lint
```

## Architecture

### System Flow
```
Parent (Dashboard) → Laravel API (REST + Reverb WebSocket) ← Child PC (Tauri Service)
```
The child device **initiates** the outbound WebSocket connection; the parent never connects directly to the child machine.

### Backend Domain Model (`apps/backend/app/`)

Organized by business domain, not framework layer:

- `Domain/Auth/` — `LoginParent`, `RegisterParent` actions
- `Domain/Devices/` — `RecordHeartbeat`, device lifecycle
- `Domain/Pairing/` — `IssuePairingCode`, `RedeemPairingCode` — 6-digit code, expires
- `Domain/Commands/` — `CreateCommand`, `AcknowledgeCommand`, `CompleteCommand`
- `Domain/Audit/` — `AuditLogger` service, append-only log
- `Http/Controllers/Api/` — thin controllers, delegate to Domain actions
- `Models/` — `User`, `Device`, `Command`, `AuditLog`, `DeviceMembership`, `PairingCode`, `DeviceToken`
- `Enums/` — `CommandStatus`, `CommandType`, `DeviceStatus`

Auth: parents authenticate as users (Sanctum token), devices authenticate as devices (long-lived `DeviceToken`, separate from Sanctum). Device routes are in `routes/device.php`, parent routes in `routes/api.php`.

### Dashboard Architecture (`apps/dashboard/src/`)

- `app/(auth)/` — login page
- `app/(dashboard)/` — device list, device detail, pair-device flow (Next.js route groups)
- `app/api/` — Next.js route handlers that proxy to the Laravel backend
- `lib/backend-client.ts` — `backendFetch()` with Bearer token from cookie
- `lib/auth.ts` — httpOnly cookie management (`pc_guard_session`)
- `components/devices/` — `DeviceControlCenter`, `CommandPanel`, `AuditTimeline`, `PairingForm`, `DevicePollingProvider`

The dashboard uses **Next.js route handlers as a BFF** (backend-for-frontend). The browser never calls Laravel directly; all API calls go through `/api/*` route handlers which forward them with the stored auth token.

Requires `BACKEND_URL` env variable (e.g., `http://127.0.0.1:8000`).

### Child Agent Architecture (`apps/child-agent/`)

Split into two processes communicating over a **named pipe**:

- **Windows Service** (`src-tauri/src/service/`) — runs on boot, owns backend connectivity, command execution, SQLite cache, and retry state. Cannot be stopped by the tray.
- **Tray Companion** (`src-tauri/src/tray/`) — displays pairing code, connection status, and diagnostics. IPC client only; does not own any state.

Key domains:
- `ipc/` — named pipe server/client contract
- `pairing/` — pairing code generation, device credential secure storage
- `storage/` — SQLite tables: `service_state`, `pairing_cache`, `command_queue`, `diagnostic_events`
- `commands/` — `lock`, `shutdown`, `restart` handlers (minimal safe set; unsupported commands rejected before execution)

SQLite is local cache and retry state only — **backend is always the authoritative source** for device state, command outcomes, and audit history.

The tray frontend (`src/`) is a minimal web UI shell rendered by Tauri.

### Pairing Flow
1. Unpaired service generates a pairing code → exposes it over the named pipe
2. Tray reads and displays the code
3. Parent enters code in the dashboard → `POST /api/devices/pair`
4. Backend validates and issues a `DeviceToken`
5. Service stores token, marks itself paired, opens Reverb WebSocket

### Transport
Primary: Reverb WebSocket. Fallback: HTTP polling with heartbeat reconnect loop. Service updates SQLite cache and exposes status to tray via named pipe.

## Key Constraints

- The tray UI must **never** be able to stop the Windows Service.
- Commands outside the approved set (`lock`, `shutdown`, `restart`) must be **rejected before execution** and reported back as failures.
- The backend remains the only source of truth; SQLite stores only cache and retry state.
- The installer requires administrator privileges; the service is registered to auto-start on boot.

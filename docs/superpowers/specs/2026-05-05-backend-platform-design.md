# Backend Platform Design

**Date:** 2026-05-05
**Project:** Parent Remote Control System
**Scope:** Backend-first phase inside a Turborepo monorepo

## Goal

Define the first implementation target for a parental control platform as a production-ready backend foundation that supports:

- parent authentication
- child-device pairing
- shared device access
- online/offline presence
- remote command dispatch
- command/audit logging

This backend is the first milestone in a larger monorepo that will later add a Next.js dashboard and a Tauri child agent.

## Context

The current PRD describes three major subsystems:

- backend platform
- parent dashboard
- child PC agent

Those should not be planned as a single implementation unit. The approved sequence is:

1. backend platform
2. Next.js dashboard
3. Tauri child agent

The backend phase should be designed so the later dashboard and child agent can integrate without forcing a major rewrite of identity, pairing, or command-delivery behavior.

## Architecture

The recommended architecture is a modular Laravel monolith inside a Turborepo monorepo.

### Monorepo Shape

- `apps/backend` - Laravel API, Reverb integration, queues/jobs, persistence
- `apps/dashboard` - reserved for the future Next.js parent dashboard
- `apps/child-agent` - reserved for the future Tauri child device agent
- `packages/*` - optional future shared tooling, contracts, scripts, or documentation helpers
- `docs/` - product specs, implementation plans, API notes, and operational documents

### Why This Approach

This keeps the first phase operationally simple while still giving the codebase clean boundaries. Turborepo provides workspace orchestration and shared developer workflows, but does not force a rewrite of the backend into TypeScript. Laravel remains the backend runtime.

### Backend Domain Boundaries

The Laravel backend should be split by business domain rather than by generic framework buckets.

- `Auth` - parent registration, login, logout
- `Devices` - device lifecycle, status, metadata, last-seen state
- `Pairing` - temporary pairing code issuance and redemption
- `DeviceAccess` - shared parent-to-device relationships
- `Commands` - validation, creation, dispatch, acknowledgement, result handling
- `Realtime` - websocket authentication, presence, connection/session behavior
- `Audit` - append-only records for sensitive or operationally relevant events

This is still one deployable application, but the domain boundaries should be reflected in service classes, policies, validation, tests, and folder structure.

## Technology Decisions

- Monorepo tool: `Turborepo`
- Backend framework: `Laravel`
- Database: `MySQL`
- Realtime transport: `Laravel Reverb`
- Parent auth: simplest MVP using email/password only
- Shared access model: multiple parent users per device, all with the same permissions in phase 1
- Offline command behavior: reject commands while device is offline, but keep the command model extensible for retained delivery later

## Data Model

Phase 1 should model shared access explicitly rather than embedding ownership directly on the device row.

### Core Tables

#### `users`

Stores parent accounts for dashboard/backend access.

Suggested fields:

- `id`
- `name`
- `email`
- `password`
- timestamps

#### `devices`

Stores child machine identity and current state.

Suggested fields:

- `id`
- `device_uuid`
- `device_name`
- `status`
- `last_seen_at`
- `paired_at`
- `created_by_user_id` - tracks first claiming user for future evolution, even though phase 1 uses a single effective role
- timestamps

#### `device_memberships`

Join table between users and devices. This is the source of truth for shared access.

Suggested fields:

- `id`
- `device_id`
- `user_id`
- timestamps

Phase 1 does not require role granularity, but this table should exist from the start rather than using `devices.user_id`.

#### `pairing_codes`

Stores short-lived pairing attempts initiated by the child agent.

Suggested fields:

- `id`
- `device_uuid`
- `code`
- `expires_at`
- `used_at`
- timestamps

#### `device_tokens`

Stores hashed long-lived credentials for child-agent authentication after successful pairing.

Suggested fields:

- `id`
- `device_id`
- `token_hash`
- `last_used_at`
- `revoked_at`
- timestamps

Server-side storage should be hashed. Raw credentials are only shown at issuance time to the child agent.

#### `commands`

Stores requested remote actions and their execution lifecycle.

Suggested fields:

- `id`
- `device_id`
- `requested_by_user_id`
- `command_type`
- `payload`
- `status`
- `rejection_reason`
- `dispatched_at`
- `acknowledged_at`
- `executed_at`
- `result_message`
- `result_code`
- timestamps

#### `audit_logs`

Stores security and operational events.

Suggested fields:

- `id`
- `actor_type`
- `actor_id`
- `device_id` nullable
- `event_type`
- `event_payload`
- `created_at`

### Command Statuses

Recommended phase 1 statuses:

- `pending`
- `dispatched`
- `acknowledged`
- `succeeded`
- `failed`
- `rejected`

Although offline commands are rejected in phase 1, this status model allows a future retained-delivery mode without redesigning the command API surface.

## Runtime Flows

### Pairing Flow

1. Unpaired child agent starts and requests a temporary pairing code.
2. Backend generates a short-lived, single-use code tied to a device-generated identifier.
3. Parent enters the code in the dashboard.
4. Backend validates the code.
5. Backend creates or resolves the device record.
6. Backend links the parent user to the device through `device_memberships`.
7. Backend issues a long-lived device credential and stores only its hash.
8. Backend marks the pairing code as used and writes an audit event.
9. Child agent stores the credential securely for future websocket authentication.

### Realtime Connection Flow

1. Child agent starts and loads its stored device credential.
2. Child agent opens an authenticated websocket connection through Reverb.
3. Backend authenticates the device identity.
4. Backend marks the device online and updates activity timestamps.
5. Presence is refreshed through connection activity and/or heartbeat updates.
6. On disconnect or expiry of expected activity, backend marks the device offline.

### Command Flow

1. A linked parent requests a command for a device.
2. Backend verifies the requesting user is linked to that device.
3. Backend validates command type and payload shape.
4. If the device is offline, backend rejects the request and records why.
5. If the device is online, backend creates the command as `pending`.
6. Backend publishes the command through the realtime layer.
7. Device acknowledges receipt.
8. Device executes the action and reports the result.
9. Backend advances status through the lifecycle and records an audit/log event.

## Laravel and Reverb Responsibility Split

Reverb is transport and connection infrastructure. Laravel application services remain the source of truth for:

- authorization
- command lifecycle transitions
- device access checks
- persistence
- audit logging
- validation

This prevents websocket handlers from becoming an unstructured business-logic layer.

## Security Model

Phase 1 security should stay minimal but strict.

- Parent users authenticate with email/password only.
- Child devices authenticate with long-lived device credentials, not user credentials.
- Device credentials must be stored hashed server-side.
- Pairing codes must be short-lived and single-use.
- Every command request must verify membership in `device_memberships`.
- Every important action must be audit logged.
- Command payloads must be validated server-side per supported command type.

Phase 1 does not include:

- MFA
- email verification
- granular device permissions
- retained offline command queue
- multi-tenant administration

Those can be added later if the domain model stays clean.

## Error Handling

The backend should return explicit, stable failure categories rather than generic errors.

### Pairing Errors

- `invalid_pairing_code`
- `expired_pairing_code`
- `used_pairing_code`

### Access Errors

- `unauthorized_device_access`
- `invalid_device_credential`

### Command Errors

- `device_offline`
- `unsupported_command`
- `invalid_payload`
- `command_execution_failed`

Realtime disconnections should never rely on manual cleanup. Device presence must converge automatically through disconnect events and/or activity timeout rules.

## Testing Strategy

Phase 1 testing should prove the core business behaviors, not just endpoint existence.

### High-Value Test Areas

- pairing code generation
- pairing code expiry enforcement
- pairing code single-use enforcement
- device claim flow
- device-membership creation
- shared-access authorization for linked vs unlinked parents
- online/offline status updates
- offline command rejection behavior
- online command lifecycle transitions
- websocket/device authentication
- audit-log creation for pairing, sharing, commands, and results

### Test Layers

- feature tests for HTTP API behavior
- integration tests for command lifecycle and persistence
- targeted realtime/authentication tests around websocket connection contracts
- unit tests only where domain logic is isolated and valuable

## Success Criteria

The backend phase is successful when all of the following work end-to-end:

1. A parent can register and log in.
2. An unpaired child device can request a pairing code.
3. A parent can claim that device using the pairing code.
4. The parent becomes linked to the device through shared-access membership.
5. A paired child device can connect through Reverb using its device credential.
6. Linked parents can view current device status.
7. Linked parents can send a supported command to an online device.
8. The device can acknowledge and report command results.
9. Pairing, access, and command events are persisted in audit/history records.

## Out of Scope for This Spec

The following are intentionally excluded from the first backend spec:

- dashboard UI implementation
- child-agent implementation details
- app blocking
- schedules
- screen time enforcement
- screenshots
- reporting/analytics
- granular permission models
- offline command retention
- notifications

## Recommended Next Step

Create a dedicated implementation plan for this backend phase only. The plan should assume:

- monorepo scaffolding must be created first
- Laravel lives in `apps/backend`
- phase 1 covers API, realtime, persistence, and auditability
- future dashboard and child agent should consume stable backend contracts rather than force backend redesign

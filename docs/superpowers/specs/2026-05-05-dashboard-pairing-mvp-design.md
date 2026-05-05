# Dashboard Pairing MVP Design

**Date:** 2026-05-05
**Project:** Parent Remote Control System
**Scope:** Next.js parent dashboard inside the existing Turborepo monorepo

## Goal

Define the first dashboard implementation as a server-first Next.js application that allows a parent to:

- log in against the Laravel backend
- pair a device using a temporary pairing code
- view linked devices
- open a full control-center device detail page
- send supported commands
- inspect command history and audit logs

This dashboard is the second major milestone after the backend platform and before the Tauri child agent implementation.

## Context

The backend phase is already implemented in `apps/backend` and currently exposes the core APIs needed for:

- parent authentication
- device pairing
- device listing and detail
- command creation and command history
- audit log history
- device presence updates through heartbeat-backed state

The dashboard should consume those APIs without redefining backend behavior. Phase 1 is explicitly a `Pairing MVP`, not a generic marketing or account-management application.

## Architecture

The dashboard should live in `apps/dashboard` as a Next.js App Router application inside the current Turborepo monorepo.

### Rendering Model

- server components by default
- client components only where interaction requires them
- Laravel remains the source of truth for auth and business data
- Next.js is responsible for presentation, protected routing, server-first loading, and focused interactive behavior

This follows the approved `server-first` direction and avoids drifting into a client-heavy SPA.

### Monorepo Placement

- `apps/dashboard` - Next.js parent dashboard
- `apps/backend` - existing Laravel API and Reverb server
- `apps/child-agent` - reserved for the future Tauri child app

### Recommended Dashboard Structure

- `apps/dashboard/app` - App Router routes, layouts, and page entry points
- `apps/dashboard/components` - focused UI pieces
- `apps/dashboard/lib` - Laravel API client, auth helpers, polling utilities, type guards, and schemas
- `apps/dashboard/app/(auth)` - login screen
- `apps/dashboard/app/(dashboard)` - protected app shell
- `apps/dashboard/app/(dashboard)/devices` - device list and detail pages
- `apps/dashboard/app/(dashboard)/pair-device` - parent-side device pairing page
- `apps/dashboard/tests` - route, component, and integration coverage

### Why This Approach

This architecture keeps the dashboard aligned with secure server-driven rendering while still allowing small client-side state islands for:

- polling updates
- form submission feedback
- control-center interactions

It also leaves a clean path for later Reverb-based realtime updates without reshaping the route structure or protected layout.

## Product Scope

Phase 1 is a `Pairing MVP` with a `Full control center` device detail experience.

The first usable release includes:

- parent login
- protected dashboard shell
- linked device list
- parent-side pairing flow
- device detail control center
- supported command submission
- command history visibility
- audit log timeline visibility
- hybrid status updates using polling now and a realtime-ready state model later

Out of scope for this dashboard phase:

- child-agent implementation
- marketing site or public landing pages
- password reset or advanced account settings
- MFA or email verification UX
- app blocking configuration
- schedules or screen time rules
- screenshots or reporting views
- granular role management

## User Flows

### 1. Login

The parent lands on a dedicated login page, submits email/password, and enters the protected dashboard if Laravel authentication succeeds.

Requirements:

- unauthenticated access to protected routes redirects cleanly
- auth failure stays on the login page with a clear error
- no partial protected render before auth resolution

### 2. Device List

After login, the parent sees a control-center overview of linked devices.

Each device card or row should show:

- device name
- online/offline status
- last seen
- entry point to the device detail page
- clear pairing/controllability state

This view should use polling to keep operational status fresh without forcing full page reloads.

### 3. Pair Device

The parent opens a pairing page, enters the temporary code shown by the child device, optionally confirms naming, and submits.

On success:

- the device is linked
- the device appears in the parent device list
- the parent can navigate directly to the device detail page

On failure:

- the UI reflects backend pairing states like `invalid_pairing_code`, `expired_pairing_code`, and `used_pairing_code`

### 4. Device Detail / Full Control Center

This is the primary working surface for phase 1.

It should include:

- current device status
- last seen
- supported command controls
- recent command history with statuses and results
- audit log timeline
- manual refresh controls
- background polling for status, commands, and logs

This page should be dense but readable, optimized for operational use rather than sparse CRUD presentation.

## Data And State Model

The dashboard should explicitly separate server-loaded page state from client refresh state.

### Server-Loaded Page State

Used for:

- auth gate decisions
- initial device list
- initial device detail payload
- initial command history
- initial audit log timeline

This ensures secure and predictable first render behavior.

### Client Refresh State

Used for:

- polling device status
- polling command history changes
- polling audit timeline changes
- pairing form submission feedback
- command submission loading, success, and error states
- stale-data indicators when refresh fails

### Realtime-Ready Constraint

Although phase 1 uses polling, the state model should not assume full-page refresh as the only update mechanism.

The client-side state should be organized around focused containers for:

- device summary
- command collection
- audit collection

Later, Reverb events can update the same structures without changing the route hierarchy or page architecture.

## Backend Contracts

The dashboard should rely on the current Laravel APIs:

- `POST /api/auth/login`
- `GET /api/auth/me`
- `POST /api/auth/logout`
- `GET /api/devices`
- `GET /api/devices/{id}`
- `POST /api/devices/pair`
- `GET /api/devices/{id}/commands`
- `POST /api/devices/{id}/commands`
- `GET /api/devices/{id}/audit-logs`

The current backend does not yet expose a dashboard-specific realtime contract that the Next.js app should depend on for phase 1. Polling is the approved baseline.

## Auth Strategy

The dashboard should use `Laravel auth only`.

Implications:

- Next.js does not introduce NextAuth or a second auth authority
- Laravel remains the authoritative source for login, current-user identity, and logout
- the dashboard should centralize backend auth calls in a small API/auth utility layer

The implementation may still use secure dashboard-side cookie/session bridging if needed for server rendering, but that should remain an integration detail rather than a second auth system.

## Hybrid Status Update Strategy

The approved phase 1 update model is `Hybrid`.

### Phase 1 Behavior

- polling drives device status refresh
- polling drives command history refresh
- polling drives audit timeline refresh

### Future-Proofing Constraint

The UI state and component boundaries should be designed so Reverb can later be introduced for:

- live device online/offline changes
- live command lifecycle updates
- live activity timeline updates

without replacing the main dashboard structure.

## Error Handling

The dashboard should preserve the backend's explicit operational semantics.

### Authentication Errors

- invalid credentials should show a direct login error
- protected routes should redirect unauthenticated users cleanly

### Pairing Errors

Map backend codes to clear user-facing messages while preserving meaning:

- `invalid_pairing_code`
- `expired_pairing_code`
- `used_pairing_code`

### Command Errors

- if the backend returns `device_offline`, the UI should say the device is offline and avoid implying queued delivery
- if polling fails, keep the last known state visible and show a stale-data or refresh-failed indicator

The UI should fail operationally, not theatrically. Preserve context wherever possible.

## Testing Strategy

Phase 1 testing should focus on route protection, server-driven rendering, and operational flows.

### High-Value Test Areas

- auth gate behavior for protected routes
- login success and login failure
- device list rendering from server-fetched data
- pairing form success
- pairing error states from backend response codes
- device detail command submission flow
- command history rendering
- audit timeline rendering
- polling updater behavior
- redirect behavior for unauthenticated access

### Testing Layers

- route and integration tests for auth and protected layouts
- component tests for control-center widgets and forms
- targeted tests for polling-driven state refresh behavior

## UI Direction

The dashboard should feel like a `Full control center`, not a generic admin template.

Design direction:

- dense but readable layout
- strong information grouping
- clear operational hierarchy
- high scanability for online/offline state
- desktop-first composition with mobile-safe behavior
- minimal decorative noise

The core layout should clearly separate:

- device summary
- command controls
- command history
- audit timeline

## Success Criteria

The dashboard phase is successful when all of the following are true:

1. A parent can log in through the dashboard using Laravel auth.
2. Protected routes correctly reject or redirect unauthenticated access.
3. A parent can pair a device from the dashboard using a valid pairing code.
4. Linked devices appear in the dashboard list.
5. A parent can open a full control-center device detail page.
6. A parent can send a supported command from the dashboard.
7. Command history is visible from the device detail page.
8. Audit log history is visible from the device detail page.
9. Polling keeps device status and history reasonably fresh without a structural dependency on Reverb.

## Recommended Next Step

Create a dedicated implementation plan for the dashboard phase only.

The plan should assume:

- `apps/dashboard` is currently empty
- the backend in `apps/backend` is the integration target
- the app should be server-first by default
- the first dashboard release must include pairing and a full control-center device detail page

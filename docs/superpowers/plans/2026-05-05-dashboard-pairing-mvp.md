# Dashboard Pairing MVP Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a server-first Next.js dashboard in `apps/dashboard` that uses Laravel as the only auth source and supports login, device pairing, device list, full control-center device detail, command actions, and audit visibility.

**Architecture:** The dashboard is an App Router Next.js app inside the existing Turborepo. Server components handle protected route loading and initial data fetches. A secure httpOnly cookie stores the Laravel personal access token, and Next.js route handlers/server actions proxy browser interactions so the backend token does not leak to client JavaScript. Polling-driven client components refresh device, command, and audit state without changing the overall server-first route model.

**Tech Stack:** Next.js, React, TypeScript, pnpm, Turborepo, Vitest, React Testing Library

---

## File Structure

- `apps/dashboard/package.json` - dashboard scripts and dependencies
- `apps/dashboard/next.config.ts` - Next.js app config
- `apps/dashboard/tsconfig.json` - TypeScript config
- `apps/dashboard/src/app/layout.tsx` - global layout
- `apps/dashboard/src/app/page.tsx` - root redirect entry
- `apps/dashboard/src/app/(auth)/login/page.tsx` - login page
- `apps/dashboard/src/app/(dashboard)/layout.tsx` - protected shell
- `apps/dashboard/src/app/(dashboard)/devices/page.tsx` - device list
- `apps/dashboard/src/app/(dashboard)/devices/[deviceId]/page.tsx` - control-center detail
- `apps/dashboard/src/app/(dashboard)/pair-device/page.tsx` - pairing page
- `apps/dashboard/src/app/api/auth/login/route.ts` - secure login proxy
- `apps/dashboard/src/app/api/auth/logout/route.ts` - secure logout proxy
- `apps/dashboard/src/app/api/devices/route.ts` - devices polling proxy
- `apps/dashboard/src/app/api/devices/[deviceId]/route.ts` - device detail proxy
- `apps/dashboard/src/app/api/devices/[deviceId]/commands/route.ts` - command list/create proxy
- `apps/dashboard/src/app/api/devices/[deviceId]/audit-logs/route.ts` - audit list proxy
- `apps/dashboard/src/app/api/devices/pair/route.ts` - pairing proxy
- `apps/dashboard/src/lib/auth.ts` - cookie-backed auth helpers
- `apps/dashboard/src/lib/backend-client.ts` - typed Laravel API client
- `apps/dashboard/src/lib/dashboard-types.ts` - dashboard DTO types
- `apps/dashboard/src/lib/dashboard-api.ts` - higher-level server fetch helpers
- `apps/dashboard/src/components/auth/login-form.tsx` - interactive login form
- `apps/dashboard/src/components/devices/device-list.tsx` - device list rendering
- `apps/dashboard/src/components/devices/pair-device-form.tsx` - pairing UI
- `apps/dashboard/src/components/devices/device-control-center.tsx` - top-level detail composition
- `apps/dashboard/src/components/devices/command-panel.tsx` - command actions
- `apps/dashboard/src/components/devices/command-history.tsx` - command timeline
- `apps/dashboard/src/components/devices/audit-timeline.tsx` - audit log timeline
- `apps/dashboard/src/components/devices/device-polling-provider.tsx` - polling state coordination
- `apps/dashboard/src/middleware.ts` - route protection redirects
- `apps/dashboard/src/tests/**` - app, component, and route-handler tests

## Chunk 1: Scaffold The Dashboard App

### Task 1: Create The Next.js App And Workspace Wiring

**Files:**
- Create: `apps/dashboard/*` via Next.js scaffold
- Modify: `apps/dashboard/package.json`
- Modify: `package.json`
- Test: `apps/dashboard/src/tests/smoke/dashboard-smoke.test.ts`

- [ ] **Step 1: Remove the placeholder and scaffold the Next.js app**
- [ ] **Step 2: Add a failing smoke test that asserts the dashboard root route setup exists**
- [ ] **Step 3: Add Turbo-compatible dashboard scripts for `dev`, `build`, `test`, and `lint`**
- [ ] **Step 4: Install frontend test dependencies and a Vitest setup**
- [ ] **Step 5: Run the smoke test to verify the scaffold is wired correctly**
- [ ] **Step 6: Commit**

## Chunk 2: Auth Bridge And Protected Routing

### Task 2: Implement Laravel Token Cookie Auth

**Files:**
- Create: `apps/dashboard/src/lib/auth.ts`
- Create: `apps/dashboard/src/lib/backend-client.ts`
- Create: `apps/dashboard/src/app/api/auth/login/route.ts`
- Create: `apps/dashboard/src/app/api/auth/logout/route.ts`
- Create: `apps/dashboard/src/tests/auth/login-route.test.ts`
- Create: `apps/dashboard/src/tests/auth/logout-route.test.ts`

- [ ] **Step 1: Write failing route-handler tests for login and logout**
- [ ] **Step 2: Verify they fail for missing handlers**
- [ ] **Step 3: Implement backend login proxy and secure token cookie storage**
- [ ] **Step 4: Implement logout proxy and cookie cleanup**
- [ ] **Step 5: Run the auth route tests**
- [ ] **Step 6: Commit**

### Task 3: Add Protected Route Behavior And Login Screen

**Files:**
- Create: `apps/dashboard/src/middleware.ts`
- Create: `apps/dashboard/src/app/(auth)/login/page.tsx`
- Create: `apps/dashboard/src/components/auth/login-form.tsx`
- Create: `apps/dashboard/src/app/page.tsx`
- Create: `apps/dashboard/src/tests/auth/protected-routing.test.tsx`
- Create: `apps/dashboard/src/tests/auth/login-form.test.tsx`

- [ ] **Step 1: Write failing tests for redirect/protected route behavior and login form submission states**
- [ ] **Step 2: Verify the tests fail**
- [ ] **Step 3: Implement middleware-based protection and root redirect behavior**
- [ ] **Step 4: Implement the login page and client form**
- [ ] **Step 5: Run the auth UI tests**
- [ ] **Step 6: Commit**

## Chunk 3: Device List And Pairing

### Task 4: Implement Dashboard Shell And Device List

**Files:**
- Create: `apps/dashboard/src/app/(dashboard)/layout.tsx`
- Create: `apps/dashboard/src/app/(dashboard)/devices/page.tsx`
- Create: `apps/dashboard/src/components/devices/device-list.tsx`
- Create: `apps/dashboard/src/app/api/devices/route.ts`
- Create: `apps/dashboard/src/lib/dashboard-types.ts`
- Create: `apps/dashboard/src/lib/dashboard-api.ts`
- Create: `apps/dashboard/src/tests/devices/device-list-page.test.tsx`
- Create: `apps/dashboard/src/tests/devices/devices-route.test.ts`

- [ ] **Step 1: Write failing tests for the devices proxy route and device list page rendering**
- [ ] **Step 2: Verify they fail**
- [ ] **Step 3: Implement typed device fetch helpers and the devices route proxy**
- [ ] **Step 4: Implement the protected dashboard shell and device list page**
- [ ] **Step 5: Run the devices tests**
- [ ] **Step 6: Commit**

### Task 5: Implement Pair Device Flow

**Files:**
- Create: `apps/dashboard/src/app/(dashboard)/pair-device/page.tsx`
- Create: `apps/dashboard/src/components/devices/pair-device-form.tsx`
- Create: `apps/dashboard/src/app/api/devices/pair/route.ts`
- Create: `apps/dashboard/src/tests/devices/pair-device-route.test.ts`
- Create: `apps/dashboard/src/tests/devices/pair-device-form.test.tsx`

- [ ] **Step 1: Write failing tests for successful pairing and pairing error-state mapping**
- [ ] **Step 2: Verify they fail**
- [ ] **Step 3: Implement the pair-device proxy route**
- [ ] **Step 4: Implement the pair-device page and form UX**
- [ ] **Step 5: Run the pairing tests**
- [ ] **Step 6: Commit**

## Chunk 4: Full Control Center

### Task 6: Implement Device Detail Server Load And Polling Model

**Files:**
- Create: `apps/dashboard/src/app/(dashboard)/devices/[deviceId]/page.tsx`
- Create: `apps/dashboard/src/app/api/devices/[deviceId]/route.ts`
- Create: `apps/dashboard/src/app/api/devices/[deviceId]/commands/route.ts`
- Create: `apps/dashboard/src/app/api/devices/[deviceId]/audit-logs/route.ts`
- Create: `apps/dashboard/src/components/devices/device-polling-provider.tsx`
- Create: `apps/dashboard/src/components/devices/device-control-center.tsx`
- Create: `apps/dashboard/src/tests/devices/device-detail-page.test.tsx`
- Create: `apps/dashboard/src/tests/devices/device-polling-provider.test.tsx`

- [ ] **Step 1: Write failing tests for server-loaded device detail and polling refresh behavior**
- [ ] **Step 2: Verify they fail**
- [ ] **Step 3: Implement device detail, commands, and audit proxy routes**
- [ ] **Step 4: Implement the server page and polling coordinator**
- [ ] **Step 5: Run the detail and polling tests**
- [ ] **Step 6: Commit**

### Task 7: Implement Command Panel, Command History, And Audit Timeline

**Files:**
- Create: `apps/dashboard/src/components/devices/command-panel.tsx`
- Create: `apps/dashboard/src/components/devices/command-history.tsx`
- Create: `apps/dashboard/src/components/devices/audit-timeline.tsx`
- Create: `apps/dashboard/src/tests/devices/command-panel.test.tsx`
- Create: `apps/dashboard/src/tests/devices/command-history.test.tsx`
- Create: `apps/dashboard/src/tests/devices/audit-timeline.test.tsx`

- [ ] **Step 1: Write failing component tests for command submission, history rendering, and audit rendering**
- [ ] **Step 2: Verify they fail**
- [ ] **Step 3: Implement the full control-center UI sections**
- [ ] **Step 4: Make command submission update local/polled state cleanly**
- [ ] **Step 5: Run the control-center component tests**
- [ ] **Step 6: Commit**

## Chunk 5: Final Verification And Docs

### Task 8: Finalize Docs And End-To-End Dashboard Coverage

**Files:**
- Modify: `README.md`
- Modify: `apps/dashboard/package.json`
- Create: `apps/dashboard/README.md`
- Create: `apps/dashboard/src/tests/integration/dashboard-flow.test.tsx`

- [ ] **Step 1: Write a failing integration-style dashboard flow test for login -> devices -> pair -> control center**
- [ ] **Step 2: Verify it fails**
- [ ] **Step 3: Update the root and dashboard run/test docs**
- [ ] **Step 4: Make the integration flow pass**
- [ ] **Step 5: Run the full dashboard test suite**
- [ ] **Step 6: Run the root workspace dashboard test command**
- [ ] **Step 7: Commit**

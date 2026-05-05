# Dashboard App

## Purpose

This Next.js application is the parent-facing dashboard for the Parent Remote Control System.

Phase 1 includes:

- Laravel-backed login
- linked device list
- parent-side pairing flow
- full control-center device detail page
- command submission
- command history
- audit timeline
- polling-driven status refresh

## Environment

Create `apps/dashboard/.env.local` and set:

```bash
BACKEND_URL=http://127.0.0.1:8000
```

The dashboard uses Laravel as the only auth source and stores the Laravel API token in a secure httpOnly cookie through its own route handlers.

## Commands

Install dependencies:

```bash
cd ../..
pnpm install
```

Run the dashboard:

```bash
pnpm dev
```

Run tests:

```bash
pnpm test
```

Run lint:

```bash
pnpm lint
```

Build the dashboard:

```bash
pnpm build
```

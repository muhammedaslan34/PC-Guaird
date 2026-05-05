# PC-Guaird

## Overview

This repository is the Turborepo monorepo for the Parent Remote Control System.

Phase 1 now includes the backend platform in `apps/backend`, the parent dashboard in `apps/dashboard`, and the Windows child agent in:

- `apps/child-agent` for the Tauri child agent

## Required Local Tools

- Node.js
- pnpm
- Rust
- Cargo
- PHP
- Composer
- MySQL
- Redis
- WiX Toolset (for Windows child-agent installer packaging)

## Workspace Layout

- `apps/backend` - Laravel backend API and Reverb server
- `apps/dashboard` - Next.js parent dashboard
- `apps/child-agent` - Windows child agent service, tray companion, and installer assets
- `docs/` - specs and implementation plans

## Setup

1. Install root JavaScript dependencies:

```bash
pnpm install
```

2. Install backend PHP dependencies:

```bash
cd apps/backend
composer install
copy .env.example .env
php artisan key:generate
```

3. Create the dashboard environment file:

```bash
copy apps/dashboard/.env.example apps/dashboard/.env.local
```

4. Configure backend services in `apps/backend/.env`:

- MySQL database
- Redis
- Reverb app credentials

5. Configure dashboard services in `apps/dashboard/.env.local`:

- `BACKEND_URL=http://127.0.0.1:8000`

6. Create the child-agent environment file when working on the Windows service:

```bash
copy apps/child-agent/.env.example apps/child-agent/.env
```

7. Run backend migrations:

```bash
php artisan migrate
```

## Run

Start the backend from the repo root:

```bash
pnpm dev:backend
```

Start the dashboard from the repo root:

```bash
pnpm dev:dashboard
```

Build the dashboard from the repo root:

```bash
pnpm build:dashboard
```

Start the child agent from the repo root:

```bash
pnpm dev:child-agent
```

Build the child agent Rust workspace from the repo root:

```bash
pnpm build:child-agent
```

Build the Windows child-agent installer from the repo root:

```bash
pnpm package:child-agent
```

Start Reverb from the backend app:

```bash
cd apps/backend
php artisan reverb:start
```

## Test

Run the backend test suite from the repo root:

```bash
pnpm test:backend
```

Run the dashboard test suite from the repo root:

```bash
pnpm test:dashboard
```

Run the child-agent test suite from the repo root:

```bash
pnpm test:child-agent
```

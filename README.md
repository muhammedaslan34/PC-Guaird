# PC-Guaird

## Overview

This repository is the Turborepo monorepo for the Parent Remote Control System.

Phase 1 now includes the backend platform in `apps/backend` and the parent dashboard in `apps/dashboard`. The child agent will live in:

- `apps/child-agent` for the Tauri child agent

## Required Local Tools

- Node.js
- pnpm
- PHP
- Composer
- MySQL
- Redis

## Workspace Layout

- `apps/backend` - Laravel backend API and Reverb server
- `apps/dashboard` - Next.js parent dashboard
- `apps/child-agent` - reserved for the future desktop agent
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

6. Run backend migrations:

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

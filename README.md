# PC-Guaird

## Overview

This repository is the Turborepo monorepo for the Parent Remote Control System.

Phase 1 focuses on the backend platform in `apps/backend`. Future applications will live in:

- `apps/dashboard` for the Next.js parent dashboard
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
- `apps/dashboard` - reserved for the future dashboard
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

3. Configure backend services in `apps/backend/.env`:

- MySQL database
- Redis
- Reverb app credentials

4. Run backend migrations:

```bash
php artisan migrate
```

## Run

Start the backend from the repo root:

```bash
pnpm dev:backend
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

# Backend App

## Purpose

This Laravel application is the backend platform for the Parent Remote Control System.

Phase 1 covers:

- parent authentication
- device pairing
- shared device access
- device presence
- command dispatch and lifecycle tracking
- audit logs

## Environment

Copy the example environment file and update the values:

```bash
copy .env.example .env
```

Set at minimum:

- `DB_CONNECTION=mysql`
- `DB_HOST`
- `DB_PORT`
- `DB_DATABASE`
- `DB_USERNAME`
- `DB_PASSWORD`
- `REDIS_HOST`
- `REDIS_PORT`
- `REVERB_APP_ID`
- `REVERB_APP_KEY`
- `REVERB_APP_SECRET`
- `REVERB_HOST`
- `REVERB_PORT`
- `REVERB_SCHEME`

## Commands

Install PHP dependencies:

```bash
composer install
```

Generate the app key:

```bash
php artisan key:generate
```

Run migrations:

```bash
php artisan migrate
```

Start the API server:

```bash
php artisan serve
```

Start the Reverb server:

```bash
php artisan reverb:start
```

Run tests:

```bash
php artisan test
```

Run a targeted test file:

```bash
php artisan test tests/Feature/Integration/BackendFlowTest.php
```

# Backend Platform Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Turborepo monorepo with a Laravel backend that supports parent auth, device pairing, shared device access, online presence, remote commands, and audit logging.

**Architecture:** The repository starts as a Turborepo workspace with `apps/backend` as the only production app in phase 1. The backend is a modular Laravel monolith backed by MySQL and Reverb, with domain logic split into focused services for auth, devices, pairing, commands, realtime presence, and audit logging. Future `apps/dashboard` and `apps/child-agent` folders are created now so the monorepo shape is stable from day one.

**Tech Stack:** Turborepo, pnpm, Laravel, PHP, Composer, MySQL, Redis, Laravel Sanctum, Laravel Reverb, PHPUnit

---

## File Structure

### Repository Root

- `package.json` - root workspace scripts for Turbo and backend orchestration
- `pnpm-workspace.yaml` - workspace package discovery
- `turbo.json` - pipeline definitions
- `.gitignore` - root ignores for Node, PHP, vendor, build output, and environment files
- `README.md` - repository bootstrap and run instructions
- `scripts/verify-workspace.ps1` - smoke-checks required monorepo files and app folders
- `apps/dashboard/.gitkeep` - reserves the future Next.js app path
- `apps/child-agent/.gitkeep` - reserves the future Tauri app path

### Backend App

- `apps/backend/bootstrap/app.php` - route and middleware registration
- `apps/backend/routes/api.php` - parent-facing API routes
- `apps/backend/routes/device.php` - child-device HTTP routes
- `apps/backend/routes/channels.php` - Reverb channel authorization
- `apps/backend/config/auth.php` - guards/providers
- `apps/backend/config/sanctum.php` - parent API auth behavior
- `apps/backend/config/broadcasting.php` - websocket transport settings
- `apps/backend/config/reverb.php` - Reverb server configuration
- `apps/backend/config/queue.php` - queue driver configuration for command/audit jobs
- `apps/backend/config/cors.php` - dashboard-safe API CORS rules
- `apps/backend/app/Models/User.php` - parent account model
- `apps/backend/app/Models/Device.php` - child device aggregate root
- `apps/backend/app/Models/DeviceMembership.php` - shared-access join model
- `apps/backend/app/Models/PairingCode.php` - short-lived claim code model
- `apps/backend/app/Models/DeviceToken.php` - hashed child credential model
- `apps/backend/app/Models/Command.php` - remote command model
- `apps/backend/app/Models/AuditLog.php` - audit trail model
- `apps/backend/app/Enums/DeviceStatus.php` - `online` / `offline` status values
- `apps/backend/app/Enums/CommandStatus.php` - lifecycle states for commands
- `apps/backend/app/Enums/CommandType.php` - supported command identifiers
- `apps/backend/app/Policies/DevicePolicy.php` - linked-user access checks
- `apps/backend/app/Domain/Auth/*` - auth actions kept out of controllers
- `apps/backend/app/Domain/Pairing/*` - pairing-code issuance and redemption
- `apps/backend/app/Domain/Devices/*` - heartbeat and presence updates
- `apps/backend/app/Domain/Commands/*` - command creation and lifecycle updates
- `apps/backend/app/Domain/Audit/AuditLogger.php` - centralized audit writer
- `apps/backend/app/Http/Controllers/Api/**` - thin transport controllers
- `apps/backend/app/Http/Requests/**` - request validation per endpoint
- `apps/backend/app/Http/Middleware/AuthenticateDevice.php` - child credential authentication
- `apps/backend/database/migrations/**` - schema for devices, memberships, pairing, commands, audit
- `apps/backend/tests/Feature/**` - endpoint and integration behavior
- `apps/backend/tests/Unit/**` - small, isolated domain-policy tests

## API Surface To Build In Phase 1

### Parent Routes

- `POST /api/auth/register`
- `POST /api/auth/login`
- `POST /api/auth/logout`
- `GET /api/auth/me`
- `GET /api/health`
- `GET /api/devices`
- `GET /api/devices/{device}`
- `POST /api/devices/pair`
- `GET /api/devices/{device}/commands`
- `POST /api/devices/{device}/commands`
- `GET /api/devices/{device}/audit-logs`

### Child Device Routes

- `POST /api/device/pairing-codes`
- `POST /api/device/heartbeat`
- `POST /api/device/commands/{command}/acknowledge`
- `POST /api/device/commands/{command}/complete`

### Realtime Contract

- device-authenticated websocket connection through Reverb
- per-device private channel for command delivery
- parent-visible presence/state changes persisted in the database

## Chunk 1: Bootstrap The Monorepo

### Task 1: Initialize The Repository And Root Workspace

**Files:**
- Create: `package.json`
- Create: `pnpm-workspace.yaml`
- Create: `turbo.json`
- Create: `.gitignore`
- Create: `README.md`
- Create: `scripts/verify-workspace.ps1`
- Create: `apps/dashboard/.gitkeep`
- Create: `apps/child-agent/.gitkeep`
- Modify: none
- Test: `scripts/verify-workspace.ps1`

- [ ] **Step 1: Write the failing workspace verification script**

```powershell
$required = @(
  "package.json",
  "pnpm-workspace.yaml",
  "turbo.json",
  "apps/backend",
  "apps/dashboard",
  "apps/child-agent"
)

$missing = $required | Where-Object { -not (Test-Path $_) }
if ($missing.Count -gt 0) {
  Write-Error ("Missing: " + ($missing -join ", "))
  exit 1
}
```

- [ ] **Step 2: Run the script to verify it fails**

Run: `powershell -ExecutionPolicy Bypass -File scripts/verify-workspace.ps1`
Expected: FAIL with missing root workspace files and app folders

- [ ] **Step 3: Create the root workspace files**

`package.json`

```json
{
  "name": "pc-managment",
  "private": true,
  "packageManager": "pnpm@10",
  "scripts": {
    "dev:backend": "turbo run dev --filter=backend",
    "test:backend": "turbo run test --filter=backend",
    "lint:backend": "turbo run lint --filter=backend"
  },
  "devDependencies": {
    "turbo": "^2.0.0"
  }
}
```

`pnpm-workspace.yaml`

```yaml
packages:
  - "apps/*"
  - "packages/*"
```

`turbo.json`

```json
{
  "$schema": "https://turbo.build/schema.json",
  "tasks": {
    "dev": { "cache": false, "persistent": true },
    "test": { "outputs": [] },
    "lint": { "outputs": [] }
  }
}
```

- [ ] **Step 4: Create placeholder app folders and update README**

Document:
- required local tools: Node, pnpm, PHP, Composer, MySQL, Redis
- backend app location: `apps/backend`
- future app folders: `apps/dashboard`, `apps/child-agent`

- [ ] **Step 5: Run the workspace verification script again**

Run: `powershell -ExecutionPolicy Bypass -File scripts/verify-workspace.ps1`
Expected: PASS with exit code `0`

- [ ] **Step 6: Initialize git if the repository is not already initialized**

Run: `git init`
Expected: repository created at the project root

- [ ] **Step 7: Commit the workspace bootstrap**

```bash
git add package.json pnpm-workspace.yaml turbo.json .gitignore README.md scripts/verify-workspace.ps1 apps/dashboard/.gitkeep apps/child-agent/.gitkeep
git commit -m "chore: bootstrap monorepo workspace"
```

### Task 2: Scaffold The Laravel Backend Shell

**Files:**
- Create: `apps/backend/*` via Laravel installer
- Modify: `apps/backend/composer.json`
- Modify: `apps/backend/package.json`
- Modify: `apps/backend/.env.example`
- Test: `apps/backend/tests/Feature/Smoke/HealthCheckTest.php`

- [ ] **Step 1: Scaffold the Laravel application into `apps/backend`**

Run: `composer create-project laravel/laravel apps/backend`
Expected: Laravel skeleton created with `artisan`, `app/`, `bootstrap/`, `config/`, `routes/`, `database/`, and `tests/`

- [ ] **Step 2: Install the backend packages needed for phase 1**

Run: `composer require laravel/sanctum laravel/reverb predis/predis`
Expected: Composer updates complete without version conflicts

- [ ] **Step 3: Add backend package scripts for Turbo**

`apps/backend/package.json`

```json
{
  "name": "backend",
  "private": true,
  "scripts": {
    "dev": "php artisan serve",
    "test": "php artisan test",
    "lint": "php artisan test --testsuite=Unit"
  }
}
```

- [ ] **Step 4: Write the failing backend smoke test**

`apps/backend/tests/Feature/Smoke/HealthCheckTest.php`

```php
public function test_health_endpoint_returns_ok(): void
{
    $response = $this->getJson('/api/health');

    $response->assertOk()->assertJson([
        'status' => 'ok',
    ]);
}
```

- [ ] **Step 5: Run the smoke test to verify it fails**

Run: `php artisan test tests/Feature/Smoke/HealthCheckTest.php`
Expected: FAIL with `404` for `/api/health`

- [ ] **Step 6: Install API scaffolding and Reverb base files**

Run: `php artisan install:api`
Expected: Sanctum-ready API scaffolding installed

Run: `php artisan reverb:install`
Expected: Reverb config, broadcasting setup, and channel wiring added

- [ ] **Step 7: Commit the Laravel scaffold**

```bash
git add apps/backend
git commit -m "chore: scaffold laravel backend app"
```

## Chunk 2: Create Backend Foundations And Schema

### Task 3: Configure Route Registration, Middleware, And Health Endpoint

**Files:**
- Modify: `apps/backend/bootstrap/app.php`
- Modify: `apps/backend/routes/api.php`
- Create: `apps/backend/routes/device.php`
- Modify: `apps/backend/config/cors.php`
- Modify: `apps/backend/config/auth.php`
- Modify: `apps/backend/config/sanctum.php`
- Modify: `apps/backend/tests/Feature/Smoke/HealthCheckTest.php`
- Create: `apps/backend/tests/Feature/Smoke/DeviceRouteProtectionTest.php`
- Test: `apps/backend/tests/Feature/Smoke/HealthCheckTest.php`
- Test: `apps/backend/tests/Feature/Smoke/DeviceRouteProtectionTest.php`

- [ ] **Step 1: Write the failing route-protection test for device endpoints**

```php
public function test_device_heartbeat_requires_a_device_credential(): void
{
    $response = $this->postJson('/api/device/heartbeat', []);

    $response->assertUnauthorized();
}
```

- [ ] **Step 2: Run smoke tests to verify they fail**

Run: `php artisan test tests/Feature/Smoke`
Expected: FAIL because `/api/health` is missing and `/api/device/heartbeat` is not registered

- [ ] **Step 3: Register separate parent and device route files**

In `bootstrap/app.php`:

```php
->withRouting(
    api: __DIR__.'/../routes/api.php',
    commands: __DIR__.'/../routes/console.php',
    health: '/up',
    then: function () {
        Route::middleware('api')
            ->prefix('api/device')
            ->group(base_path('routes/device.php'));
    },
)
```

- [ ] **Step 4: Implement the health endpoint and secure the device route group**

`routes/api.php`

```php
Route::get('/health', fn () => response()->json(['status' => 'ok']));
```

Create a custom `AuthenticateDevice` middleware and apply it to:
- `POST /api/device/heartbeat`
- `POST /api/device/commands/{command}/acknowledge`
- `POST /api/device/commands/{command}/complete`

- [ ] **Step 5: Update CORS and auth config for API-only use**

Ensure:
- Sanctum auth works for token-based API calls
- dashboard origin can be added later through `.env`
- device endpoints do not use parent-user session auth

- [ ] **Step 6: Run the smoke suite**

Run: `php artisan test tests/Feature/Smoke`
Expected: PASS

- [ ] **Step 7: Commit the foundation routing changes**

```bash
git add apps/backend/bootstrap/app.php apps/backend/routes/api.php apps/backend/routes/device.php apps/backend/config/cors.php apps/backend/config/auth.php apps/backend/config/sanctum.php apps/backend/app/Http/Middleware/AuthenticateDevice.php apps/backend/tests/Feature/Smoke
git commit -m "feat: register api and device route foundations"
```

### Task 4: Build The Core Schema, Models, And Enums

**Files:**
- Create: `apps/backend/database/migrations/xxxx_xx_xx_xxxxxx_create_devices_table.php`
- Create: `apps/backend/database/migrations/xxxx_xx_xx_xxxxxx_create_device_memberships_table.php`
- Create: `apps/backend/database/migrations/xxxx_xx_xx_xxxxxx_create_pairing_codes_table.php`
- Create: `apps/backend/database/migrations/xxxx_xx_xx_xxxxxx_create_device_tokens_table.php`
- Create: `apps/backend/database/migrations/xxxx_xx_xx_xxxxxx_create_commands_table.php`
- Create: `apps/backend/database/migrations/xxxx_xx_xx_xxxxxx_create_audit_logs_table.php`
- Create: `apps/backend/app/Models/Device.php`
- Create: `apps/backend/app/Models/DeviceMembership.php`
- Create: `apps/backend/app/Models/PairingCode.php`
- Create: `apps/backend/app/Models/DeviceToken.php`
- Create: `apps/backend/app/Models/Command.php`
- Create: `apps/backend/app/Models/AuditLog.php`
- Create: `apps/backend/app/Enums/DeviceStatus.php`
- Create: `apps/backend/app/Enums/CommandStatus.php`
- Create: `apps/backend/app/Enums/CommandType.php`
- Create: `apps/backend/tests/Feature/Database/CoreSchemaTest.php`
- Create: `apps/backend/tests/Unit/Models/DeviceRelationshipsTest.php`
- Test: `apps/backend/tests/Feature/Database/CoreSchemaTest.php`
- Test: `apps/backend/tests/Unit/Models/DeviceRelationshipsTest.php`

- [ ] **Step 1: Write failing database and relationship tests**

`apps/backend/tests/Feature/Database/CoreSchemaTest.php`

```php
public function test_core_tables_exist(): void
{
    $this->assertTrue(Schema::hasTables([
        'devices',
        'device_memberships',
        'pairing_codes',
        'device_tokens',
        'commands',
        'audit_logs',
    ]));
}
```

`apps/backend/tests/Unit/Models/DeviceRelationshipsTest.php`

```php
public function test_device_has_many_memberships(): void
{
    $device = new Device();

    $this->assertInstanceOf(HasMany::class, $device->memberships());
}
```

- [ ] **Step 2: Run those tests to verify they fail**

Run: `php artisan test tests/Feature/Database/CoreSchemaTest.php tests/Unit/Models/DeviceRelationshipsTest.php`
Expected: FAIL because tables and models do not exist

- [ ] **Step 3: Create the migrations with the approved schema**

Include:
- `devices.device_uuid` unique
- `devices.status` default `offline`
- `device_memberships` unique composite on `device_id` + `user_id`
- `pairing_codes.code` indexed, `used_at` nullable
- `device_tokens.token_hash` unique
- `commands.status` default `pending`
- `audit_logs.event_type` indexed

- [ ] **Step 4: Create the models and enum casts**

Examples:

```php
enum DeviceStatus: string
{
    case Online = 'online';
    case Offline = 'offline';
}
```

```php
enum CommandType: string
{
    case Shutdown = 'shutdown';
    case Restart = 'restart';
    case Lock = 'lock';
}
```

- [ ] **Step 5: Add model relationships and guarded/fillable fields**

At minimum:
- `User` -> belongsToMany `Device`
- `Device` -> hasMany `DeviceMembership`, `DeviceToken`, `Command`, `AuditLog`
- `Command` -> belongsTo `Device`, belongsTo `User`

- [ ] **Step 6: Run migrations and rerun the schema tests**

Run: `php artisan migrate:fresh --env=testing`
Expected: migrations complete

Run: `php artisan test tests/Feature/Database/CoreSchemaTest.php tests/Unit/Models/DeviceRelationshipsTest.php`
Expected: PASS

- [ ] **Step 7: Commit the schema layer**

```bash
git add apps/backend/database/migrations apps/backend/app/Models apps/backend/app/Enums apps/backend/tests/Feature/Database apps/backend/tests/Unit/Models apps/backend/app/Models/User.php
git commit -m "feat: add backend core schema and models"
```

## Chunk 3: Parent Auth And Shared Device Access

### Task 5: Build Parent Authentication Endpoints

**Files:**
- Create: `apps/backend/app/Http/Requests/Auth/RegisterRequest.php`
- Create: `apps/backend/app/Http/Requests/Auth/LoginRequest.php`
- Create: `apps/backend/app/Domain/Auth/Actions/RegisterParent.php`
- Create: `apps/backend/app/Domain/Auth/Actions/LoginParent.php`
- Create: `apps/backend/app/Http/Controllers/Api/Auth/RegisterController.php`
- Create: `apps/backend/app/Http/Controllers/Api/Auth/LoginController.php`
- Create: `apps/backend/app/Http/Controllers/Api/Auth/LogoutController.php`
- Create: `apps/backend/app/Http/Controllers/Api/Auth/MeController.php`
- Modify: `apps/backend/routes/api.php`
- Create: `apps/backend/tests/Feature/Auth/RegisterParentTest.php`
- Create: `apps/backend/tests/Feature/Auth/LoginParentTest.php`
- Create: `apps/backend/tests/Feature/Auth/LogoutParentTest.php`
- Create: `apps/backend/tests/Feature/Auth/FetchCurrentParentTest.php`
- Test: `apps/backend/tests/Feature/Auth/*`

- [ ] **Step 1: Write the failing auth feature tests**

Representative tests:

```php
public function test_parent_can_register(): void
{
    $response = $this->postJson('/api/auth/register', [
        'name' => 'Parent One',
        'email' => 'parent@example.com',
        'password' => 'secret1234',
        'password_confirmation' => 'secret1234',
    ]);

    $response->assertCreated()
        ->assertJsonStructure(['user', 'token']);
}
```

```php
public function test_authenticated_parent_can_fetch_self(): void
{
    Sanctum::actingAs(User::factory()->create());

    $this->getJson('/api/auth/me')->assertOk();
}
```

- [ ] **Step 2: Run the auth suite to verify it fails**

Run: `php artisan test tests/Feature/Auth`
Expected: FAIL because auth routes and handlers do not exist

- [ ] **Step 3: Implement request validation and auth actions**

Validation rules:
- `name` required on registration
- `email` required, valid, unique
- `password` required, confirmed, min length set once in request class

Implementation details:
- hash passwords with Laravel defaults
- issue Sanctum token on successful register/login
- revoke current access token on logout

- [ ] **Step 4: Wire the auth controllers and routes**

`routes/api.php`

```php
Route::prefix('auth')->group(function () {
    Route::post('/register', RegisterController::class);
    Route::post('/login', LoginController::class);

    Route::middleware('auth:sanctum')->group(function () {
        Route::post('/logout', LogoutController::class);
        Route::get('/me', MeController::class);
    });
});
```

- [ ] **Step 5: Run the auth suite**

Run: `php artisan test tests/Feature/Auth`
Expected: PASS

- [ ] **Step 6: Commit the auth slice**

```bash
git add apps/backend/app/Http/Requests/Auth apps/backend/app/Domain/Auth apps/backend/app/Http/Controllers/Api/Auth apps/backend/routes/api.php apps/backend/tests/Feature/Auth
git commit -m "feat: add parent authentication api"
```

### Task 6: Build Device Listing And Shared-Access Authorization

**Files:**
- Create: `apps/backend/app/Policies/DevicePolicy.php`
- Create: `apps/backend/app/Http/Controllers/Api/Devices/ListDevicesController.php`
- Create: `apps/backend/app/Http/Controllers/Api/Devices/ShowDeviceController.php`
- Create: `apps/backend/app/Http/Resources/DeviceResource.php`
- Modify: `apps/backend/routes/api.php`
- Create: `apps/backend/tests/Feature/Devices/ListDevicesTest.php`
- Create: `apps/backend/tests/Feature/Devices/ShowDeviceTest.php`
- Create: `apps/backend/tests/Unit/Policies/DevicePolicyTest.php`
- Test: `apps/backend/tests/Feature/Devices/ListDevicesTest.php`
- Test: `apps/backend/tests/Feature/Devices/ShowDeviceTest.php`
- Test: `apps/backend/tests/Unit/Policies/DevicePolicyTest.php`

- [ ] **Step 1: Write the failing device-access tests**

Test cases:
- linked parent sees a device in `GET /api/devices`
- unlinked parent does not see the device
- linked parent can open `GET /api/devices/{device}`
- unlinked parent receives `403`

- [ ] **Step 2: Run the device-access tests to verify they fail**

Run: `php artisan test tests/Feature/Devices/ListDevicesTest.php tests/Feature/Devices/ShowDeviceTest.php tests/Unit/Policies/DevicePolicyTest.php`
Expected: FAIL because routes, resource, and policy do not exist

- [ ] **Step 3: Implement `DevicePolicy` using membership existence**

Core rule:

```php
return $device->memberships()->where('user_id', $user->id)->exists();
```

- [ ] **Step 4: Add list/show controllers and resource output**

Response fields:
- `id`
- `device_uuid`
- `device_name`
- `status`
- `last_seen_at`
- `paired_at`

- [ ] **Step 5: Protect routes with `auth:sanctum` and policy checks**

Routes:

```php
Route::middleware('auth:sanctum')->group(function () {
    Route::get('/devices', ListDevicesController::class);
    Route::get('/devices/{device}', ShowDeviceController::class);
});
```

- [ ] **Step 6: Run the device-access suite**

Run: `php artisan test tests/Feature/Devices/ListDevicesTest.php tests/Feature/Devices/ShowDeviceTest.php tests/Unit/Policies/DevicePolicyTest.php`
Expected: PASS

- [ ] **Step 7: Commit the shared-access slice**

```bash
git add apps/backend/app/Policies/DevicePolicy.php apps/backend/app/Http/Controllers/Api/Devices apps/backend/app/Http/Resources/DeviceResource.php apps/backend/routes/api.php apps/backend/tests/Feature/Devices apps/backend/tests/Unit/Policies
git commit -m "feat: add shared device access authorization"
```

## Chunk 4: Pairing And Device Presence

### Task 7: Implement Pairing Code Issuance And Device Claiming

**Files:**
- Create: `apps/backend/app/Http/Requests/Devices/IssuePairingCodeRequest.php`
- Create: `apps/backend/app/Http/Requests/Devices/PairDeviceRequest.php`
- Create: `apps/backend/app/Domain/Pairing/Actions/IssuePairingCode.php`
- Create: `apps/backend/app/Domain/Pairing/Actions/RedeemPairingCode.php`
- Create: `apps/backend/app/Http/Controllers/Api/Devices/IssuePairingCodeController.php`
- Create: `apps/backend/app/Http/Controllers/Api/Devices/PairDeviceController.php`
- Create: `apps/backend/app/Domain/Audit/AuditLogger.php`
- Modify: `apps/backend/routes/api.php`
- Modify: `apps/backend/routes/device.php`
- Create: `apps/backend/tests/Feature/Devices/IssuePairingCodeTest.php`
- Create: `apps/backend/tests/Feature/Devices/PairDeviceTest.php`
- Test: `apps/backend/tests/Feature/Devices/IssuePairingCodeTest.php`
- Test: `apps/backend/tests/Feature/Devices/PairDeviceTest.php`

- [ ] **Step 1: Write the failing pairing tests**

Required behaviors:
- device can request a short-lived pairing code at `POST /api/device/pairing-codes`
- linked parent can redeem a valid code at `POST /api/devices/pair`
- expired code is rejected with `expired_pairing_code`
- used code is rejected with `used_pairing_code`

Example assertion:

```php
$response->assertUnprocessable()
    ->assertJson(['code' => 'expired_pairing_code']);
```

- [ ] **Step 2: Run the pairing suite to verify it fails**

Run: `php artisan test tests/Feature/Devices/IssuePairingCodeTest.php tests/Feature/Devices/PairDeviceTest.php`
Expected: FAIL because pairing actions and endpoints do not exist

- [ ] **Step 3: Implement pairing-code issuance**

Behavior:
- accept `device_uuid` and optional `device_name`
- generate six-digit numeric code
- set a short expiry window such as 10 minutes
- write an audit event for code issuance

- [ ] **Step 4: Implement pairing-code redemption**

Behavior:
- require `auth:sanctum`
- find active code by `code`
- reject invalid, expired, or already used entries
- create or update `devices`
- create `device_memberships` link for the redeeming parent
- create a `device_tokens` row with hashed credential
- return the raw credential once in the response for the child agent to store

- [ ] **Step 5: Wire the parent and device routes**

Route targets:
- `POST /api/device/pairing-codes`
- `POST /api/devices/pair`

- [ ] **Step 6: Run the pairing suite**

Run: `php artisan test tests/Feature/Devices/IssuePairingCodeTest.php tests/Feature/Devices/PairDeviceTest.php`
Expected: PASS

- [ ] **Step 7: Commit the pairing flow**

```bash
git add apps/backend/app/Http/Requests/Devices apps/backend/app/Domain/Pairing apps/backend/app/Domain/Audit apps/backend/app/Http/Controllers/Api/Devices apps/backend/routes/api.php apps/backend/routes/device.php apps/backend/tests/Feature/Devices/IssuePairingCodeTest.php apps/backend/tests/Feature/Devices/PairDeviceTest.php
git commit -m "feat: add device pairing flow"
```

### Task 8: Implement Device Credential Auth, Heartbeats, And Reverb Channel Auth

**Files:**
- Create: `apps/backend/app/Http/Middleware/AuthenticateDevice.php`
- Create: `apps/backend/app/Http/Requests/Realtime/HeartbeatRequest.php`
- Create: `apps/backend/app/Domain/Devices/Actions/RecordHeartbeat.php`
- Create: `apps/backend/app/Http/Controllers/Api/Devices/HeartbeatController.php`
- Modify: `apps/backend/routes/device.php`
- Modify: `apps/backend/routes/channels.php`
- Modify: `apps/backend/config/broadcasting.php`
- Modify: `apps/backend/config/reverb.php`
- Create: `apps/backend/tests/Feature/Realtime/RecordDeviceHeartbeatTest.php`
- Create: `apps/backend/tests/Feature/Realtime/DeviceChannelAuthTest.php`
- Test: `apps/backend/tests/Feature/Realtime/RecordDeviceHeartbeatTest.php`
- Test: `apps/backend/tests/Feature/Realtime/DeviceChannelAuthTest.php`

- [ ] **Step 1: Write the failing realtime tests**

Required behaviors:
- valid device credential can call `POST /api/device/heartbeat`
- invalid credential gets `401`
- heartbeat updates `devices.status` to `online`
- heartbeat updates `last_seen_at`
- Reverb channel auth only allows the matching device onto its private device channel

- [ ] **Step 2: Run the realtime suite to verify it fails**

Run: `php artisan test tests/Feature/Realtime/RecordDeviceHeartbeatTest.php tests/Feature/Realtime/DeviceChannelAuthTest.php`
Expected: FAIL because middleware, heartbeat action, and channel authorization are incomplete

- [ ] **Step 3: Implement `AuthenticateDevice` using hashed token lookup**

Requirements:
- accept bearer token from child requests
- hash incoming token before comparing if using deterministic hashing strategy, or use Laravel-style token split and constant-time check
- load the owning `Device`
- attach it to the request for downstream handlers

- [ ] **Step 4: Implement heartbeat handling**

Behavior:
- mark device `online`
- update `last_seen_at`
- optionally persist lightweight metadata such as app version or host name if provided
- write an audit event only when status actually changes or a reconnect occurs

- [ ] **Step 5: Add channel authorization for command delivery**

In `routes/channels.php` define a per-device private channel such as:

```php
Broadcast::channel('devices.{deviceId}', function ($device, int $deviceId) {
    return (int) $device->id === $deviceId;
});
```

Use a custom auth flow for devices rather than parent-user guards.

- [ ] **Step 6: Run the realtime suite**

Run: `php artisan test tests/Feature/Realtime/RecordDeviceHeartbeatTest.php tests/Feature/Realtime/DeviceChannelAuthTest.php`
Expected: PASS

- [ ] **Step 7: Commit the presence and realtime auth slice**

```bash
git add apps/backend/app/Http/Middleware/AuthenticateDevice.php apps/backend/app/Http/Requests/Realtime apps/backend/app/Domain/Devices apps/backend/app/Http/Controllers/Api/Devices/HeartbeatController.php apps/backend/routes/device.php apps/backend/routes/channels.php apps/backend/config/broadcasting.php apps/backend/config/reverb.php apps/backend/tests/Feature/Realtime
git commit -m "feat: add device presence and realtime auth"
```

## Chunk 5: Commands, Audit Logs, And Final Verification

### Task 9: Implement Command Creation, Listing, Acknowledgement, And Completion

**Files:**
- Create: `apps/backend/app/Http/Requests/Commands/StoreCommandRequest.php`
- Create: `apps/backend/app/Http/Requests/Commands/AcknowledgeCommandRequest.php`
- Create: `apps/backend/app/Http/Requests/Commands/CompleteCommandRequest.php`
- Create: `apps/backend/app/Domain/Commands/Actions/CreateCommand.php`
- Create: `apps/backend/app/Domain/Commands/Actions/AcknowledgeCommand.php`
- Create: `apps/backend/app/Domain/Commands/Actions/CompleteCommand.php`
- Create: `apps/backend/app/Http/Controllers/Api/Commands/ListCommandsController.php`
- Create: `apps/backend/app/Http/Controllers/Api/Commands/StoreCommandController.php`
- Create: `apps/backend/app/Http/Controllers/Api/Commands/AcknowledgeCommandController.php`
- Create: `apps/backend/app/Http/Controllers/Api/Commands/CompleteCommandController.php`
- Create: `apps/backend/app/Http/Resources/CommandResource.php`
- Modify: `apps/backend/routes/api.php`
- Modify: `apps/backend/routes/device.php`
- Create: `apps/backend/tests/Feature/Commands/CreateCommandTest.php`
- Create: `apps/backend/tests/Feature/Commands/ListCommandsTest.php`
- Create: `apps/backend/tests/Feature/Commands/AcknowledgeCommandTest.php`
- Create: `apps/backend/tests/Feature/Commands/CompleteCommandTest.php`
- Test: `apps/backend/tests/Feature/Commands/*`

- [ ] **Step 1: Write the failing command tests**

Required parent behaviors:
- linked parent can list commands for a linked device
- linked parent can create `shutdown`, `restart`, and `lock` commands
- unlinked parent receives `403`
- offline device command request is rejected with `device_offline`

Required device behaviors:
- device can acknowledge a pending command
- device can complete an acknowledged command with `succeeded` or `failed`

- [ ] **Step 2: Run the command suite to verify it fails**

Run: `php artisan test tests/Feature/Commands`
Expected: FAIL because command controllers and lifecycle actions do not exist

- [ ] **Step 3: Implement `CreateCommand` with server-side validation**

Rules:
- command type must be one of `shutdown`, `restart`, `lock`
- payload defaults to an empty object for current command types
- device must be linked to the requesting parent
- device must be `online` or reject with `rejected` + `device_offline`

- [ ] **Step 4: Broadcast accepted commands to the device channel**

Use Reverb to publish the command after persistence:
- persist command as `pending`
- broadcast to `devices.{deviceId}`
- update `dispatched_at` and status `dispatched` once publish succeeds

- [ ] **Step 5: Implement acknowledgement and completion actions**

State rules:
- `pending` or `dispatched` -> `acknowledged`
- `acknowledged` -> `succeeded` or `failed`
- ignore or reject duplicate terminal updates with a stable API error

- [ ] **Step 6: Run the command suite**

Run: `php artisan test tests/Feature/Commands`
Expected: PASS

- [ ] **Step 7: Commit the command lifecycle slice**

```bash
git add apps/backend/app/Http/Requests/Commands apps/backend/app/Domain/Commands apps/backend/app/Http/Controllers/Api/Commands apps/backend/app/Http/Resources/CommandResource.php apps/backend/routes/api.php apps/backend/routes/device.php apps/backend/tests/Feature/Commands
git commit -m "feat: add command lifecycle api"
```

### Task 10: Expose Audit Logs, Add End-To-End Coverage, And Finalize Docs

**Files:**
- Create: `apps/backend/app/Http/Controllers/Api/Audit/ListAuditLogsController.php`
- Create: `apps/backend/app/Http/Resources/AuditLogResource.php`
- Modify: `apps/backend/routes/api.php`
- Create: `apps/backend/tests/Feature/Audit/ListAuditLogsTest.php`
- Create: `apps/backend/tests/Feature/Integration/BackendFlowTest.php`
- Modify: `README.md`
- Create: `apps/backend/README.md`
- Test: `apps/backend/tests/Feature/Audit/ListAuditLogsTest.php`
- Test: `apps/backend/tests/Feature/Integration/BackendFlowTest.php`

- [ ] **Step 1: Write the failing audit and end-to-end tests**

End-to-end flow should prove:
- parent registers
- device issues pairing code
- parent redeems code
- linked parent sees the device
- device heartbeat marks it online
- parent sends a command
- device acknowledges and completes it
- audit records exist for pairing and commands

- [ ] **Step 2: Run those tests to verify they fail**

Run: `php artisan test tests/Feature/Audit/ListAuditLogsTest.php tests/Feature/Integration/BackendFlowTest.php`
Expected: FAIL because audit listing and integrated flow assertions are incomplete

- [ ] **Step 3: Implement audit listing**

Route:

```php
Route::get('/devices/{device}/audit-logs', ListAuditLogsController::class);
```

Response fields:
- `event_type`
- `actor_type`
- `actor_id`
- `created_at`
- `event_payload`

- [ ] **Step 4: Update docs and local run instructions**

Root `README.md`:
- workspace bootstrap
- install dependencies
- backend env setup
- MySQL and Redis requirements
- running `pnpm dev:backend`

`apps/backend/README.md`:
- migration commands
- Reverb start command
- test commands
- expected env variables

- [ ] **Step 5: Run the end-to-end and targeted suites**

Run: `php artisan test tests/Feature/Audit/ListAuditLogsTest.php tests/Feature/Integration/BackendFlowTest.php`
Expected: PASS

Run: `php artisan test`
Expected: PASS

Run: `pnpm test:backend`
Expected: PASS

- [ ] **Step 6: Commit the audit and verification slice**

```bash
git add apps/backend/app/Http/Controllers/Api/Audit apps/backend/app/Http/Resources/AuditLogResource.php apps/backend/routes/api.php apps/backend/tests/Feature/Audit apps/backend/tests/Feature/Integration README.md apps/backend/README.md
git commit -m "feat: expose audit logs and verify backend flow"
```

## Execution Notes

- Keep controllers thin. Put branching business rules in `app/Domain/**` action classes.
- Do not add offline command retention in phase 1. Return stable rejection errors instead.
- Do not add granular membership roles in phase 1. Shared access is membership-only.
- Prefer feature tests that hit real HTTP endpoints over isolated unit tests for transport-heavy behavior.
- Use factories for `User`, `Device`, `DeviceMembership`, `PairingCode`, and `Command` as soon as each model exists.
- If the Laravel installer generates slightly different files than expected, adapt the implementation while preserving the planned boundaries and endpoint contracts.
- If git was already initialized before execution, skip `git init` and start commits from Task 1 Step 7.

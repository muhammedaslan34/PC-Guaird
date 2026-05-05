# 🧠 Parent Remote Control System (Tauri v2 + Rust + Laravel)

## 📌 Overview

This project is a parental control system that allows a parent to remotely control a child’s PC.

### 🎯 Main Goals

- Shutdown child PC instantly
- Restart / Lock screen
- Close specific applications
- Monitor device online status
- Send commands remotely from web or desktop

---

## 🏗️ Architecture


Parent Dashboard (Web / Desktop)
↓ HTTPS
Backend API (Laravel)
↓ WebSocket (WSS)
Child PC Agent (Tauri v2 + Rust)


### 🔑 Key Principle

The **child PC connects to the server**, not the other way around.


Child PC → Server → Parent Control


---

## 👥 Roles

### 👨‍👩‍👦 Parent

- Login to dashboard
- Add devices
- Send commands
- View logs

### 💻 Child Device

- Connect to backend
- Receive commands
- Execute system actions

---

## 🔗 Pairing Flow

### Step 1 — Child PC

Displays pairing code:


Pairing Code: 483921


---

### Step 2 — Parent

Enters code in dashboard.

---

### Step 3 — Backend

- Links device to parent
- Generates `device_token`

---

### Step 4 — Child App

Stores token and connects via:


wss://api.yourdomain.com/ws/device


---

## 🔄 Connection Flow

On every startup:

1. Load `device_token`
2. Connect to WebSocket
3. Authenticate
4. Stay online
5. Wait for commands

---

## ⚡ Command Flow

### Example: Shutdown

1. Parent clicks "Shutdown"
2. Backend creates command
3. Server sends via WebSocket
4. Child executes command
5. Result sent back

---

## 🧩 Features

### ✅ MVP

- Device pairing
- Online/offline status
- Shutdown PC
- Restart PC
- Lock screen
- Command logs

---

### 🚀 Future Features

- App blocking
- Schedule control
- Screen time limits
- Notifications
- Screenshot capture
- Usage reports

---

## 🛠️ Tech Stack

### Child App

- Tauri v2
- Rust
- WebSocket client

---

### Backend

- Laravel
- Sanctum (Auth)
- Reverb / Soketi (WebSocket)
- MySQL / PostgreSQL
- Redis

---

### Dashboard

- Laravel Blade / Livewire
- OR React / Next.js

---

## 🗄️ Database Schema

### users

- id
- name
- email
- password

---

### devices

- id
- user_id
- device_uuid
- device_name
- device_token_hash
- status
- last_seen_at

---

### pairing_codes

- id
- device_uuid
- code
- expires_at
- used_at

---

### commands

- id
- device_id
- command_type
- payload
- status
- executed_at

---

## 🔌 API Endpoints

### Auth


POST /api/register
POST /api/login
POST /api/logout
GET /api/me


---

### Devices


POST /api/devices/register-temp
POST /api/devices/pair
GET /api/devices
DELETE /api/devices/{id}


---

### Commands


POST /api/devices/{id}/commands
GET /api/devices/{id}/commands
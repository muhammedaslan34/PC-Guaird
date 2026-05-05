<?php

use App\Models\Device;
use Illuminate\Support\Facades\Broadcast;

Broadcast::channel('App.Models.User.{id}', function ($user, $id) {
    return (int) $user->id === (int) $id;
});

Broadcast::channel('devices.{deviceId}', function (Device $device, int $deviceId) {
    return (int) $device->id === $deviceId;
});

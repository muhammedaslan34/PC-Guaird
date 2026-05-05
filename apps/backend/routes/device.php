<?php

use App\Http\Controllers\Api\Devices\IssuePairingCodeController;
use App\Http\Controllers\Api\Devices\HeartbeatController;
use Illuminate\Support\Facades\Route;

Route::post('/pairing-codes', IssuePairingCodeController::class);

Route::middleware('device.auth')->group(function () {
    Route::post('/heartbeat', HeartbeatController::class);

    Route::post('/broadcasting/auth', function () {
        $device = request()->attributes->get('device');
        $channelName = (string) request()->input('channel_name');
        $expectedChannel = "private-devices.{$device->id}";

        if ($channelName !== $expectedChannel) {
            abort(403);
        }

        return response()->json([
            'auth' => "device:{$device->id}",
        ]);
    });

    Route::post('/commands/{command}/acknowledge', function () {
        return response()->noContent();
    });

    Route::post('/commands/{command}/complete', function () {
        return response()->noContent();
    });
});

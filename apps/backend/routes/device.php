<?php

use App\Http\Controllers\Api\Devices\IssuePairingCodeController;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;

Route::post('/pairing-codes', IssuePairingCodeController::class);

Route::middleware('device.auth')->group(function () {
    Route::post('/heartbeat', function (Request $request) {
        return response()->json([
            'status' => 'ok',
            'token' => $request->bearerToken(),
        ]);
    });

    Route::post('/commands/{command}/acknowledge', function () {
        return response()->noContent();
    });

    Route::post('/commands/{command}/complete', function () {
        return response()->noContent();
    });
});

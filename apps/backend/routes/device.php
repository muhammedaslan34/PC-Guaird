<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;

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

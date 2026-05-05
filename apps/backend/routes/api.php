<?php

use App\Http\Controllers\Api\Auth\LoginController;
use App\Http\Controllers\Api\Auth\LogoutController;
use App\Http\Controllers\Api\Auth\MeController;
use App\Http\Controllers\Api\Auth\RegisterController;
use App\Http\Controllers\Api\Audit\ListAuditLogsController;
use App\Http\Controllers\Api\Commands\ListCommandsController;
use App\Http\Controllers\Api\Commands\StoreCommandController;
use App\Http\Controllers\Api\Devices\PairDeviceController;
use App\Http\Controllers\Api\Devices\ListDevicesController;
use App\Http\Controllers\Api\Devices\ShowDeviceController;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;

Route::get('/health', fn () => response()->json(['status' => 'ok']));

Route::prefix('auth')->group(function () {
    Route::post('/register', RegisterController::class);
    Route::post('/login', LoginController::class);

    Route::middleware('auth:sanctum')->group(function () {
        Route::post('/logout', LogoutController::class);
        Route::get('/me', MeController::class);
    });
});

Route::middleware('auth:sanctum')->group(function () {
    Route::get('/devices', ListDevicesController::class);
    Route::get('/devices/{device}', ShowDeviceController::class);
    Route::post('/devices/pair', PairDeviceController::class);
    Route::get('/devices/{device}/commands', ListCommandsController::class);
    Route::post('/devices/{device}/commands', StoreCommandController::class);
    Route::get('/devices/{device}/audit-logs', ListAuditLogsController::class);
});

Route::get('/user', function (Request $request) {
    return $request->user();
})->middleware('auth:sanctum');

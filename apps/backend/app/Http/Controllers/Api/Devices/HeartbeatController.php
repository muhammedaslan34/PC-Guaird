<?php

namespace App\Http\Controllers\Api\Devices;

use App\Domain\Devices\Actions\RecordHeartbeat;
use App\Http\Controllers\Controller;
use App\Http\Requests\Realtime\HeartbeatRequest;
use Illuminate\Http\JsonResponse;

class HeartbeatController extends Controller
{
    public function __invoke(HeartbeatRequest $request, RecordHeartbeat $recordHeartbeat): JsonResponse
    {
        $device = $recordHeartbeat->handle($request);

        return response()->json([
            'status' => 'ok',
            'last_seen_at' => $device->last_seen_at?->toISOString(),
        ]);
    }
}

<?php

namespace App\Http\Controllers\Api\Devices;

use App\Domain\Pairing\Actions\RedeemPairingCode;
use App\Http\Controllers\Controller;
use App\Http\Requests\Devices\PairDeviceRequest;
use Illuminate\Http\JsonResponse;

class PairDeviceController extends Controller
{
    public function __invoke(PairDeviceRequest $request, RedeemPairingCode $redeemPairingCode): JsonResponse
    {
        $result = $redeemPairingCode->handle($request);

        if (isset($result['error'])) {
            return response()->json([
                'code' => $result['error'],
            ], JsonResponse::HTTP_UNPROCESSABLE_ENTITY);
        }

        return response()->json([
            'device' => [
                'id' => $result['device']->id,
                'device_uuid' => $result['device']->device_uuid,
                'device_name' => $result['device']->device_name,
            ],
            'token' => $result['token'],
        ]);
    }
}

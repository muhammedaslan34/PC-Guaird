<?php

namespace App\Http\Controllers\Api\Devices;

use App\Domain\Pairing\Actions\IssuePairingCode;
use App\Http\Controllers\Controller;
use App\Http\Requests\Devices\IssuePairingCodeRequest;
use Illuminate\Http\JsonResponse;

class IssuePairingCodeController extends Controller
{
    public function __invoke(IssuePairingCodeRequest $request, IssuePairingCode $issuePairingCode): JsonResponse
    {
        $pairingCode = $issuePairingCode->handle($request);

        return response()->json([
            'code' => $pairingCode->code,
            'expires_at' => $pairingCode->expires_at->toISOString(),
        ], JsonResponse::HTTP_CREATED);
    }
}

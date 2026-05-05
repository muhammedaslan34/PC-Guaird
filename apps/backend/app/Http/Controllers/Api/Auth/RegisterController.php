<?php

namespace App\Http\Controllers\Api\Auth;

use App\Domain\Auth\Actions\RegisterParent;
use App\Http\Controllers\Controller;
use App\Http\Requests\Auth\RegisterRequest;
use Illuminate\Http\JsonResponse;

class RegisterController extends Controller
{
    public function __invoke(RegisterRequest $request, RegisterParent $registerParent): JsonResponse
    {
        $payload = $registerParent->handle($request);

        return response()->json([
            'user' => $payload['user']->only(['id', 'name', 'email']),
            'token' => $payload['token'],
        ], JsonResponse::HTTP_CREATED);
    }
}

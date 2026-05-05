<?php

namespace App\Http\Controllers\Api\Auth;

use App\Domain\Auth\Actions\LoginParent;
use App\Http\Controllers\Controller;
use App\Http\Requests\Auth\LoginRequest;
use Illuminate\Http\JsonResponse;

class LoginController extends Controller
{
    public function __invoke(LoginRequest $request, LoginParent $loginParent): JsonResponse
    {
        $payload = $loginParent->handle($request);

        return response()->json([
            'user' => $payload['user']->only(['id', 'name', 'email']),
            'token' => $payload['token'],
        ]);
    }
}

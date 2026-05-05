<?php

namespace App\Domain\Auth\Actions;

use App\Http\Requests\Auth\RegisterRequest;
use App\Models\User;

class RegisterParent
{
    public function handle(RegisterRequest $request): array
    {
        $user = User::query()->create($request->validated());

        return [
            'user' => $user,
            'token' => $user->createToken('parent-auth')->plainTextToken,
        ];
    }
}

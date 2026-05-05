<?php

namespace App\Domain\Auth\Actions;

use App\Http\Requests\Auth\LoginRequest;
use App\Models\User;
use Illuminate\Validation\ValidationException;

class LoginParent
{
    public function handle(LoginRequest $request): array
    {
        $user = User::query()->where('email', $request->string('email'))->first();

        if (! $user || ! password_verify($request->string('password')->value(), $user->password)) {
            throw ValidationException::withMessages([
                'email' => 'The provided credentials are incorrect.',
            ]);
        }

        return [
            'user' => $user,
            'token' => $user->createToken('parent-auth')->plainTextToken,
        ];
    }
}

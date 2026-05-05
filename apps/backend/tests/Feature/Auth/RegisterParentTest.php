<?php

namespace Tests\Feature\Auth;

use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class RegisterParentTest extends TestCase
{
    use RefreshDatabase;

    public function test_parent_can_register(): void
    {
        $response = $this->postJson('/api/auth/register', [
            'name' => 'Parent One',
            'email' => 'parent@example.com',
            'password' => 'secret1234',
            'password_confirmation' => 'secret1234',
        ]);

        $response->assertCreated()
            ->assertJsonStructure([
                'user' => ['id', 'name', 'email'],
                'token',
            ]);
    }
}

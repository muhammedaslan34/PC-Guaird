<?php

namespace Tests\Feature\Auth;

use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class LoginParentTest extends TestCase
{
    use RefreshDatabase;

    public function test_parent_can_log_in(): void
    {
        User::factory()->create([
            'email' => 'parent@example.com',
            'password' => 'secret1234',
        ]);

        $response = $this->postJson('/api/auth/login', [
            'email' => 'parent@example.com',
            'password' => 'secret1234',
        ]);

        $response->assertOk()
            ->assertJsonStructure([
                'user' => ['id', 'name', 'email'],
                'token',
            ]);
    }
}

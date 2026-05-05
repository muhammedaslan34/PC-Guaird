<?php

namespace Tests\Feature\Auth;

use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Laravel\Sanctum\Sanctum;
use Tests\TestCase;

class LogoutParentTest extends TestCase
{
    use RefreshDatabase;

    public function test_authenticated_parent_can_log_out(): void
    {
        Sanctum::actingAs(User::factory()->create());

        $this->postJson('/api/auth/logout')
            ->assertOk()
            ->assertJson([
                'message' => 'Logged out.',
            ]);
    }
}

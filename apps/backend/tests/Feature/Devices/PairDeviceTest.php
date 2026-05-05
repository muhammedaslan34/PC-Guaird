<?php

namespace Tests\Feature\Devices;

use App\Models\PairingCode;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Laravel\Sanctum\Sanctum;
use Tests\TestCase;

class PairDeviceTest extends TestCase
{
    use RefreshDatabase;

    public function test_linked_parent_can_redeem_a_valid_pairing_code(): void
    {
        $user = User::factory()->create();
        $pairingCode = PairingCode::query()->create([
            'device_uuid' => 'device-pair-2',
            'code' => '483921',
            'expires_at' => now()->addMinutes(10),
        ]);

        Sanctum::actingAs($user);

        $response = $this->postJson('/api/devices/pair', [
            'code' => $pairingCode->code,
            'device_name' => 'Office PC',
        ]);

        $response->assertOk()
            ->assertJsonStructure([
                'device' => ['id', 'device_uuid', 'device_name'],
                'token',
            ]);
    }

    public function test_expired_code_is_rejected(): void
    {
        $user = User::factory()->create();

        PairingCode::query()->create([
            'device_uuid' => 'device-pair-3',
            'code' => '111111',
            'expires_at' => now()->subMinute(),
        ]);

        Sanctum::actingAs($user);

        $this->postJson('/api/devices/pair', [
            'code' => '111111',
        ])->assertUnprocessable()
            ->assertJson([
                'code' => 'expired_pairing_code',
            ]);
    }

    public function test_used_code_is_rejected(): void
    {
        $user = User::factory()->create();

        PairingCode::query()->create([
            'device_uuid' => 'device-pair-4',
            'code' => '222222',
            'expires_at' => now()->addMinute(),
            'used_at' => now(),
        ]);

        Sanctum::actingAs($user);

        $this->postJson('/api/devices/pair', [
            'code' => '222222',
        ])->assertUnprocessable()
            ->assertJson([
                'code' => 'used_pairing_code',
            ]);
    }
}

<?php

namespace Tests\Feature\Realtime;

use App\Enums\DeviceStatus;
use App\Models\Device;
use App\Models\DeviceToken;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class RecordDeviceHeartbeatTest extends TestCase
{
    use RefreshDatabase;

    public function test_valid_device_credential_can_record_a_heartbeat(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-heartbeat-1',
            'device_name' => 'Heartbeat Device',
            'status' => DeviceStatus::Offline,
            'created_by_user_id' => $user->id,
        ]);

        $plainTextToken = 'device-heartbeat-token';

        DeviceToken::query()->create([
            'device_id' => $device->id,
            'token_hash' => hash('sha256', $plainTextToken),
        ]);

        $this->withToken($plainTextToken)
            ->postJson('/api/device/heartbeat', [
                'host_name' => 'child-pc',
            ])
            ->assertOk()
            ->assertJson([
                'status' => 'ok',
            ]);

        $device->refresh();

        $this->assertSame(DeviceStatus::Online, $device->status);
        $this->assertNotNull($device->last_seen_at);
    }

    public function test_invalid_device_credential_gets_unauthorized(): void
    {
        $this->withToken('invalid-token')
            ->postJson('/api/device/heartbeat', [])
            ->assertUnauthorized();
    }
}

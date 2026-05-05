<?php

namespace Tests\Feature\Realtime;

use App\Enums\DeviceStatus;
use App\Models\Device;
use App\Models\DeviceToken;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class DeviceChannelAuthTest extends TestCase
{
    use RefreshDatabase;

    public function test_device_can_authorize_its_own_private_channel(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-channel-1',
            'device_name' => 'Channel Device',
            'status' => DeviceStatus::Offline,
            'created_by_user_id' => $user->id,
        ]);

        $plainTextToken = 'device-channel-token';

        DeviceToken::query()->create([
            'device_id' => $device->id,
            'token_hash' => hash('sha256', $plainTextToken),
        ]);

        $this->withToken($plainTextToken)
            ->post('/api/device/broadcasting/auth', [
                'socket_id' => '1234.5678',
                'channel_name' => "private-devices.{$device->id}",
            ])
            ->assertOk();
    }

    public function test_device_cannot_authorize_another_devices_channel(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-channel-2',
            'device_name' => 'Channel Device Two',
            'status' => DeviceStatus::Offline,
            'created_by_user_id' => $user->id,
        ]);
        $otherDevice = Device::query()->create([
            'device_uuid' => 'device-channel-3',
            'device_name' => 'Other Device',
            'status' => DeviceStatus::Offline,
            'created_by_user_id' => $user->id,
        ]);

        $plainTextToken = 'device-channel-token-two';

        DeviceToken::query()->create([
            'device_id' => $device->id,
            'token_hash' => hash('sha256', $plainTextToken),
        ]);

        $this->withToken($plainTextToken)
            ->post('/api/device/broadcasting/auth', [
                'socket_id' => '1234.5678',
                'channel_name' => "private-devices.{$otherDevice->id}",
            ])
            ->assertForbidden();
    }
}

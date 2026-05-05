<?php

namespace Tests\Feature\Devices;

use App\Enums\DeviceStatus;
use App\Models\Device;
use App\Models\DeviceMembership;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Laravel\Sanctum\Sanctum;
use Tests\TestCase;

class ShowDeviceTest extends TestCase
{
    use RefreshDatabase;

    public function test_linked_parent_can_open_a_device(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-show-1',
            'device_name' => 'Visible Device',
            'status' => DeviceStatus::Offline,
            'created_by_user_id' => $user->id,
        ]);

        DeviceMembership::query()->create([
            'device_id' => $device->id,
            'user_id' => $user->id,
        ]);

        Sanctum::actingAs($user);

        $this->getJson("/api/devices/{$device->id}")
            ->assertOk()
            ->assertJsonPath('data.id', $device->id);
    }

    public function test_unlinked_parent_receives_forbidden(): void
    {
        $linkedUser = User::factory()->create();
        $unlinkedUser = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-show-2',
            'device_name' => 'Blocked Device',
            'status' => DeviceStatus::Offline,
            'created_by_user_id' => $linkedUser->id,
        ]);

        DeviceMembership::query()->create([
            'device_id' => $device->id,
            'user_id' => $linkedUser->id,
        ]);

        Sanctum::actingAs($unlinkedUser);

        $this->getJson("/api/devices/{$device->id}")
            ->assertForbidden();
    }
}

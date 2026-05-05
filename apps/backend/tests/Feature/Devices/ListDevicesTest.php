<?php

namespace Tests\Feature\Devices;

use App\Enums\DeviceStatus;
use App\Models\Device;
use App\Models\DeviceMembership;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Laravel\Sanctum\Sanctum;
use Tests\TestCase;

class ListDevicesTest extends TestCase
{
    use RefreshDatabase;

    public function test_linked_parent_sees_a_device_in_the_list(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-linked-1',
            'device_name' => 'Linked Device',
            'status' => DeviceStatus::Offline,
            'created_by_user_id' => $user->id,
        ]);

        DeviceMembership::query()->create([
            'device_id' => $device->id,
            'user_id' => $user->id,
        ]);

        Sanctum::actingAs($user);

        $this->getJson('/api/devices')
            ->assertOk()
            ->assertJsonCount(1, 'data')
            ->assertJsonPath('data.0.id', $device->id);
    }

    public function test_unlinked_parent_does_not_see_the_device(): void
    {
        $linkedUser = User::factory()->create();
        $unlinkedUser = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-linked-2',
            'device_name' => 'Hidden Device',
            'status' => DeviceStatus::Offline,
            'created_by_user_id' => $linkedUser->id,
        ]);

        DeviceMembership::query()->create([
            'device_id' => $device->id,
            'user_id' => $linkedUser->id,
        ]);

        Sanctum::actingAs($unlinkedUser);

        $this->getJson('/api/devices')
            ->assertOk()
            ->assertJsonCount(0, 'data');
    }
}

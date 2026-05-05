<?php

namespace Tests\Unit\Policies;

use App\Enums\DeviceStatus;
use App\Models\Device;
use App\Models\DeviceMembership;
use App\Models\User;
use App\Policies\DevicePolicy;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class DevicePolicyTest extends TestCase
{
    use RefreshDatabase;

    public function test_linked_parent_can_view_a_device(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-policy-1',
            'device_name' => 'Policy Device',
            'status' => DeviceStatus::Offline,
            'created_by_user_id' => $user->id,
        ]);

        DeviceMembership::query()->create([
            'device_id' => $device->id,
            'user_id' => $user->id,
        ]);

        $this->assertTrue((new DevicePolicy())->view($user, $device));
    }
}

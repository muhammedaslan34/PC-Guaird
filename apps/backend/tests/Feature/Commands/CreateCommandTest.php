<?php

namespace Tests\Feature\Commands;

use App\Enums\DeviceStatus;
use App\Models\Device;
use App\Models\DeviceMembership;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Laravel\Sanctum\Sanctum;
use Tests\TestCase;

class CreateCommandTest extends TestCase
{
    use RefreshDatabase;

    public function test_linked_parent_can_create_a_shutdown_command_for_an_online_device(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-command-1',
            'device_name' => 'Command Device',
            'status' => DeviceStatus::Online,
            'created_by_user_id' => $user->id,
        ]);

        DeviceMembership::query()->create([
            'device_id' => $device->id,
            'user_id' => $user->id,
        ]);

        Sanctum::actingAs($user);

        $this->postJson("/api/devices/{$device->id}/commands", [
            'command_type' => 'shutdown',
        ])->assertOk()
            ->assertJsonPath('data.command_type', 'shutdown');
    }

    public function test_unlinked_parent_receives_forbidden(): void
    {
        $owner = User::factory()->create();
        $unlinked = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-command-2',
            'device_name' => 'Blocked Command Device',
            'status' => DeviceStatus::Online,
            'created_by_user_id' => $owner->id,
        ]);

        DeviceMembership::query()->create([
            'device_id' => $device->id,
            'user_id' => $owner->id,
        ]);

        Sanctum::actingAs($unlinked);

        $this->postJson("/api/devices/{$device->id}/commands", [
            'command_type' => 'shutdown',
        ])->assertForbidden();
    }

    public function test_offline_device_command_request_is_rejected(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-command-3',
            'device_name' => 'Offline Device',
            'status' => DeviceStatus::Offline,
            'created_by_user_id' => $user->id,
        ]);

        DeviceMembership::query()->create([
            'device_id' => $device->id,
            'user_id' => $user->id,
        ]);

        Sanctum::actingAs($user);

        $this->postJson("/api/devices/{$device->id}/commands", [
            'command_type' => 'shutdown',
        ])->assertUnprocessable()
            ->assertJson([
                'code' => 'device_offline',
            ]);
    }
}

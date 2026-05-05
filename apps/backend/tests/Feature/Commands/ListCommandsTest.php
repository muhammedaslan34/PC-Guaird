<?php

namespace Tests\Feature\Commands;

use App\Enums\CommandStatus;
use App\Enums\CommandType;
use App\Enums\DeviceStatus;
use App\Models\Command;
use App\Models\Device;
use App\Models\DeviceMembership;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Laravel\Sanctum\Sanctum;
use Tests\TestCase;

class ListCommandsTest extends TestCase
{
    use RefreshDatabase;

    public function test_linked_parent_can_list_commands_for_a_linked_device(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-command-list-1',
            'device_name' => 'List Device',
            'status' => DeviceStatus::Online,
            'created_by_user_id' => $user->id,
        ]);

        DeviceMembership::query()->create([
            'device_id' => $device->id,
            'user_id' => $user->id,
        ]);

        Command::query()->create([
            'device_id' => $device->id,
            'requested_by_user_id' => $user->id,
            'command_type' => CommandType::Shutdown,
            'status' => CommandStatus::Pending,
        ]);

        Sanctum::actingAs($user);

        $this->getJson("/api/devices/{$device->id}/commands")
            ->assertOk()
            ->assertJsonCount(1, 'data');
    }
}

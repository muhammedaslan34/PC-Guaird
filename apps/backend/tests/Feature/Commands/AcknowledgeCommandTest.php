<?php

namespace Tests\Feature\Commands;

use App\Enums\CommandStatus;
use App\Enums\CommandType;
use App\Enums\DeviceStatus;
use App\Models\Command;
use App\Models\Device;
use App\Models\DeviceToken;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class AcknowledgeCommandTest extends TestCase
{
    use RefreshDatabase;

    public function test_device_can_acknowledge_a_pending_command(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-command-ack-1',
            'device_name' => 'Ack Device',
            'status' => DeviceStatus::Online,
            'created_by_user_id' => $user->id,
        ]);
        $command = Command::query()->create([
            'device_id' => $device->id,
            'requested_by_user_id' => $user->id,
            'command_type' => CommandType::Shutdown,
            'status' => CommandStatus::Pending,
        ]);

        $plainTextToken = 'device-command-ack-token';

        DeviceToken::query()->create([
            'device_id' => $device->id,
            'token_hash' => hash('sha256', $plainTextToken),
        ]);

        $this->withToken($plainTextToken)
            ->postJson("/api/device/commands/{$command->id}/acknowledge")
            ->assertOk()
            ->assertJson([
                'status' => 'acknowledged',
            ]);
    }
}

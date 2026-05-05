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

class CompleteCommandTest extends TestCase
{
    use RefreshDatabase;

    public function test_device_can_complete_an_acknowledged_command(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-command-complete-1',
            'device_name' => 'Complete Device',
            'status' => DeviceStatus::Online,
            'created_by_user_id' => $user->id,
        ]);
        $command = Command::query()->create([
            'device_id' => $device->id,
            'requested_by_user_id' => $user->id,
            'command_type' => CommandType::Shutdown,
            'status' => CommandStatus::Acknowledged,
        ]);

        $plainTextToken = 'device-command-complete-token';

        DeviceToken::query()->create([
            'device_id' => $device->id,
            'token_hash' => hash('sha256', $plainTextToken),
        ]);

        $this->withToken($plainTextToken)
            ->postJson("/api/device/commands/{$command->id}/complete", [
                'status' => 'succeeded',
                'result_message' => 'Done',
            ])
            ->assertOk()
            ->assertJson([
                'status' => 'succeeded',
            ]);
    }
}

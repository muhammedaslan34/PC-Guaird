<?php

namespace App\Domain\Commands\Actions;

use App\Domain\Audit\AuditLogger;
use App\Enums\CommandStatus;
use App\Enums\DeviceStatus;
use App\Http\Requests\Commands\StoreCommandRequest;
use App\Models\Command;
use App\Models\Device;

class CreateCommand
{
    public function __construct(private readonly AuditLogger $auditLogger)
    {
    }

    public function handle(StoreCommandRequest $request, Device $device): array
    {
        if ($device->status !== DeviceStatus::Online) {
            Command::query()->create([
                'device_id' => $device->id,
                'requested_by_user_id' => $request->user()->id,
                'command_type' => $request->enum('command_type', \App\Enums\CommandType::class),
                'payload' => $request->input('payload', []),
                'status' => CommandStatus::Rejected,
                'rejection_reason' => 'device_offline',
            ]);

            return ['error' => 'device_offline'];
        }

        $command = Command::query()->create([
            'device_id' => $device->id,
            'requested_by_user_id' => $request->user()->id,
            'command_type' => $request->enum('command_type', \App\Enums\CommandType::class),
            'payload' => $request->input('payload', []),
            'status' => CommandStatus::Dispatched,
            'dispatched_at' => now(),
        ]);

        $this->auditLogger->log(
            actorType: 'user',
            actorId: $request->user()->id,
            eventType: 'command_created',
            deviceId: $device->id,
            payload: [
                'command_id' => $command->id,
                'command_type' => $command->command_type->value,
            ],
        );

        return ['command' => $command];
    }
}

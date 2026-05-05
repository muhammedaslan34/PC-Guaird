<?php

namespace App\Domain\Commands\Actions;

use App\Domain\Audit\AuditLogger;
use App\Enums\CommandStatus;
use App\Models\Command;
use App\Models\Device;
use Symfony\Component\HttpKernel\Exception\HttpException;

class AcknowledgeCommand
{
    public function __construct(private readonly AuditLogger $auditLogger)
    {
    }

    public function handle(Device $device, Command $command): Command
    {
        if ($command->device_id !== $device->id) {
            throw new HttpException(403, 'Forbidden');
        }

        $command->forceFill([
            'status' => CommandStatus::Acknowledged,
            'acknowledged_at' => now(),
        ])->save();

        $this->auditLogger->log(
            actorType: 'device',
            actorId: $device->id,
            eventType: 'command_acknowledged',
            deviceId: $device->id,
            payload: ['command_id' => $command->id],
        );

        return $command;
    }
}

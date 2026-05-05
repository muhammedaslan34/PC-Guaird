<?php

namespace App\Domain\Commands\Actions;

use App\Domain\Audit\AuditLogger;
use App\Enums\CommandStatus;
use App\Http\Requests\Commands\CompleteCommandRequest;
use App\Models\Command;
use App\Models\Device;
use Symfony\Component\HttpKernel\Exception\HttpException;

class CompleteCommand
{
    public function __construct(private readonly AuditLogger $auditLogger)
    {
    }

    public function handle(CompleteCommandRequest $request, Device $device, Command $command): Command
    {
        if ($command->device_id !== $device->id) {
            throw new HttpException(403, 'Forbidden');
        }

        $status = CommandStatus::from($request->string('status')->value());

        $command->forceFill([
            'status' => $status,
            'result_message' => $request->input('result_message'),
            'result_code' => $request->input('result_code'),
            'executed_at' => now(),
        ])->save();

        $this->auditLogger->log(
            actorType: 'device',
            actorId: $device->id,
            eventType: 'command_completed',
            deviceId: $device->id,
            payload: [
                'command_id' => $command->id,
                'status' => $status->value,
            ],
        );

        return $command;
    }
}

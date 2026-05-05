<?php

namespace App\Domain\Devices\Actions;

use App\Domain\Audit\AuditLogger;
use App\Enums\DeviceStatus;
use App\Http\Requests\Realtime\HeartbeatRequest;
use App\Models\Device;

class RecordHeartbeat
{
    public function __construct(private readonly AuditLogger $auditLogger)
    {
    }

    public function handle(HeartbeatRequest $request): Device
    {
        /** @var Device $device */
        $device = $request->attributes->get('device');

        $wasOffline = $device->status !== DeviceStatus::Online;

        $device->forceFill([
            'status' => DeviceStatus::Online,
            'last_seen_at' => now(),
        ])->save();

        if ($wasOffline) {
            $this->auditLogger->log(
                actorType: 'device',
                actorId: $device->id,
                eventType: 'device_online',
                deviceId: $device->id,
                payload: $request->validated(),
            );
        }

        $request->attributes->get('deviceToken')?->forceFill([
            'last_used_at' => now(),
        ])->save();

        return $device;
    }
}

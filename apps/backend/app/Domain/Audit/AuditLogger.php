<?php

namespace App\Domain\Audit;

use App\Models\AuditLog;

class AuditLogger
{
    public function log(string $actorType, ?int $actorId, string $eventType, ?int $deviceId = null, ?array $payload = null): AuditLog
    {
        return AuditLog::query()->create([
            'actor_type' => $actorType,
            'actor_id' => $actorId,
            'device_id' => $deviceId,
            'event_type' => $eventType,
            'event_payload' => $payload,
        ]);
    }
}

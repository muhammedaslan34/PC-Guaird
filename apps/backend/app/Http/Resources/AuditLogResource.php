<?php

namespace App\Http\Resources;

use Illuminate\Http\Request;
use Illuminate\Http\Resources\Json\JsonResource;

class AuditLogResource extends JsonResource
{
    public function toArray(Request $request): array
    {
        return [
            'id' => $this->id,
            'event_type' => $this->event_type,
            'actor_type' => $this->actor_type,
            'actor_id' => $this->actor_id,
            'device_id' => $this->device_id,
            'event_payload' => $this->event_payload,
            'created_at' => $this->created_at?->toISOString(),
        ];
    }
}

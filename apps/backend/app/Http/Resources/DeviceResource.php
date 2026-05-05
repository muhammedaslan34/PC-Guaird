<?php

namespace App\Http\Resources;

use Illuminate\Http\Request;
use Illuminate\Http\Resources\Json\JsonResource;

class DeviceResource extends JsonResource
{
    public function toArray(Request $request): array
    {
        return [
            'id' => $this->id,
            'device_uuid' => $this->device_uuid,
            'device_name' => $this->device_name,
            'status' => $this->status?->value ?? $this->status,
            'last_seen_at' => $this->last_seen_at?->toISOString(),
            'paired_at' => $this->paired_at?->toISOString(),
        ];
    }
}

<?php

namespace App\Http\Resources;

use Illuminate\Http\Request;
use Illuminate\Http\Resources\Json\JsonResource;

class CommandResource extends JsonResource
{
    public function toArray(Request $request): array
    {
        return [
            'id' => $this->id,
            'device_id' => $this->device_id,
            'command_type' => $this->command_type?->value ?? $this->command_type,
            'status' => $this->status?->value ?? $this->status,
            'payload' => $this->payload ?? [],
            'rejection_reason' => $this->rejection_reason,
            'dispatched_at' => $this->dispatched_at?->toISOString(),
            'acknowledged_at' => $this->acknowledged_at?->toISOString(),
            'executed_at' => $this->executed_at?->toISOString(),
            'result_message' => $this->result_message,
            'result_code' => $this->result_code,
        ];
    }
}

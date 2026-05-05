<?php

namespace App\Models;

use App\Enums\CommandStatus;
use App\Enums\CommandType;
use Illuminate\Database\Eloquent\Attributes\Fillable;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

#[Fillable([
    'device_id',
    'requested_by_user_id',
    'command_type',
    'payload',
    'status',
    'rejection_reason',
    'dispatched_at',
    'acknowledged_at',
    'executed_at',
    'result_message',
    'result_code',
])]
class Command extends Model
{
    use HasFactory;

    protected function casts(): array
    {
        return [
            'command_type' => CommandType::class,
            'status' => CommandStatus::class,
            'payload' => 'array',
            'dispatched_at' => 'datetime',
            'acknowledged_at' => 'datetime',
            'executed_at' => 'datetime',
        ];
    }

    public function device(): BelongsTo
    {
        return $this->belongsTo(Device::class);
    }

    public function requestedBy(): BelongsTo
    {
        return $this->belongsTo(User::class, 'requested_by_user_id');
    }
}

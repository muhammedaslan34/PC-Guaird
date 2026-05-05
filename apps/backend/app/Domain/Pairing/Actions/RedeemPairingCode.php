<?php

namespace App\Domain\Pairing\Actions;

use App\Domain\Audit\AuditLogger;
use App\Enums\DeviceStatus;
use App\Http\Requests\Devices\PairDeviceRequest;
use App\Models\Device;
use App\Models\DeviceMembership;
use App\Models\DeviceToken;
use App\Models\PairingCode;
use Illuminate\Support\Str;

class RedeemPairingCode
{
    public function __construct(private readonly AuditLogger $auditLogger)
    {
    }

    public function handle(PairDeviceRequest $request): array
    {
        $pairingCode = PairingCode::query()
            ->where('code', $request->string('code')->value())
            ->latest('id')
            ->first();

        if (! $pairingCode) {
            return ['error' => 'invalid_pairing_code'];
        }

        if ($pairingCode->used_at) {
            return ['error' => 'used_pairing_code'];
        }

        if ($pairingCode->expires_at->isPast()) {
            return ['error' => 'expired_pairing_code'];
        }

        $device = Device::query()->firstOrCreate(
            ['device_uuid' => $pairingCode->device_uuid],
            [
                'device_name' => $request->input('device_name'),
                'status' => DeviceStatus::Offline,
                'created_by_user_id' => $request->user()->id,
                'paired_at' => now(),
            ],
        );

        if ($request->filled('device_name')) {
            $device->forceFill(['device_name' => $request->string('device_name')->value()])->save();
        }

        if (! $device->paired_at) {
            $device->forceFill(['paired_at' => now()])->save();
        }

        DeviceMembership::query()->firstOrCreate([
            'device_id' => $device->id,
            'user_id' => $request->user()->id,
        ]);

        $plainTextToken = Str::random(64);

        DeviceToken::query()->create([
            'device_id' => $device->id,
            'token_hash' => hash('sha256', $plainTextToken),
        ]);

        $pairingCode->forceFill(['used_at' => now()])->save();

        $this->auditLogger->log(
            actorType: 'user',
            actorId: $request->user()->id,
            eventType: 'device_paired',
            deviceId: $device->id,
            payload: [
                'device_uuid' => $device->device_uuid,
                'pairing_code_id' => $pairingCode->id,
            ],
        );

        return [
            'device' => $device,
            'token' => $plainTextToken,
        ];
    }
}

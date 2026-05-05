<?php

namespace App\Domain\Pairing\Actions;

use App\Domain\Audit\AuditLogger;
use App\Http\Requests\Devices\IssuePairingCodeRequest;
use App\Models\PairingCode;

class IssuePairingCode
{
    public function __construct(private readonly AuditLogger $auditLogger)
    {
    }

    public function handle(IssuePairingCodeRequest $request): PairingCode
    {
        $pairingCode = PairingCode::query()->create([
            'device_uuid' => $request->string('device_uuid')->value(),
            'code' => str_pad((string) random_int(0, 999999), 6, '0', STR_PAD_LEFT),
            'expires_at' => now()->addMinutes(10),
        ]);

        $this->auditLogger->log(
            actorType: 'device',
            actorId: null,
            eventType: 'pairing_code_issued',
            payload: [
                'device_uuid' => $request->string('device_uuid')->value(),
                'device_name' => $request->input('device_name'),
                'pairing_code_id' => $pairingCode->id,
            ],
        );

        return $pairingCode;
    }
}

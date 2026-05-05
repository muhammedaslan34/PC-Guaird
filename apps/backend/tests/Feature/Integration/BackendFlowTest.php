<?php

namespace Tests\Feature\Integration;

use App\Models\AuditLog;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class BackendFlowTest extends TestCase
{
    use RefreshDatabase;

    public function test_parent_device_pairing_and_command_flow_works_end_to_end(): void
    {
        $registerResponse = $this->postJson('/api/auth/register', [
            'name' => 'Parent Flow',
            'email' => 'flow@example.com',
            'password' => 'secret1234',
            'password_confirmation' => 'secret1234',
        ])->assertCreated();

        $parentToken = $registerResponse->json('token');

        $pairingResponse = $this->postJson('/api/device/pairing-codes', [
            'device_uuid' => 'device-flow-1',
            'device_name' => 'Flow Device',
        ])->assertCreated();

        $pairResponse = $this->withToken($parentToken)
            ->postJson('/api/devices/pair', [
                'code' => $pairingResponse->json('code'),
                'device_name' => 'Flow Device',
            ])->assertOk();

        $deviceId = $pairResponse->json('device.id');
        $deviceToken = $pairResponse->json('token');

        $this->withToken($parentToken)
            ->getJson('/api/devices')
            ->assertOk()
            ->assertJsonCount(1, 'data');

        $this->withToken($deviceToken)
            ->postJson('/api/device/heartbeat', [
                'host_name' => 'flow-host',
            ])->assertOk();

        $commandResponse = $this->withToken($parentToken)
            ->postJson("/api/devices/{$deviceId}/commands", [
                'command_type' => 'shutdown',
            ])->assertOk();

        $commandId = $commandResponse->json('data.id');

        $this->withToken($deviceToken)
            ->postJson("/api/device/commands/{$commandId}/acknowledge")
            ->assertOk()
            ->assertJson([
                'status' => 'acknowledged',
            ]);

        $this->withToken($deviceToken)
            ->postJson("/api/device/commands/{$commandId}/complete", [
                'status' => 'succeeded',
                'result_message' => 'Completed',
            ])->assertOk()
            ->assertJson([
                'status' => 'succeeded',
            ]);

        $this->withToken($parentToken)
            ->getJson("/api/devices/{$deviceId}/audit-logs")
            ->assertOk();

        $this->assertDatabaseCount('audit_logs', AuditLog::query()->count());
        $this->assertGreaterThanOrEqual(3, AuditLog::query()->count());
    }
}

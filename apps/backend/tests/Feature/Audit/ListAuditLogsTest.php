<?php

namespace Tests\Feature\Audit;

use App\Models\AuditLog;
use App\Models\Device;
use App\Models\DeviceMembership;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Laravel\Sanctum\Sanctum;
use Tests\TestCase;

class ListAuditLogsTest extends TestCase
{
    use RefreshDatabase;

    public function test_linked_parent_can_list_audit_logs_for_a_device(): void
    {
        $user = User::factory()->create();
        $device = Device::query()->create([
            'device_uuid' => 'device-audit-1',
            'device_name' => 'Audit Device',
            'created_by_user_id' => $user->id,
        ]);

        DeviceMembership::query()->create([
            'device_id' => $device->id,
            'user_id' => $user->id,
        ]);

        AuditLog::query()->create([
            'actor_type' => 'user',
            'actor_id' => $user->id,
            'device_id' => $device->id,
            'event_type' => 'device_paired',
            'event_payload' => ['device_uuid' => $device->device_uuid],
        ]);

        Sanctum::actingAs($user);

        $this->getJson("/api/devices/{$device->id}/audit-logs")
            ->assertOk()
            ->assertJsonCount(1, 'data')
            ->assertJsonPath('data.0.event_type', 'device_paired');
    }
}

<?php

namespace Tests\Feature\Devices;

use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class IssuePairingCodeTest extends TestCase
{
    use RefreshDatabase;

    public function test_device_can_request_a_pairing_code(): void
    {
        $response = $this->postJson('/api/device/pairing-codes', [
            'device_uuid' => 'device-pair-1',
            'device_name' => 'Family PC',
        ]);

        $response->assertCreated()
            ->assertJsonStructure([
                'code',
                'expires_at',
            ]);
    }
}

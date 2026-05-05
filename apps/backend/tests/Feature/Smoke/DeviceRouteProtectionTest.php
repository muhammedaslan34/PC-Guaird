<?php

namespace Tests\Feature\Smoke;

use Tests\TestCase;

class DeviceRouteProtectionTest extends TestCase
{
    public function test_device_heartbeat_requires_a_device_credential(): void
    {
        $response = $this->postJson('/api/device/heartbeat', []);

        $response->assertUnauthorized();
    }
}

<?php

namespace Tests\Feature\Smoke;

use Tests\TestCase;

class HealthCheckTest extends TestCase
{
    public function test_health_endpoint_returns_ok(): void
    {
        $response = $this->getJson('/api/health');

        $response->assertOk()->assertJson([
            'status' => 'ok',
        ]);
    }
}

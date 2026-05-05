<?php

namespace Tests\Unit\Models;

use App\Models\Device;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Tests\TestCase;

class DeviceRelationshipsTest extends TestCase
{
    public function test_device_has_many_memberships(): void
    {
        $device = new Device();

        $this->assertInstanceOf(HasMany::class, $device->memberships());
    }
}

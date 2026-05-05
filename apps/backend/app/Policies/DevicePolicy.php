<?php

namespace App\Policies;

use App\Models\Device;
use App\Models\User;

class DevicePolicy
{
    public function view(User $user, Device $device): bool
    {
        return $device->memberships()
            ->where('user_id', $user->id)
            ->exists();
    }
}

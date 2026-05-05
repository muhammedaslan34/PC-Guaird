<?php

namespace App\Http\Controllers\Api\Devices;

use App\Http\Controllers\Controller;
use App\Http\Resources\DeviceResource;
use App\Models\Device;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Gate;

class ShowDeviceController extends Controller
{
    public function __invoke(Request $request, Device $device): DeviceResource
    {
        Gate::authorize('view', $device);

        return new DeviceResource($device);
    }
}

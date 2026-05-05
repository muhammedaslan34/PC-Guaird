<?php

namespace App\Http\Controllers\Api\Devices;

use App\Http\Controllers\Controller;
use App\Http\Resources\DeviceResource;
use App\Models\Device;
use Illuminate\Http\Resources\Json\AnonymousResourceCollection;
use Illuminate\Http\Request;

class ListDevicesController extends Controller
{
    public function __invoke(Request $request): AnonymousResourceCollection
    {
        $devices = Device::query()
            ->whereHas('memberships', function ($query) use ($request): void {
                $query->where('user_id', $request->user()->id);
            })
            ->latest('id')
            ->get();

        return DeviceResource::collection($devices);
    }
}

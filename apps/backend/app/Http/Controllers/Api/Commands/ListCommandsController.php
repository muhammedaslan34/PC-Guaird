<?php

namespace App\Http\Controllers\Api\Commands;

use App\Http\Controllers\Controller;
use App\Http\Resources\CommandResource;
use App\Models\Device;
use Illuminate\Http\Request;
use Illuminate\Http\Resources\Json\AnonymousResourceCollection;
use Illuminate\Support\Facades\Gate;

class ListCommandsController extends Controller
{
    public function __invoke(Request $request, Device $device): AnonymousResourceCollection
    {
        Gate::authorize('view', $device);

        return CommandResource::collection(
            $device->commands()->latest('id')->get()
        );
    }
}

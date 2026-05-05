<?php

namespace App\Http\Controllers\Api\Audit;

use App\Http\Controllers\Controller;
use App\Http\Resources\AuditLogResource;
use App\Models\Device;
use Illuminate\Http\Request;
use Illuminate\Http\Resources\Json\AnonymousResourceCollection;
use Illuminate\Support\Facades\Gate;

class ListAuditLogsController extends Controller
{
    public function __invoke(Request $request, Device $device): AnonymousResourceCollection
    {
        Gate::authorize('view', $device);

        return AuditLogResource::collection(
            $device->auditLogs()->latest('id')->get()
        );
    }
}

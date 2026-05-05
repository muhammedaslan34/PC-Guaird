<?php

namespace App\Http\Controllers\Api\Commands;

use App\Domain\Commands\Actions\CreateCommand;
use App\Http\Controllers\Controller;
use App\Http\Requests\Commands\StoreCommandRequest;
use App\Http\Resources\CommandResource;
use App\Models\Device;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Gate;

class StoreCommandController extends Controller
{
    public function __invoke(StoreCommandRequest $request, Device $device, CreateCommand $createCommand): JsonResponse
    {
        Gate::authorize('view', $device);

        $result = $createCommand->handle($request, $device);

        if (isset($result['error'])) {
            return response()->json([
                'code' => $result['error'],
            ], JsonResponse::HTTP_UNPROCESSABLE_ENTITY);
        }

        return (new CommandResource($result['command']))
            ->response()
            ->setStatusCode(JsonResponse::HTTP_OK);
    }
}

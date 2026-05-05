<?php

namespace App\Http\Controllers\Api\Commands;

use App\Domain\Commands\Actions\CompleteCommand;
use App\Http\Controllers\Controller;
use App\Http\Requests\Commands\CompleteCommandRequest;
use App\Models\Command;
use Illuminate\Http\JsonResponse;

class CompleteCommandController extends Controller
{
    public function __invoke(CompleteCommandRequest $request, Command $command, CompleteCommand $completeCommand): JsonResponse
    {
        $device = $request->attributes->get('device');
        $command = $completeCommand->handle($request, $device, $command);

        return response()->json([
            'status' => $command->status->value,
        ]);
    }
}

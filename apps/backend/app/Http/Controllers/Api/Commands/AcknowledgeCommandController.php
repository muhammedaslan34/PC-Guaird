<?php

namespace App\Http\Controllers\Api\Commands;

use App\Domain\Commands\Actions\AcknowledgeCommand;
use App\Http\Controllers\Controller;
use App\Http\Requests\Commands\AcknowledgeCommandRequest;
use App\Models\Command;
use Illuminate\Http\JsonResponse;

class AcknowledgeCommandController extends Controller
{
    public function __invoke(AcknowledgeCommandRequest $request, Command $command, AcknowledgeCommand $acknowledgeCommand): JsonResponse
    {
        $device = $request->attributes->get('device');
        $command = $acknowledgeCommand->handle($device, $command);

        return response()->json([
            'status' => $command->status->value,
        ]);
    }
}

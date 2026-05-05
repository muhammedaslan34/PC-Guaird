<?php

namespace App\Enums;

enum CommandStatus: string
{
    case Pending = 'pending';
    case Dispatched = 'dispatched';
    case Acknowledged = 'acknowledged';
    case Succeeded = 'succeeded';
    case Failed = 'failed';
    case Rejected = 'rejected';
}

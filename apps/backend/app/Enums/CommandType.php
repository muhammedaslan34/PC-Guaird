<?php

namespace App\Enums;

enum CommandType: string
{
    case Shutdown = 'shutdown';
    case Restart = 'restart';
    case Lock = 'lock';
}

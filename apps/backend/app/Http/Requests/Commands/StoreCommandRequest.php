<?php

namespace App\Http\Requests\Commands;

use App\Enums\CommandType;
use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Validation\Rule;

class StoreCommandRequest extends FormRequest
{
    public function authorize(): bool
    {
        return true;
    }

    public function rules(): array
    {
        return [
            'command_type' => ['required', Rule::enum(CommandType::class)],
            'payload' => ['nullable', 'array'],
        ];
    }
}

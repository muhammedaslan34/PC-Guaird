<?php

namespace App\Http\Requests\Commands;

use App\Enums\CommandStatus;
use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Validation\Rule;

class CompleteCommandRequest extends FormRequest
{
    public function authorize(): bool
    {
        return true;
    }

    public function rules(): array
    {
        return [
            'status' => ['required', Rule::in([CommandStatus::Succeeded->value, CommandStatus::Failed->value])],
            'result_message' => ['nullable', 'string', 'max:255'],
            'result_code' => ['nullable', 'string', 'max:255'],
        ];
    }
}

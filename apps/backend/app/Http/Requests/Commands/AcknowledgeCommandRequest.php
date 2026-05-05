<?php

namespace App\Http\Requests\Commands;

use Illuminate\Foundation\Http\FormRequest;

class AcknowledgeCommandRequest extends FormRequest
{
    public function authorize(): bool
    {
        return true;
    }

    public function rules(): array
    {
        return [];
    }
}

$required = @(
  "package.json",
  "pnpm-workspace.yaml",
  "turbo.json",
  "apps/backend",
  "apps/dashboard",
  "apps/child-agent"
)

$missing = $required | Where-Object { -not (Test-Path $_) }
if ($missing.Count -gt 0) {
  Write-Error ("Missing: " + ($missing -join ", "))
  exit 1
}

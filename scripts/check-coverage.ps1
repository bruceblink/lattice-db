[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
$workspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$coverageManifest = Join-Path $workspaceRoot "coverage/critical-modules.toml"
$registeredPaths = Select-String -LiteralPath $coverageManifest -Pattern '^\s*path\s*=\s*"([^"]+)"' |
    ForEach-Object { $_.Matches[0].Groups[1].Value }
if ($registeredPaths.Count -eq 0) {
    Write-Error "coverage manifest has no registered paths: $coverageManifest"
    exit 1
}
foreach ($registeredPath in $registeredPaths) {
    $absolutePath = Join-Path $workspaceRoot $registeredPath.Replace('/', '\\')
    if (-not (Test-Path -LiteralPath $absolutePath -PathType Leaf)) {
        Write-Error "coverage manifest path does not exist: $registeredPath"
        exit 1
    }
}

# The CLI only assembles startup commands; its routing tests run with the workspace suite.
# The format facade only declares modules and re-exports APIs; behavior is covered below it.
$coverageArgs = @(
    "llvm-cov",
    "--workspace",
    "--exclude", "lattice-cli",
    "--all-targets",
    "--ignore-filename-regex", 'lattice-format[\\/]src[\\/]lib\.rs$',
    "--summary-only",
    "--fail-under-lines", "100",
    "--fail-under-functions", "100",
    "--fail-under-regions", "100"
)

& cargo @coverageArgs
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
$coverageArgs = @(
    "llvm-cov",
    "--workspace",
    "--exclude", "lattice-cli",
    "--all-targets",
    "--summary-only",
    "--fail-under-lines", "100",
    "--fail-under-functions", "100",
    "--fail-under-regions", "100"
)

& cargo @coverageArgs
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

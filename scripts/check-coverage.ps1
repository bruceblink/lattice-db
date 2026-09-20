[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
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

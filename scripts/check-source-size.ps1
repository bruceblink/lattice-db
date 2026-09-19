[CmdletBinding()]
param(
    [int]$MaxFileLines = 600,
    [int]$MaxFunctionLines = 120
)

$ErrorActionPreference = "Stop"
$workspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$sourceRoot = Join-Path $workspaceRoot "crates"
$violations = [System.Collections.Generic.List[string]]::new()

function Get-PhysicalLineCount {
    param([string]$Path)
    return (Get-Content -LiteralPath $Path).Count
}

function Get-FunctionViolations {
    param(
        [string]$Path,
        [int]$Limit
    )

    $lines = Get-Content -LiteralPath $Path
    $text = $lines -join "`n"
    $pattern = '(?m)^\s*(?:pub\s+)?(?:async\s+)?fn\s+\w+[^\{]*\{'
    $results = [System.Collections.Generic.List[string]]::new()

    foreach ($match in [regex]::Matches($text, $pattern)) {
        $start = ($text.Substring(0, $match.Index) -split "`n").Count
        $depth = 0
        $end = $start
        for ($index = $start - 1; $index -lt $lines.Count; $index++) {
            $depth += ([regex]::Matches($lines[$index], '\{')).Count
            $depth -= ([regex]::Matches($lines[$index], '\}')).Count
            if ($depth -le 0 -and $index -ge $start - 1) {
                $end = $index + 1
                break
            }
        }
        $length = $end - $start + 1
        if ($length -gt $Limit) {
            $results.Add("${Path}:$start-$end ($length lines)")
        }
    }

    return $results
}

$files = Get-ChildItem -LiteralPath $sourceRoot -Recurse -File -Filter *.rs
foreach ($file in $files) {
    $lineCount = Get-PhysicalLineCount -Path $file.FullName
    if ($lineCount -gt $MaxFileLines) {
        $violations.Add("file exceeds $MaxFileLines lines: $($file.FullName) ($lineCount)")
    }
    foreach ($violation in Get-FunctionViolations -Path $file.FullName -Limit $MaxFunctionLines) {
        $violations.Add("function exceeds $MaxFunctionLines lines: $violation")
    }
}

if ($violations.Count -gt 0) {
    $violations | Write-Error
    exit 1
}

Write-Output "source-size: pass ($($files.Count) Rust files)"

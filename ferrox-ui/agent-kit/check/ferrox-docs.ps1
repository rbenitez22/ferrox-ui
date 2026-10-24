#!/usr/bin/env pwsh
# ferrox-docs: prints the docs folder of the ferrox-ui an app builds against,
# so an agent reads the docs for the version it compiles. Asks Cargo
# (`cargo metadata`); ferrox-docs.spec.md is the full behaviour
# (ferrox-docs.sh is the same in bash).
#
# Usage: ferrox-docs.ps1 [folder]      (the app's folder; default: the current)
# Exit:  0 printed, 1 ferrox-ui isn't a dependency there, 2 usage or Cargo error

param([Parameter(Position = 0)][string]$Folder = '.')
$ErrorActionPreference = 'Stop'

if ($args.Count -gt 0 -or -not (Test-Path -LiteralPath $Folder -PathType Container)) {
    [Console]::Error.WriteLine('usage: ferrox-docs.ps1 [folder]')
    exit 2
}

Push-Location -LiteralPath $Folder
try {
    $json = cargo metadata --format-version 1 2>$null
    $ok = $LASTEXITCODE -eq 0
} finally {
    Pop-Location
}
if (-not $ok) {
    [Console]::Error.WriteLine("ferrox-docs: cargo metadata failed in $Folder")
    exit 2
}

$package = ($json | ConvertFrom-Json).packages | Where-Object { $_.name -eq 'ferrox-ui' } | Select-Object -First 1
$dir = if ($package) { Split-Path -Parent $package.manifest_path } else { $null }
if (-not $dir -or -not (Test-Path -LiteralPath (Join-Path $dir 'docs/index.md'))) {
    [Console]::Error.WriteLine("ferrox-docs: ferrox-ui isn't a dependency in $Folder")
    exit 1
}
"$dir/docs"

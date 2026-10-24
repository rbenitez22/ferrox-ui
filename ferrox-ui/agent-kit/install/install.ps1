#!/usr/bin/env pwsh
# install: puts ferrox-ui's agent kit into an app, where its agent finds it.
# install/targets.tsv says what goes where; install.spec.md is the full
# behaviour (install.sh is the same in bash).
#
# Usage: install.ps1 --generic [--dry-run] <app folder>
# Exit:  0 installed (or, with --dry-run, shown), 2 usage error

$ErrorActionPreference = 'Stop'

$kit = Split-Path -Parent $PSScriptRoot
# Where the kit goes in the app; {{KIT}} in its Markdown files becomes this.
$kitDir = '.ferrox'

function Exit-Usage {
    [Console]::Error.WriteLine('usage: install.ps1 --generic [--dry-run] <app folder>')
    exit 2
}

$adapter = ''
$dry = $false
$app = ''
foreach ($arg in $args) {
    switch -CaseSensitive ($arg) {
        '--generic' { $adapter = 'generic' }
        '--dry-run' { $dry = $true }
        default {
            if ($arg.StartsWith('-') -or $app) { Exit-Usage }
            $app = $arg
        }
    }
}
if (-not $adapter -or -not $app -or -not (Test-Path -LiteralPath $app -PathType Container)) { Exit-Usage }
$version = ([IO.File]::ReadAllText((Join-Path $kit 'VERSION'))).Trim()

# The plan: source file and destination, in targets order, a folder's files
# in ordinal order of their path inside it.
$plan = [System.Collections.Generic.List[object]]::new()
foreach ($raw in [IO.File]::ReadAllLines((Join-Path $kit 'install/targets.tsv'))) {
    if ($raw -eq '' -or $raw.StartsWith('#')) { continue }
    $rowAdapter, $source, $dest = $raw.Split("`t")
    if ($rowAdapter -ne $adapter) { continue }
    $sourcePath = Join-Path $kit $source
    if (Test-Path -LiteralPath $sourcePath -PathType Container) {
        $base = (Resolve-Path -LiteralPath $sourcePath).Path
        [string[]]$files = @(Get-ChildItem -LiteralPath $base -Recurse -File -Force | ForEach-Object {
            $_.FullName.Substring($base.Length + 1).Replace('\', '/')
        } | Where-Object { -not $_.StartsWith('tests/') })
        [Array]::Sort($files, [StringComparer]::Ordinal)
        foreach ($file in $files) { $plan.Add(@{ Source = Join-Path $base $file; Dest = "$dest/$file" }) }
    } else {
        $plan.Add(@{ Source = $sourcePath; Dest = $dest })
    }
}

# A kit already there is kept, as the first free .ferrox.bak, .ferrox.bak.2, …
if (Test-Path -LiteralPath (Join-Path $app $kitDir)) {
    $backup = "$kitDir.bak"
    $n = 2
    while (Test-Path -LiteralPath (Join-Path $app $backup)) {
        $backup = "$kitDir.bak.$n"
        $n++
    }
    if ($dry) {
        "would back up $kitDir to $backup"
    } else {
        Move-Item -LiteralPath (Join-Path $app $kitDir) -Destination (Join-Path $app $backup)
        "backed up $kitDir to $backup"
    }
}

foreach ($entry in $plan) {
    if ($dry) {
        "would write $($entry.Dest)"
        continue
    }
    $target = Join-Path $app $entry.Dest
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $target) | Out-Null
    if ($entry.Dest.EndsWith('.md')) {
        [IO.File]::WriteAllText($target, [IO.File]::ReadAllText($entry.Source).Replace('{{KIT}}', $kitDir))
    } else {
        Copy-Item -LiteralPath $entry.Source -Destination $target
    }
    "wrote $($entry.Dest)"
}

if ($dry) {
    'ferrox-kit: dry run, nothing written'
} else {
    "ferrox-kit: installed $version ($adapter) into $kitDir"
}

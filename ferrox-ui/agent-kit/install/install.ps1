#!/usr/bin/env pwsh
# install: puts ferrox-ui's agent kit into an app, and wrappers for the
# agents asked for. install/targets.tsv says what goes where;
# install.spec.md is the full behaviour (install.sh is the same in bash).
#
# Usage: install.ps1 <--generic|--claude|--copilot|--agents-md>... [--dry-run] <app folder>
#        install.ps1 --update [adapter flags] [--dry-run] <app folder>
# Exit:  0 done (or, with --dry-run, shown), 2 usage error

$ErrorActionPreference = 'Stop'

$kit = Split-Path -Parent $PSScriptRoot
# Where the kit goes in the app; {{KIT}} becomes this.
$kitDir = '.ferrox'
$utf8 = [Text.UTF8Encoding]::new($false)

function Exit-Usage {
    [Console]::Error.WriteLine('usage: install.ps1 <--generic|--claude|--copilot|--agents-md>... [--update] [--dry-run] <app folder>')
    exit 2
}

$flags = [System.Collections.Generic.List[string]]::new()
$update = $false
$dry = $false
$app = ''
foreach ($arg in $args) {
    switch -CaseSensitive ($arg) {
        { $_ -in '--generic', '--claude', '--copilot', '--agents-md' } { $flags.Add($arg.Substring(2)); break }
        '--update' { $update = $true; break }
        '--dry-run' { $dry = $true; break }
        default {
            if ($arg.StartsWith('-') -or $app) { Exit-Usage }
            $app = $arg
        }
    }
}
if (-not $app -or -not (Test-Path -LiteralPath $app -PathType Container)) { Exit-Usage }
if ($flags.Count -eq 0) {
    $installed = Join-Path $app "$kitDir/INSTALLED"
    if (-not $update -or -not (Test-Path -LiteralPath $installed -PathType Leaf)) { Exit-Usage }
    foreach ($line in [IO.File]::ReadAllLines($installed)) { if ($line) { $flags.Add($line) } }
}
$version = ([IO.File]::ReadAllText((Join-Path $kit 'VERSION'))).Trim()

# Rows of a tab-separated file: comments and blank lines skipped, empty
# fields kept.
function Get-Rows([string]$Path) {
    foreach ($line in [IO.File]::ReadAllLines($Path)) {
        if ($line -eq '' -or $line.StartsWith('#')) { continue }
        , $line.Split("`t")
    }
}

# The kit's pieces.
$pieces = @(Get-Rows (Join-Path $kit 'kit.tsv') | ForEach-Object { @{ Kind = $_[0]; Id = $_[1]; Path = $_[2]; Purpose = $_[3] } })

# The targets, and the adapters chosen: generic, then the others asked for,
# in the order they first appear in targets.tsv.
$targets = @(Get-Rows (Join-Path $kit 'install/targets.tsv') | ForEach-Object { @{ Adapter = $_[0]; Source = $_[1]; Dest = $_[2]; How = $_[3] } })
$chosen = [System.Collections.Generic.List[string]]::new()
$chosen.Add('generic')
foreach ($target in $targets) {
    if ($flags.Contains($target.Adapter) -and -not $chosen.Contains($target.Adapter)) { $chosen.Add($target.Adapter) }
}

# A template rendered for a piece.
function Format-Template([string]$Text, $Piece) {
    $Text.Replace('{{ID}}', $Piece.Id).Replace('{{KIND}}', $Piece.Kind).Replace('{{PATH}}', $Piece.Path).Replace('{{PURPOSE}}', $Piece.Purpose).Replace('{{KIT}}', $kitDir)
}

function Test-Kind([string]$How, [string]$Kind) { (",$($How.Substring($How.IndexOf(':') + 1)),").Contains(",$Kind,") }

# The plan: What (raw: copy Source; text: write Text; block: put Text into
# the file as a block) and Dest.
$plan = [System.Collections.Generic.List[object]]::new()
foreach ($target in $targets) {
    if (-not $chosen.Contains($target.Adapter)) { continue }
    $source = Join-Path $kit $target.Source
    if ($target.How -eq 'copy') {
        if (Test-Path -LiteralPath $source -PathType Container) {
            $base = (Resolve-Path -LiteralPath $source).Path
            [string[]]$files = @(Get-ChildItem -LiteralPath $base -Recurse -File -Force | ForEach-Object {
                $_.FullName.Substring($base.Length + 1).Replace('\', '/')
            } | Where-Object { -not $_.StartsWith('tests/') })
            [Array]::Sort($files, [StringComparer]::Ordinal)
            $pairs = $files | ForEach-Object { @{ From = Join-Path $base $_; To = "$($target.Dest)/$_" } }
        } else {
            $pairs = @(@{ From = $source; To = $target.Dest })
        }
        foreach ($pair in $pairs) {
            if ($pair.To.EndsWith('.md')) {
                $plan.Add(@{ What = 'text'; Dest = $pair.To; Text = [IO.File]::ReadAllText($pair.From).Replace('{{KIT}}', $kitDir) })
            } else {
                $plan.Add(@{ What = 'raw'; Dest = $pair.To; Source = $pair.From })
            }
        }
    } elseif ($target.How.StartsWith('each:')) {
        $template = [IO.File]::ReadAllText($source)
        foreach ($piece in $pieces) {
            if (-not (Test-Kind $target.How $piece.Kind)) { continue }
            $plan.Add(@{ What = 'text'; Dest = $target.Dest.Replace('{{ID}}', $piece.Id); Text = (Format-Template $template $piece) })
        }
    } elseif ($target.How.StartsWith('block:')) {
        $template = [IO.File]::ReadAllText($source)
        $block = "<!-- ferrox-kit: written by ferrox-ui's agent kit; install --update rewrites it -->`n"
        foreach ($piece in $pieces) {
            if (Test-Kind $target.How $piece.Kind) { $block += (Format-Template $template $piece) }
        }
        $block += '<!-- /ferrox-kit -->'
        $plan.Add(@{ What = 'block'; Dest = $target.Dest; Text = $block })
    }
}
$plan.Add(@{ What = 'text'; Dest = "$kitDir/INSTALLED"; Text = (($chosen | ForEach-Object { "$_`n" }) -join '') })

# A block put into a file's text: replacing the old block, or added at the end.
function Join-Block([string]$Old, [string]$Block) {
    $start = '<!-- ferrox-kit'
    $end = '<!-- /ferrox-kit -->'
    $at = if ($Old.StartsWith($start)) { 0 } else { $i = $Old.IndexOf("`n$start"); if ($i -ge 0) { $i + 1 } else { -1 } }
    if ($at -ge 0) {
        $close = $Old.IndexOf($end, $at)
        if ($close -ge 0) { return $Old.Substring(0, $at) + $Block + $Old.Substring($close + $end.Length) }
    }
    if ($Old -eq '') { return "$Block`n" }
    if (-not $Old.EndsWith("`n")) { $Old += "`n" }
    "$Old`n$Block`n"
}

# An entry's new content (a raw copy's is its source's).
function Get-NewContent($Entry) {
    switch ($Entry.What) {
        'raw' { [IO.File]::ReadAllText($Entry.Source) }
        'text' { $Entry.Text }
        'block' {
            $path = Join-Path $app $Entry.Dest
            $old = if (Test-Path -LiteralPath $path -PathType Leaf) { [IO.File]::ReadAllText($path) } else { '' }
            Join-Block $old $Entry.Text
        }
    }
}

# Writes an entry (a raw copy keeps its source's permissions).
function Write-Entry($Entry) {
    $target = Join-Path $app $Entry.Dest
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $target) | Out-Null
    if ($Entry.What -eq 'raw') {
        Copy-Item -LiteralPath $Entry.Source -Destination $target
    } else {
        [IO.File]::WriteAllText($target, (Get-NewContent $Entry), $utf8)
    }
}

$adapters = $chosen -join ', '

if (-not $update) {
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
        $what = if ($dry) { 'would write' } else { 'wrote' }
        if ($entry.What -eq 'block') { $what += ' block in' }
        if (-not $dry) { Write-Entry $entry }
        "$what $($entry.Dest)"
    }
    if ($dry) { 'ferrox-kit: dry run, nothing written' } else { "ferrox-kit: installed $version ($adapters) into $kitDir" }
    exit 0
}

# --update: only what changes.
$changes = 0
$planned = [System.Collections.Generic.HashSet[string]]::new()
foreach ($entry in $plan) {
    [void]$planned.Add($entry.Dest)
    $target = Join-Path $app $entry.Dest
    $new = Get-NewContent $entry
    if (-not (Test-Path -LiteralPath $target -PathType Leaf)) {
        if ($dry) { "would add $($entry.Dest)" } else { "added $($entry.Dest)" }
    } elseif ([IO.File]::ReadAllText($target) -ceq $new) {
        continue
    } else {
        if ($dry) { "would change $($entry.Dest)" } else { "changed $($entry.Dest)" }
    }
    $changes++
    if (-not $dry) { Write-Entry $entry }
}
$installedDir = Join-Path $app $kitDir
if (Test-Path -LiteralPath $installedDir -PathType Container) {
    $base = (Resolve-Path -LiteralPath $installedDir).Path
    [string[]]$files = @(Get-ChildItem -LiteralPath $base -Recurse -File -Force | ForEach-Object { $_.FullName.Substring($base.Length + 1).Replace('\', '/') })
    [Array]::Sort($files, [StringComparer]::Ordinal)
    foreach ($file in $files) {
        if ($planned.Contains("$kitDir/$file")) { continue }
        if ($dry) { "would remove $kitDir/$file" } else { "removed $kitDir/$file" }
        $changes++
        if (-not $dry) { Remove-Item -LiteralPath (Join-Path $base $file) }
    }
}
if ($dry) { "ferrox-kit: dry run, $changes change(s), nothing written" } else { "ferrox-kit: updated to $version ($adapters); $changes change(s)" }

#!/usr/bin/env pwsh
# ferrox-check: lists where an app drifts from ferrox-ui, with the fix for
# each. The rules are in rules.tsv; ferrox-check.spec.md is the full
# behaviour (ferrox-check.sh is the same check in bash).
#
# Usage: ferrox-check.ps1 [folder]     (default: the current folder)
# Exit:  0 no drift, 1 drift found, 2 usage error

param([Parameter(Position = 0)][string]$Folder = '.')
$ErrorActionPreference = 'Stop'

if ($args.Count -gt 0 -or -not (Test-Path -LiteralPath $Folder -PathType Container)) {
    [Console]::Error.WriteLine('usage: ferrox-check.ps1 [folder]')
    exit 2
}
$root = (Resolve-Path -LiteralPath $Folder).Path
$rulesPath = Join-Path $PSScriptRoot 'rules.tsv'

# The files checked: *.rs, *.css and *.html, outside build output, generated
# styles, dependencies and hidden folders. Paths relative to the folder,
# with forward slashes.
function Get-CheckedFiles([string]$Dir, [string]$Rel) {
    foreach ($entry in Get-ChildItem -LiteralPath $Dir -Force) {
        $path = if ($Rel) { "$Rel/$($entry.Name)" } else { $entry.Name }
        if ($entry.PSIsContainer) {
            if ($entry.Name -in 'target', 'dist', 'node_modules' -or $entry.Name.StartsWith('.') -or $path -match '(^|/)style/ferrox$') { continue }
            Get-CheckedFiles $entry.FullName $path
        } elseif ($entry.Name -match '\.(rs|css|html)$') {
            $path
        }
    }
}
[string[]]$files = @(Get-CheckedFiles $root '')
[Array]::Sort($files, [StringComparer]::Ordinal)

# Whether a file is one a rule applies to: "*.rs" and the like.
function Test-Applies([string]$File, [string]$Glob) { $File.EndsWith($Glob.Substring(1), [StringComparison]::Ordinal) }

$findings = [System.Collections.Generic.List[object]]::new()
$missing = [System.Collections.Generic.List[string]]::new()
$index = 0
foreach ($raw in [IO.File]::ReadAllLines($rulesPath)) {
    if ($raw -eq '' -or $raw.StartsWith('#')) { continue }
    $id, $kind, $glob, $pattern, $except, $rule, $fix = $raw.Split("`t")
    $index++
    $text = "${id}: $rule (fix: $fix)"
    if ($kind -eq 'forbid') {
        for ($f = 0; $f -lt $files.Count; $f++) {
            if (-not (Test-Applies $files[$f] $glob)) { continue }
            $lines = [IO.File]::ReadAllLines((Join-Path $root $files[$f]))
            for ($i = 0; $i -lt $lines.Count; $i++) {
                $current = $lines[$i]
                if (-not [regex]::IsMatch($current, $pattern)) { continue }
                if ($except -and [regex]::IsMatch($current, $except)) { continue }
                $previous = if ($i -gt 0) { $lines[$i - 1] } else { '' }
                if (("$current$previous").Contains("ferrox-check: allow $id")) { continue }
                $findings.Add([pscustomobject]@{ File = $f; Line = $i + 1; Rule = $index; Text = "$($files[$f]):$($i + 1): $text" })
            }
        }
    } elseif (Test-Path -LiteralPath (Join-Path $root 'Cargo.toml')) {
        # Require rules check a whole app: only at its root.
        $some = $false
        $found = $false
        foreach ($file in $files) {
            if (-not (Test-Applies $file $glob)) { continue }
            $some = $true
            # Line by line, as grep does, so ^ and $ mean the same in both.
            foreach ($line in [IO.File]::ReadAllLines((Join-Path $root $file))) {
                if ([regex]::IsMatch($line, $pattern)) { $found = $true; break }
            }
            if ($found) { break }
        }
        if ($some -and -not $found) { $missing.Add("${glob}: $text") }
    }
}

foreach ($finding in ($findings | Sort-Object File, Line, Rule)) { $finding.Text }
foreach ($m in $missing) { $m }
$count = $findings.Count + $missing.Count
if ($count -eq 0) {
    'ferrox-check: no drift'
    exit 0
}
"ferrox-check: $count problem(s)"
exit 1

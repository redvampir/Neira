# neira:meta
# id: NEI-20250904-check-duplicates-ps1
# intent: ci
# summary: PowerShell-скрипт для локальной проверки дублей зависимостей с игнором Windows/WASI семейств.
# neira:meta
# id: NEI-20250905-group-duplicates
# intent: ci
# summary: Группирует дубли по crate и выводит только блоки с несколькими версиями; синхронизирован regex allowlist.

param()
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Invoke-CargoTree {
  try {
    $pinfo = New-Object System.Diagnostics.ProcessStartInfo
    $pinfo.FileName = 'cargo'
    $pinfo.Arguments = 'tree -d --target all'
    $pinfo.RedirectStandardOutput = $true
    $pinfo.RedirectStandardError = $true
    $pinfo.UseShellExecute = $false
    $p = New-Object System.Diagnostics.Process
    $p.StartInfo = $pinfo
    $null = $p.Start()
    $stdout = $p.StandardOutput.ReadToEnd()
    $stderr = $p.StandardError.ReadToEnd()
    $p.WaitForExit()
    if ($p.ExitCode -ne 0) {
      if ($stderr) { Write-Error $stderr }
      exit $p.ExitCode
    }
    return $stdout
  } catch {
    Write-Error $_
    exit 1
  }
}

$raw = Invoke-CargoTree
if ($raw -match '^nothing to print\s*$') {
  exit 0
}

# Разбиваем вывод на абзацы и группируем по имени crate
$paragraphs = $raw -split "(\r?\n){2,}" | Where-Object { $_.Trim().Length -gt 0 }
$groups = @{}
foreach ($para in $paragraphs) {
  $lines = $para -split "\r?\n"
  $first = $lines[0].Trim()
  if (-not $first) { continue }
  $parts = $first -split '\s+'
  if ($parts.Count -lt 2) { continue }
  $name = $parts[0]
  $ver = $parts[1]
  # Ignore known Windows/WASI families (transitive duplicates upstream)
  if ($name -match '^(wasi|windows(|-sys|-core|-targets)|windows_[A-Za-z0-9_]+)$') { continue }
  if (-not $groups.ContainsKey($name)) {
    $groups[$name] = @{
      Versions = [System.Collections.Generic.HashSet[string]]::new()
      Blocks   = New-Object System.Collections.Generic.List[string]
    }
  }
  $null = $groups[$name].Versions.Add($ver)
  $null = $groups[$name].Blocks.Add($para)
}

$toOutput = @()
foreach ($entry in $groups.GetEnumerator()) {
  if ($entry.Value.Versions.Count -gt 1) {
    $toOutput += $entry.Value.Blocks
  }
}

if ($toOutput.Count -gt 0) {
  $toOutput -join "`n`n" | Write-Output
  Write-Error 'Duplicate crate versions detected (excluding Windows/WASI families).'
  exit 1
}
exit 0

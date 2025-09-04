# neira:meta
# id: NEI-20250904-check-duplicates-ps1
# intent: ci
# summary: PowerShell-скрипт для локальной проверки дублей зависимостей с игнором Windows/WASI семейств.

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

# Разбиваем на абзацы по пустой строке
$paragraphs = $raw -split "(\r?\n){2,}" | Where-Object { $_.Trim().Length -gt 0 }
$filtered = @()
foreach ($para in $paragraphs) {
  $first = ($para -split "\r?\n")[0].Trim()
  if (-not $first) { continue }
  $name = ($first -split '\s+')[0]
  # Ignore known Windows/WASI families (transitive duplicates upstream)
  if ($name -match '^(wasi|windows([_-].*)?|windows_[A-Za-z0-9_]+)$') { continue }
  $filtered += $para
}

if ($filtered.Count -gt 0) {
  $filtered -join "`n`n" | Write-Output
  Write-Error 'Duplicate crate versions detected (excluding Windows/WASI families).'
  exit 1
}
exit 0

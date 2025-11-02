# neira:meta
# id: NEI-20240709-184500-gui-launcher
# intent: feature
# summary: |
#   Р”РѕР±Р°РІРёР» WinForms-Р»Р°СѓРЅС‡РµСЂ РґР»СЏ РќРµР№СЂС‹: РІС‹Р±РѕСЂ РѕРґРЅРѕРіРѕ/РЅРµСЃРєРѕР»СЊРєРёС… РїРѕР»СЊР·РѕРІР°С‚РµР»РµР№, Р·Р°РїСѓСЃРє, РѕСЃС‚Р°РЅРѕРІРєР° Рё СЃСЃС‹Р»РєРё РЅР° РёРЅС‚РµСЂС„РµР№СЃ/Р»РѕРіРё.

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

[System.Windows.Forms.Application]::EnableVisualStyles()

$root = (Resolve-Path (Join-Path $PSScriptRoot '..' '..')).Path
$exePath = Join-Path $root 'target\release\neira.exe'
$logPath = Join-Path $root 'logs\neira.log'
$buildLogPath = Join-Path $root 'logs\launcher-build.log'

$script:serverProcess = $null
$script:exitEvent = $null

function Ensure-Directories {
    foreach ($folder in @('data', 'logs', 'static')) {
        $path = Join-Path $root $folder
        if (-not (Test-Path $path)) {
            New-Item -ItemType Directory -Path $path | Out-Null
        }
    }
}

function Set-Status([string]$text, $color = [System.Drawing.Color]::White) {
    $statusLabel.Text = $text
    $statusLabel.ForeColor = $color
}

function Ensure-Build {
    Set-Status "РЎР±РѕСЂРєР° РїСЂРѕРµРєС‚Р°... (Р¶СѓСЂРЅР°Р»: $buildLogPath)", [System.Drawing.Color]::Orange
    $launcherOutput.Text = "Р’С‹РїРѕР»РЅСЏРµС‚СЃСЏ cargo build --release`r`n"
    $launcherOutput.Refresh()

    Ensure-Directories
    $startInfo = New-Object System.Diagnostics.ProcessStartInfo
    $startInfo.FileName = 'cargo'
    $startInfo.Arguments = 'build --release'
    $startInfo.WorkingDirectory = $root
    $startInfo.UseShellExecute = $false
    $startInfo.CreateNoWindow = $true
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true

    try {
        $process = [System.Diagnostics.Process]::Start($startInfo)
    } catch {
        Set-Status "РќРµ СѓРґР°Р»РѕСЃСЊ Р·Р°РїСѓСЃС‚РёС‚СЊ cargo. РџСЂРѕРІРµСЂСЊС‚Рµ СѓСЃС‚Р°РЅРѕРІРєСѓ Rust.", [System.Drawing.Color]::Red
        $launcherOutput.Text = $_.Exception.Message
        return $false
    }
    $stdout = $process.StandardOutput.ReadToEnd()
    $stderr = $process.StandardError.ReadToEnd()
    $process.WaitForExit()

    $stdout + $stderr | Out-File -FilePath $buildLogPath -Encoding UTF8
    $launcherOutput.Text = ($stdout + $stderr)

    if ($process.ExitCode -ne 0) {
        Set-Status "РћС€РёР±РєР° СЃР±РѕСЂРєРё (РєРѕРґ $($process.ExitCode)). РЎРј. $buildLogPath", [System.Drawing.Color]::Red
        return $false
    }

    Set-Status "РЎР±РѕСЂРєР° Р·Р°РІРµСЂС€РµРЅР° СѓСЃРїРµС€РЅРѕ.", [System.Drawing.Color]::LightGreen
    return $true
}

function Start-Neira([string]$bindAddr) {
    if ($script:serverProcess -and -not $script:serverProcess.HasExited) {
        [System.Windows.Forms.MessageBox]::Show("РЎРµСЂРІРµСЂ СѓР¶Рµ Р·Р°РїСѓС‰РµРЅ.", "Neira", 'OK', 'Information') | Out-Null
        return
    }

    $startButton.Enabled = $false
    $stopButton.Enabled = $false
    $hintLabel.Text = ""

    Ensure-Directories

    if (-not (Ensure-Build)) {
        $startButton.Enabled = $true
        return
    }

    Set-Status "Р—Р°РїСѓСЃРєР°РµРј СЃРµСЂРІРµСЂ (СЂРµР¶РёРј $bindAddr)...", [System.Drawing.Color]::Orange

    $startInfo = New-Object System.Diagnostics.ProcessStartInfo
    $startInfo.FileName = $exePath
    $startInfo.WorkingDirectory = $root
    $startInfo.UseShellExecute = $false
    $startInfo.CreateNoWindow = $true
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true
    $startInfo.EnvironmentVariables['RUST_LOG'] = 'info'
    $startInfo.EnvironmentVariables['NEIRA_DATA_DIR'] = '.\data'
    $startInfo.EnvironmentVariables['NEIRA_SUCCESS_THRESHOLD'] = '0.8'
    $startInfo.EnvironmentVariables['NEIRA_BIND_ADDR'] = $bindAddr

    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $startInfo
    $process.EnableRaisingEvents = $true
    if ($script:exitEvent) {
        Unregister-Event -SubscriptionId $script:exitEvent.Id -ErrorAction SilentlyContinue
        $script:exitEvent = $null
    }
    $subscription = Register-ObjectEvent -InputObject $process -EventName Exited -Action {
        $form.BeginInvoke({
            Set-Status "РЎРµСЂРІРµСЂ РѕСЃС‚Р°РЅРѕРІР»РµРЅ.", [System.Drawing.Color]::Tomato
            $startButton.Enabled = $true
            $stopButton.Enabled = $false
        }) | Out-Null
    }
    $script:exitEvent = $subscription

    try {
        $null = $process.Start()
    } catch {
        if ($script:exitEvent) {
            Unregister-Event -SubscriptionId $script:exitEvent.Id -ErrorAction SilentlyContinue
            $script:exitEvent = $null
        }
        Set-Status "РќРµ СѓРґР°Р»РѕСЃСЊ Р·Р°РїСѓСЃС‚РёС‚СЊ neira.exe. РџСЂРѕРІРµСЂСЊС‚Рµ СЃР±РѕСЂРєСѓ.", [System.Drawing.Color]::Red
        $launcherOutput.Text = $_.Exception.Message
        $startButton.Enabled = $true
        return
    }
    $script:serverProcess = $process
    $stopButton.Enabled = $true
    $startButton.Enabled = $false
    Set-Status "РЎРµСЂРІРµСЂ Р·Р°РїСѓС‰РµРЅ. Р›РѕРіРё: $logPath", [System.Drawing.Color]::LightGreen
    $launcherOutput.Text = "РЎРµСЂРІРµСЂ Р·Р°РїСѓС‰РµРЅ. Р’С‹РІРѕРґ РЅР°РїСЂР°РІР»РµРЅ РІ logs\neira.log`r`n"

    if ($bindAddr -like '0.0.0.0:*') {
        try {
            $tailscale = Get-TailscaleAddress
            if ($tailscale) {
                $hintLabel.Text = "РђРґСЂРµСЃ Tailscale: http://$tailscale"
            } else {
                $hintLabel.Text = "РџРѕРґСЃС‚Р°РІСЊС‚Рµ СЃРІРѕР№ РІРЅРµС€РЅРёР№/VPN IP РІРјРµСЃС‚Рѕ <ip>: http://<ip>:9090"
            }
        } catch {
            $hintLabel.Text = "РџРѕРґСЃС‚Р°РІСЊС‚Рµ СЃРІРѕР№ РІРЅРµС€РЅРёР№/VPN IP: http://<ip>:9090"
        }
    } else {
        $hintLabel.Text = "Р›РѕРєР°Р»СЊРЅС‹Р№ РґРѕСЃС‚СѓРї: http://localhost:9090"
    }
}

function Stop-Neira {
    if ($script:serverProcess -and -not $script:serverProcess.HasExited) {
        try {
            $script:serverProcess.Kill()
            $script:serverProcess.WaitForExit()
        } catch {
            # ignore
        }
    }
    $script:serverProcess = $null
    if ($script:exitEvent) {
        Unregister-Event -SubscriptionId $script:exitEvent.Id -ErrorAction SilentlyContinue
        $script:exitEvent = $null
    }
    $startButton.Enabled = $true
    $stopButton.Enabled = $false
    Set-Status "РЎРµСЂРІРµСЂ РѕСЃС‚Р°РЅРѕРІР»РµРЅ.", [System.Drawing.Color]::Tomato
    $hintLabel.Text = ""
}

function Get-TailscaleAddress {
    $tailscaleExe = (Get-Command tailscale -ErrorAction SilentlyContinue)
    if (-not $tailscaleExe) {
        return $null
    }
    $info = New-Object System.Diagnostics.ProcessStartInfo
    $info.FileName = $tailscaleExe.Source
    $info.Arguments = 'ip -4'
    $info.UseShellExecute = $false
    $info.CreateNoWindow = $true
    $info.RedirectStandardOutput = $true
    $info.RedirectStandardError = $true
    $proc = [System.Diagnostics.Process]::Start($info)
    $output = $proc.StandardOutput.ReadToEnd()
    $proc.WaitForExit()
    $ip = $output.Trim().Split([Environment]::NewLine, [StringSplitOptions]::RemoveEmptyEntries) | Select-Object -First 1
    if ([string]::IsNullOrWhiteSpace($ip)) {
        return $null
    }
    return "$ip:9090"
}

$form = New-Object System.Windows.Forms.Form
$form.Text = "Neira Launcher"
$form.Size = New-Object System.Drawing.Size(520, 420)
$form.StartPosition = 'CenterScreen'

$modeGroup = New-Object System.Windows.Forms.GroupBox
$modeGroup.Text = "Р РµР¶РёРј СЂР°Р±РѕС‚С‹"
$modeGroup.Location = New-Object System.Drawing.Point(15, 15)
$modeGroup.Size = New-Object System.Drawing.Size(220, 100)

$radioLocal = New-Object System.Windows.Forms.RadioButton
$radioLocal.Text = "Р”Р»СЏ РѕРґРЅРѕРіРѕ (127.0.0.1)"
$radioLocal.Location = New-Object System.Drawing.Point(15, 25)
$radioLocal.Checked = $true

$radioRemote = New-Object System.Windows.Forms.RadioButton
$radioRemote.Text = "Р”Р»СЏ РЅРµСЃРєРѕР»СЊРєРёС… (0.0.0.0)"
$radioRemote.Location = New-Object System.Drawing.Point(15, 55)

$modeGroup.Controls.Add($radioLocal)
$modeGroup.Controls.Add($radioRemote)

$startButton = New-Object System.Windows.Forms.Button
$startButton.Text = "Р—Р°РїСѓСЃС‚РёС‚СЊ"
$startButton.Size = New-Object System.Drawing.Size(120, 35)
$startButton.Location = New-Object System.Drawing.Point(260, 25)

$stopButton = New-Object System.Windows.Forms.Button
$stopButton.Text = "РћСЃС‚Р°РЅРѕРІРёС‚СЊ"
$stopButton.Size = New-Object System.Drawing.Size(120, 35)
$stopButton.Location = New-Object System.Drawing.Point(260, 70)
$stopButton.Enabled = $false

$openUiButton = New-Object System.Windows.Forms.Button
$openUiButton.Text = "РћС‚РєСЂС‹С‚СЊ РёРЅС‚РµСЂС„РµР№СЃ"
$openUiButton.Size = New-Object System.Drawing.Size(160, 35)
$openUiButton.Location = New-Object System.Drawing.Point(390, 25)

$openLogsButton = New-Object System.Windows.Forms.Button
$openLogsButton.Text = "РћС‚РєСЂС‹С‚СЊ Р»РѕРі"
$openLogsButton.Size = New-Object System.Drawing.Size(160, 35)
$openLogsButton.Location = New-Object System.Drawing.Point(390, 70)

$statusLabel = New-Object System.Windows.Forms.Label
$statusLabel.AutoSize = $false
$statusLabel.Size = New-Object System.Drawing.Size(480, 20)
$statusLabel.Location = New-Object System.Drawing.Point(15, 125)
$statusLabel.Text = "РЎРµСЂРІРµСЂ РЅРµ Р·Р°РїСѓС‰РµРЅ."

$hintLabel = New-Object System.Windows.Forms.Label
$hintLabel.AutoSize = $false
$hintLabel.Size = New-Object System.Drawing.Size(480, 20)
$hintLabel.Location = New-Object System.Drawing.Point(15, 150)
$hintLabel.Text = ""
$hintLabel.ForeColor = [System.Drawing.Color]::LightGray

$launcherOutput = New-Object System.Windows.Forms.TextBox
$launcherOutput.Multiline = $true
$launcherOutput.ScrollBars = 'Vertical'
$launcherOutput.Location = New-Object System.Drawing.Point(15, 180)
$launcherOutput.Size = New-Object System.Drawing.Size(480, 180)
$launcherOutput.ReadOnly = $true
$launcherOutput.BackColor = [System.Drawing.Color]::Black
$launcherOutput.ForeColor = [System.Drawing.Color]::LightGreen
$launcherOutput.Font = New-Object System.Drawing.Font('Consolas', 9)

$form.Controls.AddRange(@(
    $modeGroup,
    $startButton,
    $stopButton,
    $openUiButton,
    $openLogsButton,
    $statusLabel,
    $hintLabel,
    $launcherOutput
))

$startButton.Add_Click({
    $bindAddr = if ($radioRemote.Checked) { '0.0.0.0:9090' } else { '127.0.0.1:9090' }
    Start-Neira $bindAddr
})

$stopButton.Add_Click({
    Stop-Neira
})

$openUiButton.Add_Click({
    Start-Process "http://localhost:9090" | Out-Null
})

$openLogsButton.Add_Click({
    if (-not (Test-Path $logPath)) {
        [System.Windows.Forms.MessageBox]::Show("Р¤Р°Р№Р» Р»РѕРіР° РµС‰С‘ РЅРµ СЃРѕР·РґР°РЅ.", "Neira", 'OK', 'Information') | Out-Null
        return
    }
    Start-Process notepad.exe $logPath | Out-Null
})

$form.Add_FormClosing({
    Stop-Neira
})

Set-Status "РЎРµСЂРІРµСЂ РЅРµ Р·Р°РїСѓС‰РµРЅ."

[System.Windows.Forms.Application]::Run($form)


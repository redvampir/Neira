# Monitor Test #2 - Learning & Self-Improvement
# Checks Consciousness API and files every 2 minutes

$startTime = Get-Date
$duration = 10 # minutes
$checkInterval = 120 # seconds (2 minutes)
$endTime = $startTime.AddMinutes($duration)

Write-Host "`n╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  MONITORING TEST #2: NEIRA LEARNING CAPABILITIES          ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

Write-Host "`nStart: $($startTime.ToString('HH:mm:ss'))" -ForegroundColor Yellow
Write-Host "End:   $($endTime.ToString('HH:mm:ss'))" -ForegroundColor Yellow
Write-Host "Duration: $duration minutes`n" -ForegroundColor Yellow

# Store baseline
$baseline = @{
    stats = Invoke-RestMethod -Uri "http://localhost:9090/api/neira/consciousness/stats"
    dashboardSize = (Get-Item "static/consciousness/dashboard.js").Length
    indexSize = (Get-Item "static/consciousness/index.html").Length
    cssSize = (Get-Item "static/consciousness/dashboard.css").Length
    timestamp = Get-Date
}

Write-Host "Baseline captured:" -ForegroundColor Green
Write-Host "  thoughts_recorded: $($baseline.stats.total_thoughts_recorded)" -ForegroundColor Gray
Write-Host "  improvement_tasks: $($baseline.stats.total_improvement_tasks)" -ForegroundColor Gray
Write-Host "  biases_detected:   $($baseline.stats.total_biases_detected)" -ForegroundColor Gray

$checkCount = 0

while ((Get-Date) -lt $endTime) {
    $checkCount++
    $timeLeft = ($endTime - (Get-Date)).TotalMinutes
    
    Write-Host "`n--- Check #$checkCount (Time left: $([math]::Round($timeLeft, 1))m) ---" -ForegroundColor Cyan
    
    try {
        # Check API stats
        $current = Invoke-RestMethod -Uri "http://localhost:9090/api/neira/consciousness/stats"
        
        # Compare with baseline
        $thoughtsDiff = $current.total_thoughts_recorded - $baseline.stats.total_thoughts_recorded
        $tasksDiff = $current.total_improvement_tasks - $baseline.stats.total_improvement_tasks
        $biasesDiff = $current.total_biases_detected - $baseline.stats.total_biases_detected
        
        if ($thoughtsDiff -gt 0 -or $tasksDiff -gt 0 -or $biasesDiff -gt 0) {
            Write-Host "  [CHANGE DETECTED]" -ForegroundColor Yellow
            if ($thoughtsDiff -gt 0) { Write-Host "    +$thoughtsDiff thoughts" -ForegroundColor Green }
            if ($tasksDiff -gt 0) { Write-Host "    +$tasksDiff tasks" -ForegroundColor Green }
            if ($biasesDiff -gt 0) { Write-Host "    +$biasesDiff biases" -ForegroundColor Green }
        } else {
            Write-Host "  API: No changes" -ForegroundColor Gray
        }
        
        # Check file sizes
        $currentDashboard = (Get-Item "static/consciousness/dashboard.js").Length
        $currentIndex = (Get-Item "static/consciousness/index.html").Length
        $currentCss = (Get-Item "static/consciousness/dashboard.css").Length
        
        if ($currentDashboard -ne $baseline.dashboardSize -or 
            $currentIndex -ne $baseline.indexSize -or 
            $currentCss -ne $baseline.cssSize) {
            Write-Host "  [FILE CHANGES DETECTED]" -ForegroundColor Yellow
            if ($currentDashboard -ne $baseline.dashboardSize) {
                $diff = $currentDashboard - $baseline.dashboardSize
                Write-Host "    dashboard.js: $diff bytes" -ForegroundColor Green
            }
            if ($currentIndex -ne $baseline.indexSize) {
                $diff = $currentIndex - $baseline.indexSize
                Write-Host "    index.html: $diff bytes" -ForegroundColor Green
            }
            if ($currentCss -ne $baseline.cssSize) {
                $diff = $currentCss - $baseline.cssSize
                Write-Host "    dashboard.css: $diff bytes" -ForegroundColor Green
            }
        } else {
            Write-Host "  Files: No changes" -ForegroundColor Gray
        }
        
        # Check for new files in docs/
        $newDocs = Get-ChildItem -Path "docs/" -Filter "*learning*" -ErrorAction SilentlyContinue
        if ($newDocs) {
            Write-Host "  [NEW DOCS FOUND]" -ForegroundColor Yellow
            foreach ($doc in $newDocs) {
                Write-Host "    $($doc.Name)" -ForegroundColor Green
            }
        }
        
    } catch {
        Write-Host "  [ERROR] $($_.Exception.Message)" -ForegroundColor Red
    }
    
    # Wait before next check (unless it's the last iteration)
    if ((Get-Date) -lt $endTime.AddSeconds(-$checkInterval)) {
        Write-Host "  Waiting $checkInterval seconds..." -ForegroundColor DarkGray
        Start-Sleep -Seconds $checkInterval
    } else {
        break
    }
}

Write-Host "`n╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  MONITORING COMPLETE                                       ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

# Final comparison
Write-Host "`nFinal Check:" -ForegroundColor Yellow
$final = Invoke-RestMethod -Uri "http://localhost:9090/api/neira/consciousness/stats"
Write-Host "  Thoughts: $($baseline.stats.total_thoughts_recorded) -> $($final.total_thoughts_recorded)" -ForegroundColor White
Write-Host "  Tasks:    $($baseline.stats.total_improvement_tasks) -> $($final.total_improvement_tasks)" -ForegroundColor White
Write-Host "  Biases:   $($baseline.stats.total_biases_detected) -> $($final.total_biases_detected)" -ForegroundColor White

Write-Host "`nTotal checks performed: $checkCount" -ForegroundColor Gray
Write-Host "Duration: $([math]::Round((New-TimeSpan -Start $startTime -End (Get-Date)).TotalMinutes, 1)) minutes" -ForegroundColor Gray

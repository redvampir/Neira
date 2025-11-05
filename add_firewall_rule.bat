@echo off
REM Neira Backend - Add Firewall Rule for Port 9090
echo ========================================
echo   NEIRA FIREWALL RULE SETUP
echo ========================================
echo.
echo This will add Windows Firewall rule to allow
echo incoming connections to Neira backend on port 9090
echo.
echo You need to run this as Administrator!
echo.
pause

powershell -Command "New-NetFirewallRule -DisplayName 'Neira Backend Port 9090' -Direction Inbound -LocalPort 9090 -Protocol TCP -Action Allow -Profile Private,Domain"

if %errorlevel% equ 0 (
    echo.
    echo [SUCCESS] Firewall rule created successfully!
    echo.
    echo Now you can access Neira from smartphone:
    echo   http://192.168.0.24:9090/consciousness/
    echo.
) else (
    echo.
    echo [ERROR] Failed to create firewall rule.
    echo Make sure you run this script as Administrator:
    echo   Right-click -> Run as Administrator
    echo.
)

pause

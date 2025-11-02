@echo off
chcp 65001 > nul

echo [Нейра] Очистка проекта...

:: Останавливаем процесс
taskkill /F /IM neira.exe /T 2>nul
timeout /t 2 /nobreak > nul

:: Очищаем сборку
echo [Нейра] Удаление target...
rd /s /q target 2>nul
rd /s /q spinal_cord\backend\target 2>nul

:: Очищаем временные файлы
echo [Нейра] Удаление временных файлов...
del /f /q Cargo.lock 2>nul

echo [Нейра] Очистка завершена.
pause

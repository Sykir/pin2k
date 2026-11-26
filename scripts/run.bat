@echo off
setlocal EnableExtensions
chcp 65001 >nul
cd /d "%~dp0"
cd ../

if not exist "target\release\pin2k-desktop.exe" (
    echo [ERROR] Binary not found: target\release\pin2k-desktop.exe
    echo Run "scripts\build.bat" first to compile the project.
    pause
    exit /b 1
)

set "PORT=8080"
if not "%~1"=="" set "PORT=%~1"

echo Starting pin2k-desktop at http://127.0.0.1:%PORT% ...
start "" "http://127.0.0.1:%PORT%"

if exist "work\swe1_assets\gfx_hd_2x-DeGif_4x" (
    ".\target\release\pin2k-desktop.exe" --assets-dir .\work\swe1_assets --hd-dir .\work\swe1_assets\gfx_hd_2x-DeGif_4x --hd-scale 4 --aa --window --web-port %PORT% --frames 0
) else if exist "work\swe1_assets\gfx_hd" (
    ".\target\release\pin2k-desktop.exe" --assets-dir .\work\swe1_assets --hd --hd-scale 4 --aa --window --web-port %PORT% --frames 0
) else if exist "work\swe1_assets" (
    ".\target\release\pin2k-desktop.exe" --assets-dir .\work\swe1_assets --window --web-port %PORT% --frames 0
) else (
    echo [WARNING] Folder "work\swe1_assets" not found. Launching without extracted assets.
    ".\target\release\pin2k-desktop.exe" --window --web-port %PORT% --frames 0
)

if errorlevel 1 (
    echo.
    echo [ERROR] pin2k-desktop exited with code %ERRORLEVEL%.
    pause
    exit /b %ERRORLEVEL%
)
exit /b 0

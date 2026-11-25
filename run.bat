@echo off
setlocal EnableExtensions
chcp 65001 >nul
cd /d "%~dp0"

if not exist "target\release\pin2k-desktop.exe" (
    echo [ERREUR] Binaire introuvable : target\release\pin2k-desktop.exe
    echo Lancez d'abord "build.bat" pour compiler le projet.
    pause
    exit /b 1
)

set "PORT=8080"
if not "%~1"=="" set "PORT=%~1"

echo Demarrage de pin2k-desktop sur http://127.0.0.1:%PORT% ...
start "" "http://127.0.0.1:%PORT%"

if exist "work\swe1_assets" (
    ".\target\release\pin2k-desktop.exe" --assets-dir .\work\swe1_assets --web-port %PORT% --frames 0
) else (
    echo [ATTENTION] Dossier "work\swe1_assets" introuvable. Lancement sans assets extraits.
    ".\target\release\pin2k-desktop.exe" --web-port %PORT% --frames 0
)

if errorlevel 1 (
    echo.
    echo [ERREUR] pin2k-desktop s'est arrete avec le code %ERRORLEVEL%.
    pause
    exit /b %ERRORLEVEL%
)
exit /b 0

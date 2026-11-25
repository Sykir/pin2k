@echo off
setlocal EnableExtensions
chcp 65001 >nul
cd /d "%~dp0"

set "DO_EXTRACT=0"
set "NO_PAUSE=0"

:parse_args
if "%~1"=="" goto args_done
if /I "%~1"=="--extract" set "DO_EXTRACT=1"
if /I "%~1"=="--no-pause" set "NO_PAUSE=1"
shift
goto parse_args
:args_done

echo === [1/2] Compilation release du workspace pin2k ===
cargo build --release --workspace
if errorlevel 1 (
    echo.
    echo [ERREUR] Echec de la compilation Cargo.
    if "%NO_PAUSE%"=="0" pause
    exit /b 1
)

if "%DO_EXTRACT%"=="0" (
    if exist "roms\swe1_u100.rom" (
        if not exist "work\swe1_assets\scripts\game_script.json" (
            set "DO_EXTRACT=1"
        )
    )
)

if "%DO_EXTRACT%"=="1" (
    echo.
    echo === [2/2] Reconstruction ROM, verification, extraction assets et gamecode ===
    ".\target\release\pin2k-rom-tool.exe" reconstruct --input-dir .\roms --prefix swe1_ --output .\work\swe1_prism_flat.bin --dcs-output .\work\swe1_dcs_flat.bin
    if errorlevel 1 goto extract_err

    ".\target\release\pin2k-rom-tool.exe" verify --input .\work\swe1_prism_flat.bin
    if errorlevel 1 goto extract_err

    ".\target\release\pin2k-rom-tool.exe" extract-assets --prism-flat .\work\swe1_prism_flat.bin --sound-bin .\roms\swe1_sound.bin --output-dir .\work\swe1_assets
    if errorlevel 1 goto extract_err

    ".\target\release\pin2k-rom-tool.exe" extract-gamecode --prism-flat .\work\swe1_prism_flat.bin --output-dir .\work\swe1_assets\scripts
    if errorlevel 1 goto extract_err
)

echo.
echo [OK] Build termine avec succes.
echo Lancez "run.bat" pour demarrer le runtime.
if "%NO_PAUSE%"=="0" pause
exit /b 0

:extract_err
echo.
echo [ERREUR] Echec lors de la reconstruction ou de l'extraction des ressources.
if "%NO_PAUSE%"=="0" pause
exit /b 1

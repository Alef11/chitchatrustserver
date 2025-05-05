@echo off
setlocal enabledelayedexpansion

:: Check if .env exists
if exist .env (
    echo .env file already exists.
    choice /m "Do you want to overwrite it?"
    if errorlevel 2 (
        echo Aborting without changes.
        goto :eof
    )
)

:: Prompt the user for required values
set /p MARIADB_ROOT_PASSWORD=Enter MARIADB_ROOT_PASSWORD: 
set /p MARIADB_USER=Enter MARIADB_USER: 
set /p MARIADB_PASSWORD=Enter MARIADB_PASSWORD: 
set /p PUBLIC_DOMAIN=Enter PUBLIC_DOMAIN (e.g., example.com): 

:: Prompt for optional values with defaults
set /p MARIADB_HOST=Enter MARIADB_HOST [default: localhost]: 
if "!MARIADB_HOST!"=="" set MARIADB_HOST=localhost

set /p MARIADB_PORT=Enter MARIADB_PORT [default: 3306]: 
if "!MARIADB_PORT!"=="" set MARIADB_PORT=3306

set /p MARIADB_DATABASE=Enter MARIADB_DATABASE [default: chitchat_db]: 
if "!MARIADB_DATABASE!"=="" set MARIADB_DATABASE=chitchat_db

:: Write to .env file
(
    echo MARIADB_ROOT_PASSWORD=%MARIADB_ROOT_PASSWORD%
    echo MARIADB_USER=%MARIADB_USER%
    echo MARIADB_PASSWORD=%MARIADB_PASSWORD%
    echo MARIADB_HOST=%MARIADB_HOST%
    echo MARIADB_PORT=%MARIADB_PORT%
    echo MARIADB_DATABASE=%MARIADB_DATABASE%
    echo PUBLIC_DOMAIN=%PUBLIC_DOMAIN%
) > .env

echo .env file created successfully. ✅

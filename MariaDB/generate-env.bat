@echo off
setlocal

:: Check if .env exists
if exist .env (
    echo .env file already exists.
    choice /m "Do you want to overwrite it?"
    if errorlevel 2 (
        echo Aborting without changes.
        goto :eof
    )
)

:: Prompt the user for values
set /p MARIADB_ROOT_PASSWORD=Enter MARIADB_ROOT_PASSWORD:
set /p MARIADB_USER=Enter MARIADB_USER:
set /p MARIADB_PASSWORD=Enter MARIADB_PASSWORD:

:: Write to .env file
(
    echo MARIADB_ROOT_PASSWORD=%MARIADB_ROOT_PASSWORD%
    echo MARIADB_USER=%MARIADB_USER%
    echo MARIADB_PASSWORD=%MARIADB_PASSWORD%
) > .env

echo .env file created successfully.

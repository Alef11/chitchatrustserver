@echo off
setlocal

:: Prompt the user for each variable
set /p MARIADB_ROOT_PASSWORD=Enter MARIADB_ROOT_PASSWORD:
set /p MARIADB_USER=Enter MARIADB_USER:
set /p MARIADB_PASSWORD=Enter MARIADB_PASSWORD:

:: Create the .env file
(
  echo MARIADB_ROOT_PASSWORD=%MARIADB_ROOT_PASSWORD%
  echo MARIADB_USER=%MARIADB_USER%
  echo MARIADB_PASSWORD=%MARIADB_PASSWORD%
) > .env

echo .env file created successfully.

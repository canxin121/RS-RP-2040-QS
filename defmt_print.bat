@echo off
setlocal enabledelayedexpansion
set /a count=0
:loop
defmt-print -e %~1 tcp --port 6666
if %errorlevel% neq 0 (
    set /a count+=1
    echo Failed to connect to OpenOCD RTT tcp server, retrying in 2 seconds
    timeout /t 2 /nobreak
    cls
    goto loop
)
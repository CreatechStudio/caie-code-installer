@echo off
REM 设置在出现非零退出代码时停止脚本执行
setlocal EnableDelayedExpansion

REM 获取当前执行目录
for /f "tokens=3" %%i in ('whoami') do set "username=%%i"
set "current_dir=C:\Users\%username%\.cpc"

REM 删除旧目录，准备下载最新版本
rmdir /s /q "%current_dir%" 2>nul
mkdir "%current_dir%"

REM 检查与GitHub连通性
powershell -command "Test-NetConnection api.github.com -Port 443 | Select-Object -ExpandProperty TcpTestSucceeded"
if %errorlevel% equ 0 (
    set "repo_url=https://github.com/iewnfod/CAIE_Code.git"
) else (
    set "repo_url=https://gitee.com/ricky-tap/CAIE_Code.git"
)

REM 从选定的远程仓库下载最新版本
git clone --depth=1 %repo_url% "%current_dir%" || exit /b 1

REM git config 设置
git config --global --add safe.directory "%current_dir%" || exit /b 2

REM 权限设置
takeown /f "%current_dir%" /r /d y >nul 2>&1
icacls "%current_dir%" /grant %username%:F /t >nul 2>&1

REM 检查处理器架构
IF "%PROCESSOR_ARCHITECTURE%"=="ARM64" (
    mklink /h "C:\Users\%username%\AppData\Local\Microsoft\WindowsApps\cpc" "%current_dir%\bin\cpc_arm.exe" || exit /b 4
) ELSE (
    IF "%PROCESSOR_ARCHITEW6432%"=="ARM64" (
        mklink /h "C:\Users\%username%\AppData\Local\Microsoft\WindowsApps\cpc" "%current_dir%\bin\cpc_arm.exe" || exit /b 4
    ) ELSE (
        mklink /h "C:\Users\%username%\AppData\Local\Microsoft\WindowsApps\cpc" "%current_dir%\bin\cpc.exe" || exit /b 4
    )
)

exit /b 0

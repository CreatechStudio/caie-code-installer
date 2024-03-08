@echo off
for /f "tokens=2* delims= " %%f IN ('reg query HKCU\Environment /v PATH ^| findstr /i path') do set oldVal=%%g
setx PATH "%oldVal%;%USERPROFILE%\CAIE_Code\bin"

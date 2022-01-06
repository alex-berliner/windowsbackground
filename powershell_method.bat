@echo off
@pushd "%~dp0"
mkdir C:\temp
start /min powershell -WindowStyle Hidden -Command "& { target\debug\windback.exe }"
@popd

@echo off
@pushd "%~dp0"
mkdir C:\temp
start /min /b target\debug\windback.exe
@popd

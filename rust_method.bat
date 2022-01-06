@echo off
@pushd "%~dp0"
mkdir C:\temp
start /b /min target\debug\windback.exe -i
@popd

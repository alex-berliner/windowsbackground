@echo off
mkdir C:\temp
start /min powershell -WindowStyle Hidden -Command "& { target\debug\windback.exe }"

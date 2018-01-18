@echo off
set CARGO_CFG_TARGET_FEATURE=sse,crt-static

set arg1=%1
set arg2=%*

echo please have cargo installed


If arg1 == "build" GOTO build
If arg1 == "rbuild" GOTO rbuild
If arg1 == "run" GOTO run
If arg1 == "rrun" GOTO rrun
If arg1 == "install" GOTO install
If arg1 == "unintsall" GOTO uninstall
If arg1 == "help" GOTO help

:build
	cargo build 
	GOTO exit_P
:rbuild
	cargo build --release
	GOTO exit_P
:run
	cargo run
	GOTO exit_P
:rrun
	cargo run --release
	GOTO exit_P

:install
	mkdir "%ProgramFiles%\cache_cleaner\bin"
	echo If ran with admin privlages log files are placed in "%ProgramFiles%\cache_cleaner\config"
	mkdir "%ProgramFiles%\cache_cleaner\config"
	cargo install --root "%ProgramFiles%\cache_cleaner\bin" 

	copy /y src/config/cache_cleaner_empty.conf  "%ProgramFiles%\cache_cleaner\config\cache_cleaner.conf"

	echo
	echo
	echo Please configure the files you want to be deleted. "%ProgramFiles%\cache_cleaner\config\cache_cleaner.conf"
	echo Or "C:\USERS\<you user name>\cache_cleaner\cache_cleaner.conf"

	GOTO exit_P

:uninstall
	del "%ProgramFiles%\cache_cleaner"
	GOTO exit_P

:help
	GOTO exit_P

:exit_P
rem this exits the program
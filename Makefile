export CARGO_CFG_TARGET_FEATURE=sse,crt-static

build:
	cargo build
rbuild:
	cargo build --release

run:
	cargo run

rrun:
	cargo run --release

install:
	echo "===========================Must be ran as Root==========================="
	echo "Root needs to have access to rustc and  cargo"
	echo 
	
	cargo install --root /usr/
	mkdir -p /etc/cache_cleaner/
	cp src/config/cache_cleaner_empty.conf -apvr /etc/cache_cleaner/cache_cleaner.conf

	echo
	echo
	echo "Please configure the files you want to be deleted. /etc/cache_cleaner/cache_cleaner.conf"
	echo "Or ~/.cache_cleaner/cache_cleaner.conf"

	echo "If ran as root log files are placed in /var/cache_cleaner"
	mkdir -p /var/cache_cleaner

uninstall:
	echo "run as root"
	rm /usr/bin/cache_cleaner
	rm /var/cache_cleaner -rf
	rm /etc/cache_cleaner -rf

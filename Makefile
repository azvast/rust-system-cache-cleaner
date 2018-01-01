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
	# cargo build --target=x86_64-unknown-linux-musl

	cargo install --root /usr/ 

	mkdir -p /etc/cache_cleaner/
	cp src/config/cache_cleaner_empty.conf -apvr /etc/cache_cleaner/cache_cleaner.conf

	echo
	echo
	echo "Please configure the files you want to be deleted. /etc/cache_cleaner/cache_cleaner.conf"
	echo "Or ~/.cache_cleaner/cache_cleaner.conf"

	echo "If ran as root log files are placed in /var/cache_cleaner"
	mkdir -p /var/cache_cleaner
	cp src/config/cache_cleaner.service /etc/systemd/system/cache_cleaner.service

uninstall:
	rm /usr/bin/cache_cleaner
	rm /var/cache_cleaner -rf
	rm /etc/cache_cleaner -rf
	rm /etc/systemd/system/cache_cleaner.service

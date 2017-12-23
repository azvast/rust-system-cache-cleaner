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
	mkdir -p /etc/clear_cache/
	cp src/config/clear_cache_empty.conf -apvr /etc/clear_cache/clear_cache.confs

	echo
	echo
	echo "Please configure the files you want to be deleted. /etc/clear_cache/clear_cache.conf"
	echo "Or ~/.clear_cache/clear_cache.conf"



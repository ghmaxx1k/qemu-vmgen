install:
	cargo build --release
	mv target/release/qemu-vmgen /usr/local/bin/qemu-vmgen
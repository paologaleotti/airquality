build:
	cargo build

build-relase:
	cargo build --release

flash: 
	cargo embed

flash-release: 
	cargo embed --release


clean: 
	cargo clean

.PHONY: flash clean
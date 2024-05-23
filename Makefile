build:
	cargo build

build-relase:
	cargo build --release

flash: 
	cargo embed --chip STM32F103C8

flash-release: 
	cargo embed --release --chip STM32F103C8


clean: 
	cargo clean

.PHONY: flash clean
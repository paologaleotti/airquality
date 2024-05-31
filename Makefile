build:
	cargo build --release

flash: 
	cargo embed --release

clean: 
	cargo clean

attach:
	probe-rs attach --probe 0483:3748:C3A1 --chip STM32F103C8 target/thumbv7m-none-eabi/release/airquality

.PHONY: flash clean
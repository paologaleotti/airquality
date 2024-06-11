![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

# airquality

Fully open source (hardware and firmware) cheap and fairly accurate air quality monitor.

<img src="https://github.com/paologaleotti/airquality/assets/45665769/886df23f-7fe9-4707-bbe8-d8e5c07bd646" alt="image" width="500"/>

It can measure:

- Temperature
- Humidity
- Pressure
- CO2 (eCO2)
- TVOC

It displays data to a small OLED display via a simple UI.

Hardware:

- STM32 board (`STM32f103c8` ARM based)
- `ENS160` sensor (air quality, CO2)
- `AHT21` sensor (temperature, humidity, pressure)
- `ssd1306` (OLED display)

It costs around 10 euros to buy all the required hardware
(it may require soldering the headers onto the STM32)

## Hardware

The circuit uses a single shared I2C Bus with OLED and the two sensors connected (SCL, SDA).

![circuit schematic](https://github.com/paologaleotti/airquality/assets/45665769/f12d57ee-04ac-4db3-83a6-750d398b02e5)

Note: in my case the ENS160 and AHT2x sensors were together on the same PCB.
All the electronics in my case can work with 5V, be sure to check the datasheet of your own sensors before connecting to VCC!

## Firmware

The firmware is written in Rust (embbedded, no_std) using all the embedded Rust tooling and ecocsystem.

Requirements:

- Rust
- Make
- [probe-rs](https://probe.rs/)
- Rust target `thumbv7m-none-eabi`
- STLink (or any other supported probe)

To build and flash the firmware:

```bash
make flash
```

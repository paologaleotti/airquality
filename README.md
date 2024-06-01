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

- STM32 board (`STM32f103c8` "blue pill")
- `ENS160` sensor (air quality, CO2)
- `AHT21` sensor (temperature, humidity, pressure)
- `ssd1306` (OLED display)

It costs around 10 euros to buy all the required hardware
(it may require soldering the headers onto the STM32)

## Build and flash

Requirements:

- Rust
- Make
- [probe-rs](https://probe.rs/)
- Rust target `thumbv7m-none-eabi`
- STLink (or any other supported probe)

To flash the firmware:

```bash
make flash
```

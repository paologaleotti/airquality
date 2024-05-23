# airquality

Airquality is a fully open source (hardware and firmware) cheap and fairly accurate air quality monitor.

It can measure:

- Temperature
- Humidity
- Pressure
- CO2 (eCO2)
- TVOC

It displays data to a small OLED display via a simple UI.

Hardware:

- STM32 board (STM32f103c8)
- ENS160 sensor (air quality)
- AHT21 sensor (temperature, humidity, pressure)
- ssd1306 (OLED display)

It costs around 10 euros to buy all the required hardware
(it may require soldering the headers onto the STM32)

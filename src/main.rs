#![no_std]
#![no_main]

mod consts;
mod ui;
mod utils;

use consts::ENS160_ADDR;
use cortex_m_rt::entry;
use ens160::{AirqualityIndex, Ens160};
use format_no_std::show;
use panic_rtt_core::{self, rprintln, rtt_init_print};
use ssd1306::{
    mode::DisplayConfig, rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface,
    Ssd1306,
};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
};
use ui::{UiEngine, UiLine};
use utils::{airquality_to_string, validity_to_string};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain();
    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
    let mut led_yellow = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);
    let mut led_red = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);

    let mut delay = cp.SYST.delay(&clocks);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.Hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    let bus = shared_bus::BusManagerSimple::new(i2c);

    let interface = I2CDisplayInterface::new(bus.acquire_i2c());
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    display.flush().unwrap();

    let mut ui = UiEngine::new(display);

    rprintln!("init start");

    ui.print_centered("starting up...");

    let mut ens160 = Ens160::new(bus.acquire_i2c(), ENS160_ADDR);
    delay.delay_ms(500_u16);
    ens160.operational().unwrap();
    delay.delay_ms(500_u16);

    loop {
        delay.delay_ms(1000_u16);
        ui.flush();
        ui.clear();

        let quality = ens160.airquality_index().unwrap();
        let ens160_status = ens160.status().unwrap();
        let temp_hum = ens160.temp_and_hum().unwrap();
        let eco2 = ens160.eco2().unwrap();
        let tvoc = ens160.tvoc().unwrap();

        rprintln!("status {:?}", ens160_status);

        ui.print_text(
            validity_to_string(ens160_status.validity_flag()),
            Some("STAT"),
            UiLine::L1,
        );
        ui.print_text(airquality_to_string(quality), Some("AIR"), UiLine::L2);

        let mut buf = [0u8; 10];
        let eco2_str = show(&mut buf, format_args!("{:?}ppm", *eco2)).unwrap();
        ui.print_text(eco2_str, Some("ECO2"), UiLine::L3);

        let mut buf = [0u8; 10];
        let tvoc_str = show(&mut buf, format_args!("{:?}ppb", tvoc)).unwrap();
        ui.print_text(tvoc_str, Some("TVOC"), UiLine::L4);

        match quality {
            AirqualityIndex::Excellent | AirqualityIndex::Good => {
                led_red.set_low();
                led_yellow.set_low()
            }
            AirqualityIndex::Moderate => {
                led_yellow.set_high();
                led_red.set_low()
            }
            AirqualityIndex::Poor | AirqualityIndex::Unhealthy => {
                led_red.set_high();
                led_yellow.set_low()
            }
        }

        rprintln!("quality: {:?}", quality);
        rprintln!("eco2: {:?}", eco2);
        rprintln!("tvoc: {:?}", tvoc);
        rprintln!("temp hum: {:?}", temp_hum);
    }
}

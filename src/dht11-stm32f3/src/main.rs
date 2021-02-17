#![no_std]
#![no_main]

use crate::hal::{delay, gpio, prelude::*, stm32};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f3xx_hal as hal;
use stm32f3xx_hal::rcc::RccExt;

use hal::pac;

use dht_sensor::*;
// use dht11::Dht11;

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = stm32::CorePeripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    // let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(168.mhz()).freeze(&mut flash.acr);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // This is used by `dht-sensor` to wait for signals
    let mut delay = delay::Delay::new(cp.SYST, clocks);


    // This could be any `gpio` port
    // let gpio::gpioa::Parts { pa1, .. } = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    // The DHT11 datasheet suggests 1 second
    hprintln!("Waiting on the sensor...").unwrap();
    delay.delay_ms(1000_u16);

    // An `Output<OpenDrain>` is both `InputPin` and `OutputPin`
    let mut pa1 = gpioa.pa1.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    // let mut pa1 = cortex_m::interrupt::free(|_| gpioa.pa1.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper));
    hprintln!("pa1 open drained").unwrap();

    // let mut dht11 = Dht11::new(pa1);

    // match dht11.perform_measurement(&mut delay) {
    //     Ok(meas) => hprintln!("Temp: {} Hum: {}", meas.temperature, meas.humidity).unwrap(),
    //     Err(e) => hprintln!("Error: {:?}", e).unwrap(),
    // };

    match dht11::Reading::read(&mut delay, &mut pa1) {
        Ok(dht11::Reading {
            temperature,
            relative_humidity,
        }) => hprintln!("{}°, {}% RH", temperature, relative_humidity).unwrap(),
        Err(e) => hprintln!("Error {:?}", e).unwrap(),
    }
    // let readings = dht11::Reading::read(&mut delay, &mut pa1);
    // hprintln!("readings: {:?}", readings).unwrap();
    hprintln!("Looping forever now, thanks!").unwrap();

    loop {}
}

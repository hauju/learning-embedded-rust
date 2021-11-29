#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut rcc = dp.RCC.constrain();

    // Acquire the GPIO peripheral
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let button = gpioc.pc13.into_pull_down_input(&mut gpioc.crh);
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    loop {
        if button.is_low().unwrap() {
            led.set_low().unwrap();
        }

        if button.is_high().unwrap() {
            led.set_high().unwrap();
        }
    }
}

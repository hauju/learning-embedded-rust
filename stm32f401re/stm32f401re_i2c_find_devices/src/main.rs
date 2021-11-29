#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{i2c::I2c, prelude::*, stm32};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    // Acquire the GPIO peripheral
    let mut gpiob = dp.GPIOB.split();

    // Acquire the I2C peripheral
    let scl = gpiob.pb8.into_open_drain_output();
    let sda = gpiob.pb9.into_open_drain_output();

    // Configure I2C
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 400.khz(), clocks);

    rtt_init_print!();
    rprintln!("Scanning for I2C addresses...");

    let mut buffer = [0u8; 1];
    // I2C addresses are 7-bit wide, covering the 0-127 range
    for add in 0..=127 {
        // The write method sends the specified address and checks for acknowledgement;
        if i2c.write_read(add, &[0x00], &mut buffer).is_ok() {
            rprintln!("Address: {:#X}", add);
        }
    }
    rprintln!("Done!");

    loop {}
}

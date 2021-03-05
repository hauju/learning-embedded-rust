#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIO peripheral
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Acquire the I2C peripheral
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    // Configure I2C
    let mut i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

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

    loop {}
}

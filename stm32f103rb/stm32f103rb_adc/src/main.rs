#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use nb::block;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{adc, pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = pac::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);
    let mut timer = Timer::tim2(p.TIM2, &clocks, &mut rcc.apb1).start_count_down(1.hz());

    rtt_init_print!();
    rprintln!("sysclk freq: {}", clocks.sysclk().0);
    rprintln!("adc freq: {}", clocks.adcclk().0);

    // Setup ADC
    let mut adc1 = adc::Adc::adc1(p.ADC1, &mut rcc.apb2, clocks);

    // Setup GPIOA
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

    // Configure pb0, pb1 as an analog input
    let mut ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);

    loop {
        let data: f64 = adc1.read(&mut ch0).unwrap();
        rprintln!("adc1: {}", data);
        block!(timer.wait()).unwrap();
    }
}

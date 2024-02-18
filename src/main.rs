//! An empty baseline project.

#![no_main]
#![no_std]

use panic_semihosting as _;

use stm32f1xx_hal as hal;

#[allow(unused_imports)]
use hal::prelude::*;

use hal::pac;

use cortex_m_rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    #[allow(unused_variables, unused_mut)]
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);

    #[allow(unused_variables, unused_mut)]
    let mut afio = dp.AFIO.constrain();

    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.set_high();
        delay.delay_ms(250u32);
        led.set_low();
        delay.delay_ms(250u32);
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

//! Read from a TTP229BSF keypad manually (i.e. not using an SPI interface).
//!
//! When the button is pressed, the state of the keypad is printed to openocd
//! via semihosting.
//!
//! ## µC Connections
//!
//! - A TTP229BSF (_not_ LSF) keypad with SCL at µC pin B6 & SDO at µC pin B7
//! - A button that pulls down to ground when pressed at µC pin B11

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_rt::exception;
use cortex_m_rt::ExceptionFrame;
use cortex_m_semihosting::hprintln;
use hal::pac;
use hal::prelude::*;
use hal::timer::Timer;
use nb::block;
use panic_semihosting as _;
use stm32f1xx_hal as hal;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();

    let mut scl = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
    let sdo = gpiob.pb7.into_floating_input(&mut gpiob.crl);

    let btn = gpiob.pb11.into_pull_up_input(&mut gpiob.crh);

    // const DV: u16 = 93;
    const TW: u16 = /* 10 < */ 1000;
    // const TOUT: u16 = 2_000;
    // const TRESP: u16 = 32_000;

    const FSCL: u32 = /* 1k < */ 10_000 /* < 512k */;
    const FSCL_DELAY: u32 = 1_000_000 / (2 * FSCL);

    const TW_IN_FSCL: u32 = (TW as u32) / FSCL_DELAY;

    // let mut delay = cp.SYST.delay(&clocks);

    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start((2 * FSCL).Hz()).unwrap();

    loop {
        let mut keys = [false; 16];
        // let mut keys = 0;
        for k in &mut keys {
            // for i in 0..16 {
            scl.set_low();
            // delay.delay_us(FSCL_DELAY);
            block!(timer.wait()).unwrap();
            scl.set_high();
            *k = sdo.is_low();
            // keys |= (sdo.is_low() as u16) << i;
            // delay.delay_us(FSCL_DELAY);
            block!(timer.wait()).unwrap();
        }
        // delay.delay_us(TW);

        for _ in 0..TW_IN_FSCL {
            block!(timer.wait()).unwrap();
        }

        if btn.is_low() {
            let mut ks = 0u16;
            for (i, k) in keys.into_iter().enumerate() {
                ks |= (k as u16) << i;
            }
            hprintln!("{:016b}", ks);
        }
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

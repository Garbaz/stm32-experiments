//! Doesn't really do anything, just experimenting with I2C.

#![no_std]
#![no_main]

// use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use cortex_m_rt::exception;
use cortex_m_rt::ExceptionFrame;
use cortex_m_semihosting::hprintln;
use hal::pac;
use hal::prelude::*;
use panic_semihosting as _;
use stm32f1xx_hal as hal;

// const TTP229_ADDR: u8 = 0b1010110;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();

    let mut gpiob = dp.GPIOB.split();
    let mut afio = dp.AFIO.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);
    // rcc
    //     .cfgr
    //     .use_hse(8.MHz())
    //     .sysclk(48.MHz())
    //     .pclk1(4.MHz())
    //     // .pclk2(4.MHz())
    //     .freeze(&mut flash.acr);
    // rcc
    // .cfgr
    // .use_hse(8.MHz())
    // .sysclk(48.MHz())
    // .pclk1(6.MHz())
    // .freeze(&mut flash.acr);

    cp.DWT.enable_cycle_counter();

    let mut i2c = {
        let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
        let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);
        hal::i2c::BlockingI2c::i2c1(
            dp.I2C1,
            (scl, sda),
            &mut afio.mapr,
            hal::i2c::Mode::Fast {
                frequency: 400.kHz(),
                duty_cycle: hal::i2c::DutyCycle::Ratio16to9,
            },
            clocks,
            10000,
            10,
            10000,
            10000,
        )
    };

    // hprintln!("Reading...");

    // let data = {
    //     let mut buffer = [0xaa; 2];
    //     i2c.read(0x57, &mut buffer).unwrap();
    //     (buffer[0] as u16) << 8 | (buffer[1] as u16)
    // };

    // hprintln!("Result: {}", data);

    loop {
        hprintln!("Reading...");

        let data = {
            let mut buffer = [0, 0];
            let _ = i2c.read(0x57, &mut buffer);
            (buffer[0] as u16) << 8 | (buffer[1] as u16)
        };

        hprintln!("Result: {}", data);
        // wfi();
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

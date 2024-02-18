//! Read from a TTP229BSF keypad and display the pressed button on a seven
//! segment display.
//!
//! Uses a single SPI connection to communicate with both the keypad and the
//! shift register for the display.
//!
//! When the button is pressed, the state of the keypad is printed to openocd
//! via semihosting.
//!
//! ## µC Connections
//!
//! - A TTP229BSF (_not_ LSF) keypad with SCL at µC pin B13 & SDO at µC pin B14
//! - A SN74HC595 shift register with RCLK at µC pin B13 & SER at µC pin B15
//! - A button that pulls down to ground when pressed at µC pin B11
//!
//! ## Peripheral Connections
//!
//! - The shift register's pins Qa to Qg are connected to a common cathode seven
//!   segment display's pins A to G
//! - The shift register's pin RCLK is connected through a not gate (i.e. a
//!   transistor and two resistors) to the pin SRCLK
//! - Everything else is connected the obvious way

#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
// use cortex_m_semihosting::hprint;
use cortex_m_semihosting::hprintln;
use hal::{
    pac,
    prelude::*,
    spi::{self},
};
use nb::block;
use panic_semihosting as _;
use stm32f1xx_hal as hal;
// use ttp229::Key;
// use ttp229::TTP229;

#[entry]
fn main() -> ! {
    #[allow(unused_variables)]
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);

    // let mut afio = dp.AFIO.constrain();

    // let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    let btn = gpiob.pb11.into_pull_up_input(&mut gpiob.crh);

    // let mut spi_7seg = spi::Spi::spi1(
    //     dp.SPI1,
    //     (
    //         gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl),
    //         spi::NoMiso,
    //         gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl),
    //     ),
    //     &mut afio.mapr,
    //     spi::Mode {
    //         polarity: spi::Polarity::IdleLow,
    //         phase: spi::Phase::CaptureOnFirstTransition,
    //     },
    //     100.kHz(),
    //     clocks,
    // );

    let mut spi_keys = spi::Spi::spi2(
        dp.SPI2,
        (
            gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
            gpiob.pb14.into_floating_input(&mut gpiob.crh),
            gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
        ),
        spi::Mode {
            polarity: spi::Polarity::IdleHigh,
            phase: spi::Phase::CaptureOnSecondTransition,
        },
        100.kHz(),
        clocks,
    )
    .frame_size_16bit();

    let mut delay = cp.SYST.delay(&clocks);

    let mut prev_keys = 0;

    let mut disp_state = 0;

    loop {
        block!(spi_keys.send(disp_state)).unwrap();
        let keys = block!(spi_keys.read()).unwrap();
        // let keys = spi_keys.read_data_reg();

        if btn.is_low() {
            hprintln!("{:016b}", keys);
        }

        if keys != prev_keys {
            for i in 0..16 {
                if keys & (1 << i) == 0 {
                    // hprintln!("{}", 15-i);
                    // spi_keys.write_data_reg(NUM_TO_SEGMENTS[(16 - i) % 16].reverse_bits() as u16);
                    disp_state = NUM_TO_SEGMENTS[(16 - i) % 16].reverse_bits() as u16;
                    // block!(spi_keys.read()).unwrap();
                    break;
                }
            }
        }
        prev_keys = keys;
        delay.delay_ms(20u16);
    }
}

/// For every (hexadecimal) digit from '0' to 'F', which segments have to be
/// enabled and which disabled
const NUM_TO_SEGMENTS: [u8; 16] = [
    /*0*/ 0b11111100, /*1*/ 0b01100000, /*2*/ 0b11011010, /*3*/ 0b11110010,
    /*4*/ 0b01100110, /*5*/ 0b10110110, /*6*/ 0b10111110, /*7*/ 0b11100000,
    /*8*/ 0b11111110, /*9*/ 0b11110110, /*A*/ 0b11101110, /*B*/ 0b00111110,
    /*C*/ 0b10011100, /*D*/ 0b01111010, /*E*/ 0b10011110, /*F*/ 0b10001110,
];

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

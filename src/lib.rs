#![no_std]

pub mod shape3d;
pub mod mpu;

use hal::{
    afio::MAPR,
    gpio::{Cr, Pin},
    i2c::{BlockingI2c, DutyCycle, Error, Mode},
    pac::{I2C1, I2C2},
    prelude::*,
    rcc::Clocks,
};
use stm32f1xx_hal as hal;

const START_TIMEOUT_US: u32 = 100000;
const START_RETRIES: u8 = 10;
const ADDR_TIMEOUT_US: u32 = 10000;
const DATA_TIMEOUT_US: u32 = 10000;

/// A minor convenience function for creating blocking i2c1, wrapping
/// [`stm32f1xx_hal::i2c::BlockingI2c::i2c1`].
///
/// ## Example
///
/// ```rs
/// #[cortex_m_rt::entry]
/// fn main() -> ! {
///     let mut cp = cortex_m::Peripherals::take().unwrap();
///     let dp = stm32f1xx_hal::pac::Peripherals::take().unwrap();
///     
///     let mut flash = dp.FLASH.constrain();
///     let rcc = dp.RCC.constrain();
///     
///     let clocks = rcc
///         .cfgr
///         .use_hse(8.MHz())
///         .sysclk(48.MHz())
///         .freeze(&mut flash.acr);
///     
///     cp.DCB.enable_trace();
///     cp.DWT.enable_cycle_counter();
///     
///     let mut afio = dp.AFIO.constrain();
///     
///     let mut gpiob = dp.GPIOB.split();
///     
///     let i2c = i2c1(
///             &clocks,
///             dp.I2C1,
///             gpiob.pb6,
///             gpiob.pb7,
///             &mut gpiob.crl,
///             &mut afio.mapr,
///         )
/// }
/// ```
pub fn i2c1(
    clocks: &Clocks,
    dp_i2c1: I2C1,
    gpiob_pb6: Pin<'B', 6>,
    gpiob_pb7: Pin<'B', 7>,
    gpiob_crl: &mut Cr<'B', false>,
    afio_mapr: &mut MAPR,
) -> impl _embedded_hal_blocking_i2c_Write<Error = Error>
       + _embedded_hal_blocking_i2c_WriteRead<Error = Error> {
    let scl = gpiob_pb6.into_alternate_open_drain(gpiob_crl);
    let sda = gpiob_pb7.into_alternate_open_drain(gpiob_crl);

    BlockingI2c::i2c1(
        dp_i2c1,
        (scl, sda),
        afio_mapr,
        Mode::Fast {
            frequency: 400_000.Hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        *clocks,
        START_TIMEOUT_US,
        START_RETRIES,
        ADDR_TIMEOUT_US,
        DATA_TIMEOUT_US,
    )
}

/// A minor convenience function for creating blocking i2c2, wrapping
/// [`stm32f1xx_hal::i2c::BlockingI2c::i2c2`].
///
/// ## Example
///
/// ```rs
/// #[cortex_m_rt::entry]
/// fn main() -> ! {
///     let mut cp = cortex_m::Peripherals::take().unwrap();
///     let dp = stm32f1xx_hal::pac::Peripherals::take().unwrap();
///     
///     let mut flash = dp.FLASH.constrain();
///     let rcc = dp.RCC.constrain();
///     
///     let clocks = rcc
///         .cfgr
///         .use_hse(8.MHz())
///         .sysclk(48.MHz())
///         .freeze(&mut flash.acr);
///     
///     cp.DCB.enable_trace();
///     cp.DWT.enable_cycle_counter();
///     
///     let mut afio = dp.AFIO.constrain();
///     
///     let mut gpiob = dp.GPIOB.split();
///     
///     let i2c = i2c2(
///             &clocks,
///             dp.I2C2,
///             gpiob.pb10,
///             gpiob.pb11,
///             &mut gpiob.crh,
///         )
/// }
/// ```
pub fn i2c2(
    clocks: &Clocks,
    dp_i2c2: I2C2,
    gpiob_pb10: Pin<'B', 10>,
    gpiob_pb11: Pin<'B', 11>,
    gpiob_crh: &mut Cr<'B', true>,
) -> impl _embedded_hal_blocking_i2c_Write<Error = Error>
       + _embedded_hal_blocking_i2c_WriteRead<Error = Error> {
    let scl = gpiob_pb10.into_alternate_open_drain(gpiob_crh);
    let sda = gpiob_pb11.into_alternate_open_drain(gpiob_crh);

    BlockingI2c::i2c2(
        dp_i2c2,
        (scl, sda),
        // &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.Hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        *clocks,
        START_TIMEOUT_US,
        START_RETRIES,
        ADDR_TIMEOUT_US,
        DATA_TIMEOUT_US,
    )
}

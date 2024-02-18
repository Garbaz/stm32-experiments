#![no_std]

use core::{fmt, str};

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


/// A fixed size (no `std` and no allocation) buffer to write to with
/// [`core::fmt::write`]. Useful for creating a formatted `&str` at runtime
/// without allocation. 
/// 
/// ## Example
/// 
/// ```rs
/// let mut wb: WriteBuffer<128> = WriteBuffer::new();
/// writeln!(wb, "Hello World!").unwrap();
/// 
/// assert_eq!(wb.as_str(), "Hello World!\n");
/// ```
pub struct WriteBuffer<const N: usize> {
    buffer: [u8; N],
    cursor: usize,
}

impl<const N: usize> WriteBuffer<N> {
    pub fn new() -> Self {
        Self {
            buffer: [0; N],
            cursor: 0,
        }
    }

    /// Reset the buffer's cursor back to 0.
    pub fn clear(&mut self) {
        self.cursor = 0;
    }
    
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buffer[..self.cursor]) }
    }

}

impl<const N: usize> fmt::Write for WriteBuffer<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let s = s.as_bytes();
        let end = self.cursor + s.len();

        if end >= N {
            return Err(fmt::Error);
        }

        self.buffer[self.cursor..end].copy_from_slice(s);

        self.cursor = end;

        Ok(())
    }
}

impl<const N: usize> Default for WriteBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

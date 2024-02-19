//! Read data from an MPU6050 gyro/accel/temp sensor chip and display it on an
//! SSD1306 mini oled display.
//!
//! ## µC Connections
//!
//! - An SSD1306 with SCL at µC pin B6 & SDA at µC pin B7
//! - An MPU6050 with SCL at µC pin B10 & SDA at µC pin B11

#![no_main]
#![no_std]

use core::fmt::Write as _;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    geometry::Point,
    mono_font::{iso_8859_1, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::PrimitiveStyleBuilder,
    text::{Baseline, Text},
    Drawable,
};
use hal::pac;
#[allow(unused_imports)]
use hal::prelude::*;
use heapless::String;
use mpu6050::Mpu6050;
use nalgebra::Point3;
use panic_semihosting as _;
use ssd1306::{
    prelude::*, rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306,
};
use stm32_experiments::{
    i2c1, i2c2,
    shape3d::{ARROW, CUBOID},
};
use stm32f1xx_hal as hal;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .freeze(&mut flash.acr);

    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();

    let mut afio = dp.AFIO.constrain();

    let mut gpiob = dp.GPIOB.split();

    let mut mpu = Mpu6050::new(i2c2(
        &clocks,
        dp.I2C2,
        gpiob.pb10,
        gpiob.pb11,
        &mut gpiob.crh,
    ));
    let mut delay = cp.SYST.delay(&clocks);
    mpu.init(&mut delay).unwrap();

    let mut display = Ssd1306::new(
        I2CDisplayInterface::new(i2c1(
            &clocks,
            dp.I2C1,
            gpiob.pb6,
            gpiob.pb7,
            &mut gpiob.crl,
            &mut afio.mapr,
        )),
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();
    display.init().unwrap();

    let line_style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    let text_style = MonoTextStyleBuilder::new()
        .font(&iso_8859_1::FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    loop {
        display.clear(BinaryColor::Off).unwrap();

        let acc = mpu.get_acc().unwrap();

        CUBOID.draw(&mut display, line_style, &Point3::origin(), &acc.into());
        ARROW.draw(&mut display, line_style, &Point3::origin(), &(-acc).into());

        let temp = mpu.get_temp().unwrap();

        {
            let mut buffer: String<64> = String::new();
            write!(buffer, "{:.1}°C", temp).unwrap();

            Text::with_baseline(buffer.as_str(), Point::new(1, 1), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();
        }

        display.flush().unwrap();
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

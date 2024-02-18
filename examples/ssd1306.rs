//! Adapted from
//! [ssd1306/examples](https://github.com/jamwaffles/ssd1306/tree/master/examples).
//!
//! ## µC Connections
//!
//! - An SSD1306 with SCL at µC pin B6 & SDA at µC pin B7

#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{iso_8859_1, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
    text::{Baseline, Text},
};
use hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
};
use panic_semihosting as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
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

    let mut afio = dp.AFIO.constrain();

    let mut gpiob = dp.GPIOB.split();

    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();

    // let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    // let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.Hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let yoffset = 32;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // screen outline default display size is 128x64 if you don't pass a
    // _DisplaySize_ enum to the _Builder_ struct
    let outline = Rectangle::new(Point::new(0, 0), Size::new(127, 63)).into_styled(style);

    // triangle
    let triangle = Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(style);

    let rectangle = Rectangle::new(Point::new(52, yoffset), Size::new_equal(16)).into_styled(style);

    let circle = Circle::new(Point::new(88, yoffset), 16).into_styled(style);

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../rust32.raw"), 32);

    let mut logo = Image::new(&raw, Point::new(0, 0));

    let text_style = MonoTextStyleBuilder::new()
        .font(&iso_8859_1::FONT_8X13)
        .text_color(BinaryColor::On)
        .build();

    let hello_rust = Text::with_baseline(
        "Hallöchen!",
        Point::new(24, 16),
        text_style,
        Baseline::Middle,
    );

    let mut draw = |logo_x, logo_y| {
        display.clear(BinaryColor::Off).unwrap();

        logo.translate_mut(Point::new(logo_x, logo_y));
        logo.draw(&mut display).unwrap();
        logo.translate_mut(Point::new(-logo_x, -logo_y));

        outline.draw(&mut display).unwrap();
        triangle.draw(&mut display).unwrap();
        rectangle.draw(&mut display).unwrap();
        circle.draw(&mut display).unwrap();
        hello_rust.draw(&mut display).unwrap();

        display.flush().unwrap();
    };

    loop {
        const BX: i32 = 125 - 32;
        const BY: i32 = 61 - 32;

        for x in (2..=BX).rev() {
            draw(x, BY);
        }

        for y in (2..=BY).rev() {
            draw(2, y);
        }

        for x in 2..=BX {
            draw(x, 2);
        }

        for y in 2..=BY {
            draw(BX, y);
        }
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

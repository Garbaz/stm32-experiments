use core::fmt::{Debug, Write as _};
use cortex_m::prelude::{_embedded_hal_blocking_i2c_Write, _embedded_hal_blocking_i2c_WriteRead};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    text::{renderer::TextRenderer, Baseline, Text},
    Drawable as _,
};
use mpu6050::Mpu6050;

pub fn show_mpu_info<D, S, I, Err>(mpu: &mut Mpu6050<I>, display: &mut D, text_style: S)
where
    D: DrawTarget,
    D::Error: Debug,
    S: TextRenderer<Color = D::Color>,
    Err: Debug,
    I: _embedded_hal_blocking_i2c_Write<Error = Err>
        + _embedded_hal_blocking_i2c_WriteRead<Error = Err>,
{
    let angles = mpu.get_acc_angles().unwrap();
    let acc = mpu.get_acc().unwrap();
    let gyro = mpu.get_gyro().unwrap();
    let temp = mpu.get_temp().unwrap();

    let mut buffer: heapless::String<128> = heapless::String::new();
    writeln!(
        buffer,
        "Angles: {:+5.1}° {:+5.1}°",
        angles.x.to_degrees(),
        angles.y.to_degrees()
    )
    .unwrap();
    writeln!(buffer, "Acc:  {:+.1} {:+.1} {:+.1}", acc.x, acc.y, acc.z).unwrap();
    writeln!(buffer, "Gyro: {:+.1} {:+.1} {:+.1}", gyro.x, gyro.y, gyro.z).unwrap();
    writeln!(buffer, "Temp: {:+5.1}°C", temp).unwrap();

    Text::with_baseline(
        buffer.as_str(),
        Point::new(0, 10),
        text_style,
        Baseline::Top,
    )
    .draw(display)
    .unwrap();
}

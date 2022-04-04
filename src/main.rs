use embedded_hal::adc::OneShot;
use rppal::i2c::I2c;
use sunfounder_4wd::Sunfounder4wdHat;

fn main() {
    let i2c = I2c::new().unwrap();

    let mut hat = Sunfounder4wdHat::new(i2c);
    let mut grayscale_sensor_1 = sunfounder_4wd::A0 {};
    hat.read(&mut grayscale_sensor_1).unwrap();
    println!("Hello, world!");
}

/// Implement a device which impls embedded hal traits:
/// * ADC
/// * PWM
///
use embedded_hal::adc;
use rppal::i2c::I2c;

const NUMBER_OF_PWM_CHANNELS: usize = 14;
const NUMBER_OF_ADC_CHANNELS: u8 = 8;

pub struct Sunfounder4wdHat {
    i2c: I2c,
    current_pwm_duty_cycles: [u8; NUMBER_OF_PWM_CHANNELS],
}

impl Sunfounder4wdHat {
    pub fn new(i2c: I2c) -> Self {
        let mut driver = Self {
            i2c,
            current_pwm_duty_cycles: [0u8; NUMBER_OF_PWM_CHANNELS],
        };
        driver.setup_pwm();
        driver
    }

    fn setup_pwm(&mut self) {
        // Hmm so this doesn't yet actually set a hard frequency. That will probably be nice to incorporate at some point
        const NUMBER_OF_TIMERS: u8 = 4;
        // This register is used for storing a timer prescaler
        const REG_PSC: u8 = 0x40;
        // This register is used for storing the period in ticks
        const REG_ARR: u8 = 0x44;
        // Set the period as 1000 timer ticks
        const TICKS: u16 = 1000 - 1;
        const TICKS_HIGH_BYTE: u8 = (TICKS >> 8) as u8;
        const TICKS_LOW_BYTE: u8 = (TICKS & 0xFF) as u8;
        const PRESCALER: u16 = 10 - 1;
        const PRESCALER_HIGH_BYTE: u8 = (PRESCALER >> 8) as u8;
        const PRESCALER_LOW_BYTE: u8 = (PRESCALER & 0xFF) as u8;
        for timer_id in 0..NUMBER_OF_TIMERS {
            let _ = self.i2c.block_write(
                REG_PSC + timer_id,
                &[PRESCALER_HIGH_BYTE, PRESCALER_LOW_BYTE],
            );
            let _ = self
                .i2c
                .block_write(REG_ARR + timer_id, &[TICKS_HIGH_BYTE, TICKS_LOW_BYTE]);
        }
    }
}

impl<PIN> adc::OneShot<Sunfounder4wdHat, u16, PIN> for Sunfounder4wdHat
where
    PIN: adc::Channel<Sunfounder4wdHat, ID = u8>,
{
    type Error = rppal::i2c::Error;

    fn read(&mut self, _pin: &mut PIN) -> nb::Result<u16, Self::Error> {
        let channel = (7 - PIN::channel()) | 0x10;
        if let Err(err) = self.i2c.write(&[channel, 0, 0]) {
            return Err(nb::Error::Other(err));
        } else {
            let mut buf = [0u8, 0u8];
            match (self.i2c.read(&mut buf[..1]), self.i2c.read(&mut buf[1..])) {
                (Ok(_), Ok(_)) => Ok(u16::from(buf[0]) << 8 | u16::from(buf[1])),
                (Err(err), _) | (_, Err(err)) => Err(nb::Error::Other(err)),
            }
        }
    }
}

pub struct AnalogPin<const N: u8>;
impl<const N: u8> adc::Channel<Sunfounder4wdHat> for AnalogPin<N> {
    type ID = u8;

    fn channel() -> Self::ID {
        assert!(N < NUMBER_OF_ADC_CHANNELS);
        N
    }
}

pub type A0 = AnalogPin<0>;
pub type A1 = AnalogPin<1>;
pub type A2 = AnalogPin<2>;
pub type A3 = AnalogPin<3>;
pub type A4 = AnalogPin<4>;
pub type A5 = AnalogPin<5>;
pub type A6 = AnalogPin<6>;
pub type A7 = AnalogPin<7>;

impl embedded_hal::Pwm for Sunfounder4wdHat {
    type Duty = u8;
    type Channel = u8;
    type Time = f32;

    fn disable(&mut self, _channel: Self::Channel) {}

    fn enable(&mut self, _channel: Self::Channel) {}

    fn get_period(&self) -> Self::Time {
        1. / 50.
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let channel = usize::from(channel);
        if channel < NUMBER_OF_PWM_CHANNELS {
            self.current_pwm_duty_cycles[channel]
        } else {
            0
        }
    }

    fn get_max_duty(&self) -> Self::Duty {
        100
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        // A timer controls 4 channels
        let ticks = u16::from(duty) * 10;
        let ticks_high_byte = (ticks >> 8) as u8;
        let ticks_low_bytes = (ticks & 0xff) as u8;
        const REG_CHN: u8 = 0x20;
        if let Ok(_) = self
            .i2c
            .block_write(REG_CHN + channel, &[ticks_high_byte, ticks_low_bytes])
        {
            self.current_pwm_duty_cycles[channel as usize] = duty;
        }
    }

    fn set_period<P>(&mut self, _period: P) {
        // No op since this isn't really intended for generic use
    }
}

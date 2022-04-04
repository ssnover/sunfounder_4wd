# Interfaces Notes
## PWM
Appears to be done using AT32F415 microcontroller via i2c commands. Requires specifying prescaler and PWM frequency over i2c. Pretty annoying looking calculations
* LF Motor: i2c/pwm channel 13, direction pin 23 (Pi)
* RF Motor: i2c/pwm channel 12, direction pin 24 (Pi)
* LR Motor: i2c/pwm channel 8, direction pin 13
* RR Motor: i2c/pwm channel 9, dir pin 20

## ADC
Also appears to be a function of micro over i2c. Channels A0-A7

## Servo
Uses PWM channel 0

## Ultrasonic Sensor
* Uses Pi 5 for trigger and Pi 6 for echo

## Encoders
* Called `Speed` in the python code lol
* Pi pins 25, 4. Will need to verify which is left/right since I put the encoder on the front

## Grayscale Sensor
* Just made up of 3 ADC pins: A5, A6, A7

## I2C
* Device is AT32F415 micro
* i2c address is 0x14 or 0x15
* Words appear to be 16-bit, big endian

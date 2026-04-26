use crate::gpio;

pub fn success() {
    // Turn on the pin 16
    gpio::setup(16, gpio::Modes::Output);
    gpio::output(16, true);
}

pub fn fail() {
    // Turn on the pin 16
    gpio::setup(20, gpio::Modes::Output);
    gpio::output(20, true);
}

use redpitaya::pin::digital::Pin;
use redpitaya::pin::State;

fn main() -> redpitaya::Result {
    let leds = [
        Pin::RP_LED0,
        Pin::RP_LED1,
        Pin::RP_LED2,
        Pin::RP_LED3,
        Pin::RP_LED4,
        Pin::RP_LED5,
        Pin::RP_LED6,
        Pin::RP_LED7,
    ];

    let percent = match std::env::args().nth(1) {
        Some(arg) => arg.parse().unwrap(),
        None => 50.0,
    };

    redpitaya::init()?;

    for (x, led) in leds.iter().enumerate() {
        let state = if percent > (x as f32 * 100.0 / 8.0) {
            State::RP_HIGH
        } else {
            State::RP_LOW
        };

        redpitaya::pin::digital::set_state(*led, state)?;
    }

    redpitaya::release()
}

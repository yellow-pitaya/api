use redpitaya::pin::State;
use redpitaya::pin::digital::Pin;

fn main() -> redpitaya::Result<()> {
    let period = std::time::Duration::from_millis(1_000);

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

    let n = match std::env::args().nth(1) {
        Some(arg) => arg.parse().unwrap(),
        None => 0,
    };

    let led = leds[n];
    println!("Blinking {:?}", led);

    redpitaya::init()?;

    let mut retries = 1_000_000;
    while retries > 0 {
        redpitaya::pin::digital::set_state(led, State::RP_HIGH)?;
        std::thread::sleep(period / 2);

        redpitaya::pin::digital::set_state(led, State::RP_LOW)?;
        std::thread::sleep(period / 2);

        retries -= 1;
    }

    redpitaya::release()
}

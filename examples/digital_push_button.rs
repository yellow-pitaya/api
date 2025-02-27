use redpitaya::pin::Direction;
use redpitaya::pin::digital::Pin;

fn main() -> redpitaya::Result {
    let pins = [
        Pin::RP_DIO0_N,
        Pin::RP_DIO1_N,
        Pin::RP_DIO2_N,
        Pin::RP_DIO3_N,
        Pin::RP_DIO4_N,
        Pin::RP_DIO5_N,
        Pin::RP_DIO6_N,
        Pin::RP_DIO7_N,
    ];

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

    redpitaya::init()?;

    for pin in pins {
        redpitaya::pin::digital::set_direction(pin, Direction::RP_IN)?;
    }

    'outer: loop {
        for x in 0..pins.len() {
            let Ok(state) = redpitaya::pin::digital::state(pins[x]) else {
                break 'outer;
            };

            if redpitaya::pin::digital::set_state(leds[x], state).is_err() {
                break 'outer;
            }
        }
    }

    redpitaya::release()
}

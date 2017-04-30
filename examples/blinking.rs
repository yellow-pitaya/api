extern crate redpitaya;

use redpitaya::pin::State;
use redpitaya::pin::digital::{ Pin, set_state };

use std::mem;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let led = match env::args().nth(1) {
        Some(arg) => unsafe {
            mem::transmute(arg.parse::<u32>().unwrap())
        },
        None => Pin::RP_LED0,
    };

    match blink(led) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    }
}

fn blink(led: Pin) -> Result<(), String> {
    let period = Duration::from_millis(1_000);

    println!("Blinking {:?}", led);

    redpitaya::init()?;

    let mut retries = 1_000_000;

    while retries > 0 {
        set_state(led, State::RP_HIGH)?;
        sleep(period / 2);

        set_state(led, State::RP_LOW)?;
        sleep(period / 2);

        retries -= 1;
    }

    redpitaya::release()?;

    Ok(())
}

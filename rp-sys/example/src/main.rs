extern crate rp_sys;

use std::env;

use rp_sys as rp;
use std::thread::sleep;
use std::time::Duration;
use std::mem;

fn main() {
    let period = Duration::from_millis(500);
    let led: rp::rp_dpin_t = match env::args().nth(1) {
        Some(arg) => unsafe {
            mem::transmute(arg.parse::<u32>().unwrap())
        },
        None => rp::rp_dpin_t::RP_LED0,
    };

    println!("Blinking {:?}", led);

    unsafe {
        rp::rp_Init();
    }

    let mut retries: i8 = 5;

    while retries > 0 {
        unsafe {
            rp::rp_DpinSetState(led, rp::rp_pinState_t::RP_HIGH);
        }
        sleep(period / 2);

        unsafe {
            rp::rp_DpinSetState(led, rp::rp_pinState_t::RP_LOW);
        }
        sleep(period / 2);

        retries -= 1;
    }

    unsafe {
        rp::rp_Release();
    }
}

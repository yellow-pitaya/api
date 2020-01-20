use redpitaya::pin::analog::Pin;

fn main() -> Result<(), String> {
    redpitaya::init()?;

    for pin in [Pin::RP_AIN0, Pin::RP_AIN1, Pin::RP_AIN2, Pin::RP_AIN3].iter() {
        match redpitaya::pin::analog::get_value(*pin) {
            Ok(value) => println!("Measured voltage on {:?} = {}V", pin, value),
            Err(err) => println!("Error: {}", err),
        };
    }

    redpitaya::release()
}

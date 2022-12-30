use redpitaya::pin::analog::Pin;

fn main() -> redpitaya::Result {
    let mut values = [
        (Pin::RP_AOUT0, 0.0),
        (Pin::RP_AOUT1, 0.0),
        (Pin::RP_AOUT2, 0.0),
        (Pin::RP_AOUT3, 0.0),
    ];

    for x in 0..4 {
        match std::env::args().nth(x + 1) {
            Some(arg) => values[x].1 = arg.parse().unwrap(),
            None => values[x].1 = 1.0,
        };

        println!("Voltage setting for {:?} = {}V", values[x].0, values[x].1);
    }

    redpitaya::init()?;

    for x in values.iter() {
        let (pin, value) = *x;

        match redpitaya::pin::analog::set_value(pin, value) {
            Ok(_) => (),
            Err(err) => println!("Could not set {:?} voltage: {}", pin, err),
        };
    }

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    redpitaya::release()
}

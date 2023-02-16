const SIZE: usize = 0x100;

fn main() -> redpitaya::Result {
    let length = std::env::args()
        .next()
        .and_then(|x| x.parse().ok())
        .unwrap_or(64u32);

    println!("Length = {length}");

    let mut handle = redpitaya::la::open("/dev/uio/la")?;
    handle.dma.set_size(4, SIZE * 2)?;

    let cfg = redpitaya::la::CfgRegset {
        pre: 0,
        pst: length,
    };
    handle.set_cnt_config(&cfg)?;
    handle.set_config(0x2); // Set auto mode. Will be stopped after triggered
    handle.global_trig_set(0x0); // Disable triggers

    let mut map = handle.memory()?;

    for x in 0..6 {
        println!("Repeat: {x}");

        // clear data
        for x in 0..map.len() {
            map[x] = 0;
        }

        handle.run_acq()?;
        handle.blocking_read()?;

        // printout data
        for x in 0..map.len() {
            if x % 32 == 0 {
                print!("@{:08x}: ", x * std::mem::size_of_val(&map[x]));
            }
            print!("{:04x}", map[x]);
            if x % 32 == 31 {
                println!("");
            }
        }
    }

    Ok(())
}

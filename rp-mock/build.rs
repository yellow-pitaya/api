fn main() -> std::io::Result<()> {
    let dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap();
    let out = format!("{}/impl.rs", std::env::var("OUT_DIR").unwrap());

    match std::fs::copy(
        format!("{}/../rp-sys/src/impl.rs", dir),
        &out,
    ) {
        Ok(_) => return Ok(()),
        Err(_) => (),
    };

    std::fs::copy(
        format!("{}/../../../../rp-sys/src/impl.rs", dir),
        &out,
    )?;

    Ok(())
}

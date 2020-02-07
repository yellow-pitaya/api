fn main() -> std::io::Result<()> {
    let dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap();
    let out = format!("{}/impl.rs", std::env::var("OUT_DIR").unwrap());

    if std::fs::copy(format!("{}/../rp-sys/src/impl.rs", dir), &out,).is_ok() {
        return Ok(());
    }

    std::fs::copy(
        format!("{}/../../../../rp-sys/src/impl.rs", dir),
        &out,
    )?;

    Ok(())
}

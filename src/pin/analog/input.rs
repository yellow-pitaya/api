/**
 * Gets value from analog pin in volts.
 */
pub fn value(pin: u32) -> crate::Result<f32>
{
    let mut value = 0.0;

    match handle_unsafe!(
        crate::rp::rp_AIpinGetValue(pin, &mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

/**
 * Gets raw value from analog pin.
 */
pub fn value_raw(pin: u32) -> crate::Result<u32>
{
    let mut value = 0;

    match handle_unsafe!(
        crate::rp::rp_AIpinGetValueRaw(pin, &mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

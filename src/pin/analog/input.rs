extern crate rp_sys;

/**
 * Gets value from analog pin in volts.
 */
pub fn get_value(pin: u32) -> Result<f32, String>
{
    let mut value = 0.0;

    match handle_unsafe!(
        rp_sys::rp_AIpinGetValue(pin, &mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

/**
 * Gets raw value from analog pin.
 */
pub fn get_value_raw(pin: u32) -> Result<u32, String>
{
    let mut value = 0;

    match handle_unsafe!(
        rp_sys::rp_AIpinGetValueRaw(pin, &mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

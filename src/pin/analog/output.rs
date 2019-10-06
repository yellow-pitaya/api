use std::ops::Range;

/**
* Sets analog outputs to default values (0V).
*/
pub fn reset() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AOpinReset()
    )
}

/**
 * Gets value from analog pin in volts.
 */
pub fn get_value(pin: u32) -> Result<f32, String>
{
    let mut value = 0.0;

    match handle_unsafe!(
        rp_sys::rp_AOpinGetValue(pin, &mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

/**
 * Gets raw value from analog pin.
 */
pub fn get_raw_value(pin: u32) -> Result<u32, String>
{
    let mut value = 0;

    match handle_unsafe!(
        rp_sys::rp_AOpinGetValueRaw(pin, &mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

/**
 * Sets value in volts on analog output pin.
 */
pub fn set_value(pin: u32, value: f32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AOpinSetValue(pin, value)
    )
}

/**
 * Sets raw value on analog output pin.
 */
pub fn set_raw_value(pin: u32, value: u32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AOpinSetValueRaw(pin, value)
    )
}

/**
 * Gets range in volts on specific pin.
 */
pub fn get_range(pin: u32) -> Result<Range<f32>, String>
{
    let mut min = 0.0;
    let mut max = 0.0;

    match handle_unsafe!(
        rp_sys::rp_AOpinGetRange(pin, &mut min, &mut max)
    ) {
        Ok(_) => Ok(Range { start: min, end: max }),
        Err(err) => Err(err),
    }
}

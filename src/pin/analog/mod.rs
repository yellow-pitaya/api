pub mod input;
pub mod output;

/**
 * Type representing analog input output pins.
 */
pub use crate::rp::rp_apin_t as Pin;

use std::ops::Range;

/**
 * Sets analog outputs to default values (0V).
 */
pub fn reset() -> crate::Result {
    handle_unsafe!(crate::rp::rp_ApinReset())
}

/**
 * Gets value from analog pin in volts.
 */
pub fn value(pin: Pin) -> crate::Result<f32> {
    let mut value = 0.0;

    match handle_unsafe!(crate::rp::rp_ApinGetValue(pin, &mut value)) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

/**
 * Gets raw value from analog pin.
 */
pub fn raw_value(pin: Pin) -> crate::Result<u32> {
    let mut value = 0;

    match handle_unsafe!(crate::rp::rp_ApinGetValueRaw(pin, &mut value)) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

/**
 * Sets value in volts on analog output pin.
 */
pub fn set_value(pin: Pin, value: f32) -> crate::Result {
    handle_unsafe!(crate::rp::rp_ApinSetValue(pin, value))
}

/**
 * Sets raw value on analog output pin.
 */
pub fn set_raw_value(pin: Pin, value: u32) -> crate::Result {
    handle_unsafe!(crate::rp::rp_ApinSetValueRaw(pin, value))
}

/**
 * Gets range in volts on specific pin.
 */
pub fn range(pin: Pin) -> crate::Result<Range<f32>> {
    let mut min = 0.0;
    let mut max = 0.0;

    match handle_unsafe!(crate::rp::rp_ApinGetRange(pin, &mut min, &mut max)) {
        Ok(_) => Ok(Range {
            start: min,
            end: max,
        }),
        Err(err) => Err(err),
    }
}

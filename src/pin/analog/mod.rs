extern crate rp_sys;

pub mod output;

pub use rp_sys:: rp_apin_t as Pin;

use std::ops::Range;

pub fn reset() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_ApinReset()
    )
}

pub fn get_value(pin: Pin) -> Result<f32, String>
{
    let mut value = 0.0;

    match handle_unsafe!(
        rp_sys::rp_ApinGetValue(pin, &mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

pub fn get_raw_value(pin: Pin) -> Result<u32, String>
{
    let mut value = 0;

    match handle_unsafe!(
        rp_sys::rp_ApinGetValueRaw(pin, &mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

pub fn set_value(pin: Pin, value: f32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_ApinSetValue(pin, value)
    )
}

pub fn set_raw_value(pin: Pin, value: u32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_ApinSetValueRaw(pin, value)
    )
}

pub fn get_range(pin: Pin) -> Result<Range<f32>, String>
{
    let mut min = 0.0;
    let mut max = 0.0;

    match handle_unsafe!(
        rp_sys::rp_ApinGetRange(pin, &mut min, &mut max)
    ) {
        Ok(_) => Ok(Range { start: min, end: max }),
        Err(err) => Err(err),
    }
}

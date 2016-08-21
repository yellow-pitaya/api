extern crate rp_sys;

pub use rp_sys::rp_dpin_t as Pin;

pub fn reset() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_DpinReset()
    )
}

pub fn set_state(pin: Pin, state: super::State) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_DpinSetState(pin, state)
    )
}

pub fn get_state(pin: Pin) -> Result<super::State, String>
{
    let mut state = super::State::RP_LOW;

    match handle_unsafe!(
        rp_sys::rp_DpinGetState(pin, &mut state)
    ) {
        Ok(_) => Ok(state),
        Err(err) => Err(err),
    }
}

pub fn set_direction(pin: Pin, direction: super::Direction) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_DpinSetDirection(pin, direction)
    )
}

pub fn get_direction(pin: Pin) -> Result<super::Direction, String>
{
    let mut direction = super::Direction::RP_IN;

    match handle_unsafe!(
        rp_sys::rp_DpinGetDirection(pin, &mut direction)
    ) {
        Ok(_) => Ok(direction),
        Err(err) => Err(err),
    }
}

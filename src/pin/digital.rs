/**
 * Type representing digital input output pins.
 */
pub use crate::rp::rp_dpin_t as Pin;

/**
 * Sets digital pins to default values.
 *
 * Pins DIO1_P - DIO7_P, RP_DIO0_N - RP_DIO7_N are set al OUTPUT and to LOW.
 * LEDs are set to LOW/OFF
 */
pub fn reset() -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_DpinReset()
    )
}

/**
 * Sets digital input output pin state.
 */
pub fn set_state(pin: Pin, state: super::State) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_DpinSetState(pin, state)
    )
}

/**
 * Gets digital input output pin state.
 */
pub fn get_state(pin: Pin) -> crate::Result<super::State>
{
    let mut state = super::State::RP_LOW;

    match handle_unsafe!(
        crate::rp::rp_DpinGetState(pin, &mut state)
    ) {
        Ok(_) => Ok(state),
        Err(err) => Err(err),
    }
}

/**
 * Sets digital input output pin direction.
 *
 * LED pins are already automatically set to the output direction, and they
 * cannot be set to the input direction. DIOx_P and DIOx_N are must set either
 * output or input direction before they can be used. When set to input
 * direction, it is not allowed to write into these pins.
 */
pub fn set_direction(pin: Pin, direction: super::Direction) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_DpinSetDirection(pin, direction)
    )
}

/**
 * Gets digital input output pin direction.
 */
pub fn get_direction(pin: Pin) -> crate::Result<super::Direction>
{
    let mut direction = super::Direction::RP_IN;

    match handle_unsafe!(
        crate::rp::rp_DpinGetDirection(pin, &mut direction)
    ) {
        Ok(_) => Ok(direction),
        Err(err) => Err(err),
    }
}

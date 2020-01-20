#[allow(unused_must_use)]

pub fn set_state(state: u32) -> Result<(), String>
{
    handle_unsafe!(
        crate::rp::rp_LEDSetState(state)
    )
}

pub fn get_state() -> Result<u32, String>
{
    let mut state = 0;

    match handle_unsafe!(
        crate::rp::rp_LEDGetState(&mut state)
    ) {
        Ok(_) => Ok(state),
        Err(err) => Err(err),
    }
}

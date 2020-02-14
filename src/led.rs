#[allow(unused_must_use)]
#[warn(rust_2018_idioms)]

pub fn set_state(state: u32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_LEDSetState(state)
    )
}

pub fn get_state() -> crate::Result<u32>
{
    let mut state = 0;

    match handle_unsafe!(
        crate::rp::rp_LEDGetState(&mut state)
    ) {
        Ok(_) => Ok(state),
        Err(err) => Err(err),
    }
}

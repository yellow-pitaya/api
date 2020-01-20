#[allow(unused_must_use)]

pub mod n {
    pub fn set_direction(direction: u32) -> Result<(), String>
    {
        handle_unsafe!(
            crate::rp::rp_GPIOnSetDirection(direction)
        )
    }

    pub fn get_direction() -> Result<u32, String>
    {
        let mut direction = 0;

        match handle_unsafe!(
            crate::rp::rp_GPIOnGetDirection(&mut direction)
        ) {
            Ok(_) => Ok(direction),
            Err(err) => Err(err),
        }
    }

    pub fn set_state(state: u32) -> Result<(), String>
    {
        handle_unsafe!(
            crate::rp::rp_GPIOnSetState(state)
        )
    }

    pub fn get_state() -> Result<u32, String>
    {
        let mut state = 0;

        match handle_unsafe!(
            crate::rp::rp_GPIOnGetState(&mut state)
        ) {
            Ok(_) => Ok(state),
            Err(err) => Err(err),
        }
    }
}

pub mod p {
    pub fn set_direction(direction: u32) -> Result<(), String>
    {
        handle_unsafe!(
            crate::rp::rp_GPIOpSetDirection(direction)
        )
    }

    pub fn get_direction() -> Result<u32, String>
    {
        let mut direction = 0;

        match handle_unsafe!(
            crate::rp::rp_GPIOpGetDirection(&mut direction)
        ) {
            Ok(_) => Ok(direction),
            Err(err) => Err(err),
        }
    }

    pub fn set_state(state: u32) -> Result<(), String>
    {
        handle_unsafe!(
            crate::rp::rp_GPIOpSetState(state)
            )
    }

    pub fn get_state() -> Result<u32, String>
    {
        let mut state = 0;

        match handle_unsafe!(
            crate::rp::rp_GPIOpGetState(&mut state)
        ) {
            Ok(_) => Ok(state),
            Err(err) => Err(err),
        }
    }
}

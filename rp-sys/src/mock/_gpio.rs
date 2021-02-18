#[derive(Default)]
struct GpioState {
    p_direction: u32,
    p_state: u32,
    n_direction: u32,
    n_state: u32,
}

pub unsafe fn rp_GPIOnGetDirection(direction: *mut u32) -> c_int
{
    *direction = state!().gpio.p_direction;

    ok!()
}

pub unsafe fn rp_GPIOnGetState(state: *mut u32) -> c_int
{
    *state = state!().gpio.p_state;

    ok!()
}

pub unsafe fn rp_GPIOnSetDirection(direction: u32) -> c_int
{
    state!().gpio.p_direction = direction;

    ok!()
}

pub unsafe fn rp_GPIOnSetState(state: u32) -> c_int
{
    state!().gpio.p_state = state;

    ok!()
}

pub unsafe fn rp_GPIOpGetDirection(direction: *mut u32) -> c_int
{
    *direction = state!().gpio.n_direction;

    ok!()
}

pub unsafe fn rp_GPIOpGetState(state: *mut u32) -> c_int
{
    *state = state!().gpio.n_state;

    ok!()
}

pub unsafe fn rp_GPIOpSetDirection(direction: u32) -> c_int
{
    state!().gpio.n_direction = direction;

    ok!()
}

pub unsafe fn rp_GPIOpSetState(state: u32) -> c_int
{
    state!().gpio.n_state = state;

    ok!()
}

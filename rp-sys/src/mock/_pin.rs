struct PinState {
    directions: Vec<rp_pinDirection_t>,
    states: Vec<rp_pinState_t>,
}

impl Default for PinState
{
    fn default() -> Self
    {
        Self {
            directions: vec![rp_pinDirection_t::RP_IN; 24],
            states: vec![rp_pinState_t::RP_LOW; 24],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum rp_apin_t {
    RP_AOUT0,
    RP_AOUT1,
    RP_AOUT2,
    RP_AOUT3,
    RP_AIN0,
    RP_AIN1,
    RP_AIN2,
    RP_AIN3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_dpin_t {
    RP_LED0,
    RP_LED1,
    RP_LED2,
    RP_LED3,
    RP_LED4,
    RP_LED5,
    RP_LED6,
    RP_LED7,
    RP_DIO0_P,
    RP_DIO1_P,
    RP_DIO2_P,
    RP_DIO3_P,
    RP_DIO4_P,
    RP_DIO5_P,
    RP_DIO6_P,
    RP_DIO7_P,
    RP_DIO0_N,
    RP_DIO1_N,
    RP_DIO2_N,
    RP_DIO3_N,
    RP_DIO4_N,
    RP_DIO5_N,
    RP_DIO6_N,
    RP_DIO7_N,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_pinDirection_t {
    RP_IN,
    RP_OUT,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_pinState_t {
    RP_LOW,
    RP_HIGH,
}

pub unsafe fn rp_AIpinGetValue(pin: c_uint, value: *mut f32) -> c_int
{
    let mut value_raw = 0;
    let result = rp_AIpinGetValueRaw(pin, &mut value_raw);

    *value = (value_raw as f32 / ANALOG_IN_MAX_VAL_INTEGER as f32)
        * (ANALOG_IN_MAX_VAL - ANALOG_IN_MIN_VAL)
        + ANALOG_IN_MIN_VAL;

    result
}

pub unsafe fn rp_AIpinGetValueRaw(pin: c_uint, value: *mut u32) -> c_int
{
    if pin >= 8 {
        return RP_EPN as c_int;
    }

    *value = state!().apin[pin as usize];

    ok!()
}

pub unsafe fn rp_AOpinGetRange(pin: c_uint, min_val: *mut f32, max_val: *mut f32) -> c_int
{
    *min_val = ANALOG_OUT_MIN_VAL;
    *max_val = ANALOG_OUT_MAX_VAL;

    ok!()
}

pub unsafe fn rp_AOpinGetValue(pin: c_uint, value: *mut f32) -> c_int
{
    let mut value_raw = 0;
    let result = rp_AOpinGetValueRaw(pin, &mut value_raw);

    *value = (value_raw as f32 / ANALOG_OUT_MAX_VAL_INTEGER as f32)
        * (ANALOG_OUT_MAX_VAL - ANALOG_OUT_MIN_VAL)
        + ANALOG_OUT_MIN_VAL;

    result
}

pub unsafe fn rp_AOpinGetValueRaw(pin: c_uint, value: *mut u32) -> c_int
{
    if pin >= 8 {
        return RP_EPN as c_int;
    }

    *value = state!().apin[pin as usize];

    ok!()
}

pub unsafe fn rp_AOpinReset() -> c_int
{
    state!().apin = vec![0; 8];

    ok!()
}

pub unsafe fn rp_AOpinSetValue(pin: c_uint, value: f32) -> c_int
{
    let value_raw = (value - ANALOG_OUT_MIN_VAL) / (ANALOG_OUT_MAX_VAL - ANALOG_OUT_MIN_VAL) * ANALOG_OUT_MAX_VAL_INTEGER as f32;

    rp_AOpinSetValueRaw(pin, value_raw as u32)
}

pub unsafe fn rp_AOpinSetValueRaw(pin: c_uint, value: u32) -> c_int
{
    if pin >= 8 {
        return RP_EPN as c_int;
    }

    if value > ANALOG_OUT_MAX_VAL_INTEGER {
        return RP_EOOR as c_int;
    }

    state!().apin[pin as usize] = value;

    ok!()
}

pub unsafe fn rp_ApinGetRange(pin: rp_apin_t, min_val: *mut f32, max_val: *mut f32) -> c_int
{
    if pin <= rp_apin_t::RP_AOUT3 {
        *min_val = ANALOG_OUT_MIN_VAL;
        *max_val = ANALOG_OUT_MAX_VAL;
    } else {
        *min_val = ANALOG_IN_MIN_VAL;
        *max_val = ANALOG_IN_MAX_VAL;
    }

    ok!()
}

pub unsafe fn rp_ApinGetValue(pin: rp_apin_t, value: *mut f32) -> c_int
{
    if pin <= rp_apin_t::RP_AOUT3 {
        rp_AOpinGetValue(pin as c_uint, value)
    } else {
        rp_AIpinGetValue(pin as c_uint, value)
    }
}

pub unsafe fn rp_ApinGetValueRaw(pin: rp_apin_t, value: *mut u32) -> c_int
{
    if pin <= rp_apin_t::RP_AOUT3 {
        rp_AOpinGetValueRaw(pin as c_uint, value)
    } else {
        rp_AIpinGetValueRaw(pin as c_uint, value)
    }
}

pub unsafe fn rp_ApinReset() -> c_int
{
    state!().apin = vec![0; 8];

    ok!()
}

pub unsafe fn rp_ApinSetValue(pin: rp_apin_t, value: f32) -> c_int
{
    if pin <= rp_apin_t::RP_AOUT3 {
        rp_AOpinSetValue(pin as c_uint, value)
    } else {
        RP_EPN as c_int
    }
}

pub unsafe fn rp_ApinSetValueRaw(pin: rp_apin_t, value: u32) -> c_int
{
    if pin <= rp_apin_t::RP_AOUT3 {
        rp_AOpinSetValueRaw(pin as c_uint, value)
    } else {
        RP_EPN as c_int
    }
}

pub unsafe fn rp_CmnCnvCntToV(field_len: u32, cnts: u32, adc_max_v: f32, calibScale: u32, calib_dc_off: c_int, user_dc_off: f32) -> f32
{
    cnts as f32
}

pub unsafe fn rp_DpinGetDirection(pin: rp_dpin_t, direction: *mut rp_pinDirection_t) -> c_int
{
    *direction = state!().pin.directions[pin as usize];

    ok!()
}

pub unsafe fn rp_DpinGetState(pin: rp_dpin_t, state: *mut rp_pinState_t) -> c_int
{
    *state = state!().pin.states[pin as usize];

    ok!()
}

pub unsafe fn rp_DpinReset() -> c_int
{
    state!().pin = Default::default();

    ok!()
}

pub unsafe fn rp_DpinSetDirection(pin: rp_dpin_t, direction: rp_pinDirection_t) -> c_int
{
    state!().pin.directions[pin as usize] = direction;

    ok!()
}

pub unsafe fn rp_DpinSetState(pin: rp_dpin_t, state: rp_pinState_t) -> c_int
{
    state!().pin.states[pin as usize] = state;

    ok!()
}

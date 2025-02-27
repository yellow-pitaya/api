use core::ffi::{c_char, c_int, c_uchar, c_uint};
use std::sync::{LazyLock, Mutex};

macro_rules! return_cstring {
    ($e:expr) => (
        {
            let s = std::ffi::CString::new($e)
                .unwrap();

            let p = s.as_ptr();
            std::mem::forget(s);

            p
        }
    );
}

pub const ADC_BUFFER_SIZE: u32 = 16384;

pub const RP_OK: u32 = 0;
pub const RP_EOED: u32 = 1;
pub const RP_EOMD: u32 = 2;
pub const RP_ECMD: u32 = 3;
pub const RP_EMMD: u32 = 4;
pub const RP_EUMD: u32 = 5;
pub const RP_EOOR: u32 = 6;
pub const RP_ELID: u32 = 7;
pub const RP_EMRO: u32 = 8;
pub const RP_EWIP: u32 = 9;
pub const RP_EPN: u32 = 10;
pub const RP_UIA: u32 = 11;
pub const RP_FCA: u32 = 12;
pub const RP_RCA: u32 = 13;
pub const RP_BTS: u32 = 14;
pub const RP_EIPV: u32 = 15;
pub const RP_EUF: u32 = 16;
pub const RP_ENN: u32 = 17;
pub const RP_EFOB: u32 = 18;
pub const RP_EFCB: u32 = 19;
pub const RP_EABA: u32 = 20;
pub const RP_EFRB: u32 = 21;
pub const RP_EFWB: u32 = 22;
pub const RP_EMNC: u32 = 23;
pub const RP_NOTS: u32 = 24;

pub const SPECTR_OUT_SIG_LEN: u32 = 2048;

const ADC_SAMPLE_PERIOD: u32 = 8;
const ANALOG_IN_MAX_VAL: f32 = 7.;
const ANALOG_IN_MIN_VAL: f32 = 0.;
const ANALOG_IN_MAX_VAL_INTEGER: u32 = 0xFFF;
const ANALOG_OUT_MAX_VAL: f32 = 1.8;
const ANALOG_OUT_MIN_VAL: f32 = 0.;
const ANALOG_OUT_MAX_VAL_INTEGER: u32 = 156;

#[derive(Default)]
struct State {
    acq: AcqState,
    apin: Vec<u32>,
    calib: rp_calib_params_t,
    r#gen: GenState,
    gpio: GpioState,
    led_state: u32,
    pin: PinState,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| {
    Default::default()
});

macro_rules! state {
    () => {
        STATE.lock().unwrap()
    }
}

macro_rules! ok {
    () => {
        RP_OK as c_int
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_channel_t {
    RP_CH_1,
    RP_CH_2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_channel_trigger_t {
    RP_T_CH_1,
    RP_T_CH_2,
    RP_T_CH_EXT,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_eq_filter_cof_t {
    AA,
    BB,
    PP,
    KK,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_trig_src_t {
    RP_GEN_TRIG_SRC_INTERNAL,
    RP_GEN_TRIG_SRC_EXT_PE,
    RP_GEN_TRIG_SRC_EXT_NE,
    RP_GEN_TRIG_GATED_BURST,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_waveform_t {
    RP_WAVEFORM_SINE,
    RP_WAVEFORM_SQUARE,
    RP_WAVEFORM_TRIANGLE,
    RP_WAVEFORM_RAMP_UP,
    RP_WAVEFORM_RAMP_DOWN,
    RP_WAVEFORM_DC,
    RP_WAVEFORM_PWM,
    RP_WAVEFORM_ARBITRARY,
    #[cfg(feature = "v1_04")]
    RP_WAVEFORM_DC_NEG,
    #[cfg(feature = "v1_04")]
    RP_WAVEFORM_SWEEP,
}

pub unsafe fn rp_EnableDigitalLoop(_enable: bool) -> c_int
{
    ok!()
}

pub unsafe fn rp_SetEnableTempProtection(channel: rp_channel_t, enable: bool) -> c_int
{
    RP_NOTS as c_int
}

pub unsafe fn rp_GetEnableTempProtection(channel: rp_channel_t, enable: &mut bool) -> c_int
{
    RP_NOTS as c_int
}

pub unsafe fn rp_SetLatchTempAlarm(channel: rp_channel_t, status: bool) -> c_int
{
    RP_NOTS as c_int
}

pub unsafe fn rp_GetLatchTempAlarm(channel: rp_channel_t, status: &mut bool) -> c_int
{
    RP_NOTS as c_int
}

pub unsafe fn rp_GetRuntimeTempAlarm(channel: rp_channel_t, status: &mut bool) -> c_int
{
    RP_NOTS as c_int
}

pub unsafe fn rp_GetPllControlEnable(enable: &mut bool) -> c_int
{
    RP_NOTS as c_int
}

pub unsafe fn rp_SetPllControlEnable(enable: bool) -> c_int
{
    RP_NOTS as c_int
}

pub unsafe fn rp_GetPllControlLocked(status: &mut bool) -> c_int
{
    RP_NOTS as c_int
}

pub unsafe fn rp_GetError(errorCode: c_int) -> *const c_char
{
    let error = match errorCode as u32 {
        RP_OK => "OK",
        RP_EOED => "Failed to Open EEPROM Device.",
        RP_EOMD => "Failed to open memory device.",
        RP_ECMD => "Failed to close memory device.",
        RP_EMMD => "Failed to map memory device.",
        RP_EUMD => "Failed to unmap memory device.",
        RP_EOOR => "Value out of range.",
        RP_ELID => "LED input direction is not valid.",
        RP_EMRO => "Modifying read only filed is not allowed.",
        RP_EWIP => "Writing to input pin is not valid.",
        RP_EPN => "Invalid Pin number.",
        RP_UIA => "Uninitialized Input Argument.",
        RP_FCA => "Failed to Find Calibration Parameters.",
        RP_RCA => "Failed to Read Calibration Parameters.",
        RP_BTS => "Buffer too small",
        RP_EIPV => "Invalid parameter value",
        RP_EUF => "Unsupported Feature",
        RP_ENN => "Data not normalized",
        RP_EFOB => "Failed to open bus",
        RP_EFCB => "Failed to close bus",
        RP_EABA => "Failed to acquire bus access",
        RP_EFRB => "Failed to read from the bus",
        RP_EFWB => "Failed to write to the bus",
        _ => "Unknown error",
    };

    return_cstring!(error)
}

pub unsafe fn rp_GetVersion() -> *const c_char
{
    return_cstring!("0.00-0000 (unknown)")
}

pub unsafe fn rp_IdGetDNA(dna: *mut u64) -> c_int
{
    unsafe {
        *dna = 1;
    }

    ok!()
}

pub unsafe fn rp_IdGetID(id: *mut u32) -> c_int
{
    unsafe {
        *id = 2;
    }

    ok!()
}

pub unsafe fn rp_Init() -> c_int
{
    ok!()
}

pub unsafe fn rp_InitReset(reset: bool) -> c_int
{
    ok!()
}

pub unsafe fn rp_LEDGetState(state: *mut u32) -> c_int
{
    unsafe {
        *state = state!().led_state;
    }

    ok!()
}

pub unsafe fn rp_LEDSetState(state: u32) -> c_int
{
    state!().led_state = state;

    ok!()
}

pub unsafe fn rp_Release() -> c_int
{
    ok!()
}

pub unsafe fn rp_Reset() -> c_int
{
    *state!() = Default::default();

    ok!()
}

include!("_acq.rs");
include!("_calib.rs");
include!("_gen.rs");
include!("_gpio.rs");
include!("_pin.rs");
include!("_uart.rs");

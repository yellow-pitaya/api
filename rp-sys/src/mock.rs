use libc::{c_char, c_int, c_uint};
use std::sync::Mutex;

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

pub const SPECTR_OUT_SIG_LEN: u32 = 2048;

const ADC_SAMPLE_PERIOD: u32 = 8;
const ANALOG_IN_MAX_VAL: f32 = 7.;
const ANALOG_IN_MIN_VAL: f32 = 0.;
const ANALOG_IN_MAX_VAL_INTEGER: u32 = 0xFFF;
const ANALOG_OUT_MAX_VAL: f32 = 1.8;
const ANALOG_OUT_MIN_VAL: f32 = 0.;
const ANALOG_OUT_MAX_VAL_INTEGER: u32 = 156;

struct AcqState {
    decimation: rp_acq_decimation_t,
    averaging: bool,
    gain: Vec<rp_pinState_t>,
    trigger_src: rp_acq_trig_src_t,
    trigger_delay: i64,
    trigger_hyst: f32,
    trigger_level: f32,
}

impl Default for AcqState
{
    fn default() -> Self
    {
        Self {
            decimation: rp_acq_decimation_t::RP_DEC_1, // @FIXME
            averaging: false,
            gain: vec![rp_pinState_t::RP_LOW; 2],
            trigger_src: rp_acq_trig_src_t::RP_TRIG_SRC_DISABLED,
            trigger_delay: 16381,
            trigger_hyst: 1., // @FIXME
            trigger_level: 1., // @FIXME
        }
    }
}

struct GpioState {
    p_direction: u32,
    p_state: u32,
    n_direction: u32,
    n_state: u32,
}

impl Default for GpioState
{
    fn default() -> Self
    {
        Self {
            p_direction: 0,
            p_state: 0,
            n_direction: 0,
            n_state: 0,
        }
    }
}

struct GenState {
    offsets: Vec<f32>,
    out_enable: Vec<bool>,
    phases: Vec<f32>,
    waveforms: Vec<rp_waveform_t>,
    modes: Vec<rp_gen_mode_t>,
    trigger_src: Vec<rp_trig_src_t>,
    amp: Vec<f32>,
    burst_count: Vec<i32>,
    burst_period: Vec<u32>,
    burst_repetitions: Vec<i32>,
    duty_cycle: Vec<f32>,
    freq: Vec<f32>,
}

impl Default for GenState
{
    fn default() -> Self
    {
        Self {
            offsets: vec![0.; 2],
            out_enable: vec![false; 2],
            phases: vec![0.; 2],
            waveforms: vec![rp_waveform_t::RP_WAVEFORM_SINE; 2],
            modes: vec![rp_gen_mode_t::RP_GEN_MODE_BURST; 2],
            trigger_src: vec![rp_trig_src_t::RP_GEN_TRIG_SRC_EXT_NE; 2],
            amp: vec![0.; 2],
            burst_count: vec![0; 2],
            burst_period: vec![0; 2],
            burst_repetitions: vec![0; 2],
            duty_cycle: vec![0.; 2],
            freq: vec![0.; 2],
        }
    }
}

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

struct State {
    acq: AcqState,
    apin: Vec<u32>,
    calib: rp_calib_params_t,
    gen: GenState,
    gpio: GpioState,
    led_state: u32,
    pin: PinState,
}

impl Default for State {
    fn default() -> Self
    {
        Self {
            acq: Default::default(),
            apin: vec![0; 8],
            calib: Default::default(),
            gen: Default::default(),
            gpio: Default::default(),
            led_state: 0,
            pin: Default::default(),
        }
    }
}

lazy_static::lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(Default::default());
}

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
pub struct rp_calib_params_t {
    pub fe_ch1_fs_g_hi: u32,
    pub fe_ch2_fs_g_hi: u32,
    pub fe_ch1_fs_g_lo: u32,
    pub fe_ch2_fs_g_lo: u32,
    pub fe_ch1_lo_offs: i32,
    pub fe_ch2_lo_offs: i32,
    pub be_ch1_fs: u32,
    pub be_ch2_fs: u32,
    pub be_ch1_dc_offs: i32,
    pub be_ch2_dc_offs: i32,
    pub magic: u32,
    pub fe_ch1_hi_offs: i32,
    pub fe_ch2_hi_offs: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_acq_decimation_t {
    RP_DEC_1,
    RP_DEC_8,
    RP_DEC_64,
    RP_DEC_1024,
    RP_DEC_8192,
    RP_DEC_65536,
}

impl std::convert::Into<rp_acq_sampling_rate_t> for rp_acq_decimation_t {
    fn into(self) -> rp_acq_sampling_rate_t {
        use rp_acq_sampling_rate_t::*;

        match self {
            Self::RP_DEC_1 => RP_SMP_125M,
            Self::RP_DEC_8 => RP_SMP_15_625M,
            Self::RP_DEC_64 => RP_SMP_1_953M,
            Self::RP_DEC_1024 => RP_SMP_122_070K,
            Self::RP_DEC_8192 => RP_SMP_15_258K,
            Self::RP_DEC_65536 => RP_SMP_1_907K,
        }
    }
}

impl std::convert::From<rp_acq_sampling_rate_t> for rp_acq_decimation_t {
    fn from(sampling_rate: rp_acq_sampling_rate_t) -> Self {
        use rp_acq_sampling_rate_t::*;

        match sampling_rate {
            RP_SMP_125M => Self::RP_DEC_1,
            RP_SMP_15_625M => Self::RP_DEC_8,
            RP_SMP_1_953M => Self::RP_DEC_64,
            RP_SMP_122_070K => Self::RP_DEC_1024,
            RP_SMP_15_258K => Self::RP_DEC_8192,
            RP_SMP_1_907K => Self::RP_DEC_65536,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_acq_sampling_rate_t {
    RP_SMP_125M,
    RP_SMP_15_625M,
    RP_SMP_1_953M,
    RP_SMP_122_070K,
    RP_SMP_15_258K,
    RP_SMP_1_907K,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_acq_trig_src_t {
    RP_TRIG_SRC_DISABLED,
    RP_TRIG_SRC_NOW,
    RP_TRIG_SRC_CHA_PE,
    RP_TRIG_SRC_CHA_NE,
    RP_TRIG_SRC_CHB_PE,
    RP_TRIG_SRC_CHB_NE,
    RP_TRIG_SRC_EXT_PE,
    RP_TRIG_SRC_EXT_NE,
    RP_TRIG_SRC_AWG_PE,
    RP_TRIG_SRC_AWG_NE,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_acq_trig_state_t {
    RP_TRIG_STATE_TRIGGERED,
    RP_TRIG_STATE_WAITING,
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
pub enum rp_channel_t {
    RP_CH_1,
    RP_CH_2,
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
pub enum rp_gen_mode_t {
    RP_GEN_MODE_CONTINUOUS,
    RP_GEN_MODE_BURST,
    RP_GEN_MODE_STREAM,
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

pub unsafe fn rp_AcqGetAveraging(enabled: *mut bool) -> c_int
{
    *enabled = state!().acq.averaging;

    ok!()
}

pub unsafe fn rp_AcqGetBufSize(size: *mut u32) -> c_int
{
    *size = ADC_BUFFER_SIZE;

    ok!()
}

pub unsafe fn rp_AcqGetDataPosRaw(channel: rp_channel_t, start_pos: u32, end_pos: u32, buffer: *mut i16, buffer_size: *mut u32) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetDataPosV(channel: rp_channel_t, start_pos: u32, end_pos: u32, buffer: *mut f32, buffer_size: *mut u32) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetDataRaw(channel: rp_channel_t, pos: u32, size: *mut u32, buffer: *mut i16) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetDataRawV2(pos: u32, size: *mut u32, buffer: *mut u16, buffer2: *mut u16) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetDataV(channel: rp_channel_t, pos: u32, size: *mut u32, buffer: *mut f32) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetDataV2(pos: u32, size: *mut u32, buffer1: *mut f32, buffer2: *mut f32) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetDecimation(decimation: *mut rp_acq_decimation_t) -> c_int
{
    *decimation = state!().acq.decimation;

    ok!()
}

pub unsafe fn rp_AcqGetDecimationFactor(decimation: *mut u32) -> c_int
{
    *decimation = state!().acq.decimation.into();

    ok!()
}

pub unsafe fn rp_AcqGetGain(channel: rp_channel_t, state: *mut rp_pinState_t) -> c_int
{
    *state = state!().acq.gain[channel as usize];

    ok!()
}

pub unsafe fn rp_AcqGetGainV(channel: rp_channel_t, voltage: *mut f32) -> c_int
{
    let mut state = rp_pinState_t::RP_LOW;

    rp_AcqGetGain(channel, &mut state);

    if state == rp_pinState_t::RP_LOW {
        *voltage = 1.0;
    }
    else {
        *voltage = 20.0;
    }

    ok!()
}

pub unsafe fn rp_AcqGetLatestDataRaw(channel: rp_channel_t, size: *mut u32, buffer: *mut i16) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetLatestDataV(channel: rp_channel_t, size: *mut u32, buffer: *mut f32) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetNormalizedDataPos(pos: u32) -> u32
{
    pos % ADC_BUFFER_SIZE
}

pub unsafe fn rp_AcqGetOldestDataRaw(channel: rp_channel_t, size: *mut u32, buffer: *mut i16) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetOldestDataV(channel: rp_channel_t, size: *mut u32, buffer: *mut f32) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetPreTriggerCounter(value: *mut u32) -> c_int
{
    *value = 10;

    ok!()
}

pub unsafe fn rp_AcqGetSamplingRate(sampling_rate: *mut rp_acq_sampling_rate_t) -> c_int
{
    *sampling_rate = state!().acq.decimation.into();

    ok!()
}

pub unsafe fn rp_AcqGetSamplingRateHz(sampling_rate: *mut f32) -> c_int
{
    let max_rate: f32 = 125_000_000.;
    let decimation: u32 = state!().acq.decimation.into();

    *sampling_rate = max_rate / decimation as f32;

    0
}

pub unsafe fn rp_AcqGetTriggerDelay(decimated_data_num: *mut i32) -> c_int
{
    let delay = state!().acq.trigger_delay;
    let decimation: u32 = state!().acq.decimation.into();

    *decimated_data_num = (delay / ADC_SAMPLE_PERIOD as i64 / decimation as i64) as i32;

    ok!()
}

pub unsafe fn rp_AcqGetTriggerDelayNs(time_ns: *mut i64) -> c_int
{
    *time_ns = state!().acq.trigger_delay;

    ok!()
}

pub unsafe fn rp_AcqGetTriggerHyst(voltage: *mut f32) -> c_int
{
    *voltage = state!().acq.trigger_hyst;

    ok!()
}

pub unsafe fn rp_AcqGetTriggerLevel(voltage: *mut f32) -> c_int
{
    state!().acq.trigger_level;

    ok!()
}

pub unsafe fn rp_AcqGetTriggerSrc(source: *mut rp_acq_trig_src_t) -> c_int
{
    *source = state!().acq.trigger_src;

    ok!()
}

pub unsafe fn rp_AcqGetTriggerState(state: *mut rp_acq_trig_state_t) -> c_int
{
    *state = rp_acq_trig_state_t::RP_TRIG_STATE_TRIGGERED;

    ok!()
}

pub unsafe fn rp_AcqGetWritePointer(pos: *mut u32) -> c_int
{
    *pos = 0;

    ok!()
}

pub unsafe fn rp_AcqGetWritePointerAtTrig(pos: *mut u32) -> c_int
{
    *pos = 0;

    ok!()
}

pub unsafe fn rp_AcqReset() -> c_int
{
    state!().acq = Default::default();

    ok!()
}

pub unsafe fn rp_AcqSetArmKeep(_enable: bool) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqSetAveraging(enabled: bool) -> c_int
{
    state!().acq.averaging = enabled;

    ok!()
}

pub unsafe fn rp_AcqSetDecimation(decimation: rp_acq_decimation_t) -> c_int
{
    state!().acq.decimation = decimation;

    ok!()
}

pub unsafe fn rp_AcqSetGain(channel: rp_channel_t, state: rp_pinState_t) -> c_int
{
    state!().acq.gain[channel as usize] = state;

    ok!()
}

pub unsafe fn rp_AcqSetSamplingRate(sampling_rate: rp_acq_sampling_rate_t) -> c_int
{
    state!().acq.decimation = sampling_rate.into();

    ok!()
}

pub unsafe fn rp_AcqSetTriggerDelay(decimated_data_num: i32) -> c_int
{
    let decimation: u32 = state!().acq.decimation.into();

    state!().acq.trigger_delay = (decimated_data_num as u32 * ADC_SAMPLE_PERIOD * decimation) as i64;

    ok!()
}

pub unsafe fn rp_AcqSetTriggerDelayNs(time_ns: i64) -> c_int
{
    state!().acq.trigger_delay = time_ns;

    ok!()
}

pub unsafe fn rp_AcqSetTriggerHyst(voltage: f32) -> c_int
{
    state!().acq.trigger_hyst = voltage;

    ok!()
}

pub unsafe fn rp_AcqSetTriggerLevel(channel: rp_channel_t, voltage: f32) -> c_int
{
    state!().acq.trigger_level = voltage;

    ok!()
}

pub unsafe fn rp_AcqSetTriggerSrc(source: rp_acq_trig_src_t) -> c_int
{
    state!().acq.trigger_src = source;

    ok!()
}

pub unsafe fn rp_AcqStart() -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqStop() -> c_int
{
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

pub unsafe fn rp_CalibInit() -> c_int
{
    ok!()
}

pub unsafe fn rp_CalibrateBackEnd(channel: rp_channel_t, out_params: *mut rp_calib_params_t) -> c_int
{
    *out_params = state!().calib;

    ok!()
}

pub unsafe fn rp_CalibrateBackEndOffset(channel: rp_channel_t) -> c_int
{
    ok!()
}

pub unsafe fn rp_CalibrateBackEndScale(channel: rp_channel_t) -> c_int
{
    ok!()
}

pub unsafe fn rp_CalibrateFrontEndOffset(channel: rp_channel_t, gain: rp_pinState_t, out_params: *mut rp_calib_params_t) -> c_int
{
    ok!()
}

pub unsafe fn rp_CalibrateFrontEndScaleHV(channel: rp_channel_t, referentialVoltage: f32, out_params: *mut rp_calib_params_t) -> c_int
{
    ok!()
}

pub unsafe fn rp_CalibrateFrontEndScaleLV(channel: rp_channel_t, referentialVoltage: f32, out_params: *mut rp_calib_params_t) -> c_int
{
    ok!()
}

pub unsafe fn rp_CalibrationReset() -> c_int
{
    state!().calib = Default::default();

    ok!()
}

pub unsafe fn rp_CalibrationSetCachedParams() -> c_int
{
    ok!()
}

pub unsafe fn rp_CalibrationWriteParams(calib_params: rp_calib_params_t) -> c_int
{
    state!().calib = calib_params;

    ok!()
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

pub unsafe fn rp_EnableDigitalLoop(_enable: bool) -> c_int
{
    ok!()
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

pub unsafe fn rp_GenAmp(channel: rp_channel_t, amplitude: f32) -> c_int
{
    state!().gen.amp[channel as usize] = amplitude;

    ok!()
}

pub unsafe fn rp_GenArbWaveform(channel: rp_channel_t, waveform: *mut f32, length: u32) -> c_int
{
    todo!()
}

pub unsafe fn rp_GenBurstCount(channel: rp_channel_t, num: c_int) -> c_int
{
    state!().gen.burst_count[channel as usize] = num;

    ok!()
}

pub unsafe fn rp_GenBurstPeriod(channel: rp_channel_t, period: u32) -> c_int
{
    state!().gen.burst_period[channel as usize] = period;

    ok!()
}

pub unsafe fn rp_GenBurstRepetitions(channel: rp_channel_t, repetitions: c_int) -> c_int
{
    state!().gen.burst_repetitions[channel as usize] = repetitions;

    ok!()
}

pub unsafe fn rp_GenDutyCycle(channel: rp_channel_t, ratio: f32) -> c_int
{
    state!().gen.duty_cycle[channel as usize] = ratio;

    ok!()
}

pub unsafe fn rp_GenFreq(channel: rp_channel_t, frequency: f32) -> c_int
{
    state!().gen.freq[channel as usize] = frequency;

    ok!()
}

pub unsafe fn rp_GenGetAmp(channel: rp_channel_t, amplitude: *mut f32) -> c_int
{
    *amplitude = state!().gen.amp[channel as usize];

    ok!()
}

pub unsafe fn rp_GenGetArbWaveform(channel: rp_channel_t, waveform: *mut f32, length: *mut u32) -> c_int
{
    todo!()
}

pub unsafe fn rp_GenGetBurstCount(channel: rp_channel_t, num: *mut c_int) -> c_int
{
    *num = state!().gen.burst_count[channel as usize];

    ok!()
}

pub unsafe fn rp_GenGetBurstPeriod(channel: rp_channel_t, period: *mut u32) -> c_int
{
    *period = state!().gen.burst_period[channel as usize];

    ok!()
}

pub unsafe fn rp_GenGetBurstRepetitions(channel: rp_channel_t, repetitions: *mut c_int) -> c_int
{
    *repetitions = state!().gen.burst_repetitions[channel as usize];

    ok!()
}

pub unsafe fn rp_GenGetDutyCycle(channel: rp_channel_t, ratio: *mut f32) -> c_int
{
    *ratio = state!().gen.duty_cycle[channel as usize];

    ok!()
}

pub unsafe fn rp_GenGetFreq(channel: rp_channel_t, frequency: *mut f32) -> c_int
{
    *frequency = state!().gen.freq[channel as usize];

    ok!()
}

pub unsafe fn rp_GenGetMode(channel: rp_channel_t, mode: *mut rp_gen_mode_t) -> c_int
{
    *mode = state!().gen.modes[channel as usize];

    ok!()
}

pub unsafe fn rp_GenGetOffset(channel: rp_channel_t, offset: *mut f32) -> c_int
{
    *offset = state!().gen.offsets[channel as usize];

    ok!()
}

pub unsafe fn rp_GenGetPhase(channel: rp_channel_t, phase: *mut f32) -> c_int
{
    *phase = state!().gen.phases[channel as usize];

    ok!()
}

pub unsafe fn rp_GenGetTriggerSource(channel: rp_channel_t, src: *mut rp_trig_src_t) -> c_int
{
    *src = state!().gen.trigger_src[channel as usize];

    ok!()
}

pub unsafe fn rp_GenGetWaveform(channel: rp_channel_t, type_: *mut rp_waveform_t) -> c_int
{
    *type_ = state!().gen.waveforms[channel as usize];

    ok!()
}

pub unsafe fn rp_GenMode(channel: rp_channel_t, mode: rp_gen_mode_t) -> c_int
{
    state!().gen.modes[channel as usize] = mode;

    ok!()
}

pub unsafe fn rp_GenOffset(channel: rp_channel_t, offset: f32) -> c_int
{
    state!().gen.offsets[channel as usize] = offset;

    ok!()
}

pub unsafe fn rp_GenOutDisable(channel: rp_channel_t) -> c_int
{
    state!().gen.out_enable[channel as usize] = false;

    ok!()
}

pub unsafe fn rp_GenOutEnable(channel: rp_channel_t) -> c_int
{
    state!().gen.out_enable[channel as usize] = true;

    ok!()
}

pub unsafe fn rp_GenOutIsEnabled(channel: rp_channel_t, value: *mut bool) -> c_int
{
    *value = state!().gen.out_enable[channel as usize];

    ok!()
}

pub unsafe fn rp_GenPhase(channel: rp_channel_t, phase: f32) -> c_int
{
    state!().gen.phases[channel as usize] = phase;

    ok!()
}

pub unsafe fn rp_GenReset() -> c_int
{
    state!().gen = Default::default();

    ok!()
}

pub unsafe fn rp_GenTrigger(channel: u32) -> c_int
{
    ok!()
}

pub unsafe fn rp_GenTriggerSource(channel: rp_channel_t, src: rp_trig_src_t) -> c_int
{
    state!().gen.trigger_src[channel as usize] = src;

    ok!()
}

pub unsafe fn rp_GenWaveform(channel: rp_channel_t, type_: rp_waveform_t) -> c_int
{
    state!().gen.waveforms[channel as usize] = type_;

    ok!()
}

pub unsafe fn rp_GetCalibrationSettings() -> rp_calib_params_t
{
    state!().calib
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
    *dna = 1;

    ok!()
}

pub unsafe fn rp_IdGetID(id: *mut u32) -> c_int
{
    *id = 2;

    ok!()
}

pub unsafe fn rp_Init() -> c_int
{
    ok!()
}

pub unsafe fn rp_LEDGetState(state: *mut u32) -> c_int
{
    *state = state!().led_state;

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

#![allow(dead_code, non_camel_case_types)]

extern crate libc;

use libc::{
    c_char, c_double, c_float, c_int, c_uint,
    int16_t, int32_t, int64_t,
    uint32_t, uint64_t
};

pub const ADC_BUFFER_SIZE: c_int = (16 * 1024);

pub const RP_OK: c_int = 0;
pub const RP_EOED: c_int = 1;
pub const RP_EOMD: c_int = 2;
pub const RP_ECMD: c_int = 3;
pub const RP_EMMD: c_int = 4;
pub const RP_EUMD: c_int = 5;
pub const RP_EOOR: c_int = 6;
pub const RP_ELID: c_int = 7;
pub const RP_EMRO: c_int = 8;
pub const RP_EWIP: c_int = 9;
pub const RP_EPN: c_int = 10;
pub const RP_UIA: c_int = 11;
pub const RP_FCA: c_int = 12;
pub const RP_RCA: c_int = 13;
pub const RP_BTS: c_int = 14;
pub const RP_EIPV: c_int = 15;
pub const RP_EUF: c_int = 16;
pub const RP_ENN: c_int = 17;
pub const RP_EFOB: c_int = 18;
pub const RP_EFCB: c_int = 19;
pub const RP_EABA: c_int = 20;
pub const RP_EFRB: c_int = 21;
pub const RP_EFWB: c_int = 22;
pub const RP_EMNC: c_int = 23;

pub const SPECTR_OUT_SIG_LEN: c_int = (2 * 1024);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum rp_pinState_t {
    RP_LOW,
    RP_HIGH,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum rp_pinDirection_t {
    RP_IN,
    RP_OUT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum rp_gen_mode_t {
    RP_GEN_MODE_CONTINUOUS,
    RP_GEN_MODE_BURST,
    RP_GEN_MODE_STREAM,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum rp_trig_src_t {
    RP_GEN_TRIG_SRC_INTERNAL = 1,
    RP_GEN_TRIG_SRC_EXT_PE = 2,
    RP_GEN_TRIG_SRC_EXT_NE = 3,
    RP_GEN_TRIG_GATED_BURST,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum rp_channel_t {
    RP_CH_1,
    RP_CH_2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum rp_acq_sampling_rate_t {
    RP_SMP_125M,
    RP_SMP_15_625M,
    RP_SMP_1_953M,
    RP_SMP_122_070K,
    RP_SMP_15_258K,
    RP_SMP_1_907K,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum rp_acq_decimation_t {
    RP_DEC_1,
    RP_DEC_8,
    RP_DEC_64,
    RP_DEC_1024,
    RP_DEC_8192,
    RP_DEC_65536,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum rp_acq_trig_state_t {
    RP_TRIG_STATE_TRIGGERED,
    RP_TRIG_STATE_WAITING,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct rp_calib_params_t {
    fe_ch1_fs_g_hi: uint32_t,
    fe_ch2_fs_g_hi: uint32_t,
    fe_ch1_fs_g_lo: uint32_t,
    fe_ch2_fs_g_lo: uint32_t,
    fe_ch1_lo_offs: int32_t,
    fe_ch2_lo_offs: int32_t,
    be_ch1_fs: uint32_t,
    be_ch2_fs: uint32_t,
    be_ch1_dc_offs: int32_t,
    be_ch2_dc_offs: int32_t,
    magic: uint32_t,
    fe_ch1_hi_offs: int32_t,
    fe_ch2_hi_offs: int32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wf_func_table_t {
    rp_spectr_wf_init: fn() -> c_int,
    rp_spectr_wf_clean: fn() -> c_int,
    rp_spectr_wf_clean_map: fn() -> c_int,
    rp_spectr_wf_calc: fn(cha_in: *mut c_double, chb_in: *mut c_double, koeff: c_float, koeff2: c_float) -> c_int,
    rp_spectr_wf_save_jpeg:fn(wf_file1: *const c_char, wf_file2: *const c_char) -> c_int,
}

extern {
    pub fn rp_Init() -> c_int;
    pub fn rp_CalibInit() -> c_int;
    pub fn rp_Release() -> c_int;
    pub fn rp_Reset() -> c_int;
    pub fn rp_GetVersion() -> *const c_char;
    pub fn rp_GetError(errorCode: c_int) -> *const c_char;

    pub fn rp_EnableDigitalLoop(enable: c_int) -> c_int;

    pub fn rp_GetCalibrationSettings() -> rp_calib_params_t;
    pub fn rp_CalibrateFrontEndOffset(channel: rp_channel_t, gain: rp_pinState_t, out_params: *mut rp_calib_params_t) -> c_int;
    pub fn rp_CalibrateFrontEndScaleLV(channel: rp_channel_t, referentialVoltage: c_float, out_params: *mut rp_calib_params_t) -> c_int;
    pub fn rp_CalibrateFrontEndScaleHV(channel: rp_channel_t, referentialVoltage: c_float, out_params: *mut rp_calib_params_t) -> c_int;
    pub fn rp_CalibrateBackEndOffset(channel: rp_channel_t) -> c_int;
    pub fn rp_CalibrateBackEndScale(channel: rp_channel_t) -> c_int;
    pub fn rp_CalibrateBackEnd(channel: rp_channel_t, out_params: *mut rp_calib_params_t) -> c_int;
    pub fn rp_CalibrationReset() -> c_int;
    pub fn rp_CalibrationSetCachedParams() -> c_int;
    pub fn rp_CalibrationWriteParams(calib_params: rp_calib_params_t) -> c_int;

    pub fn rp_IdGetID(id: *mut uint32_t) -> c_int;
    pub fn rp_IdGetDNA(dna: *mut uint64_t) -> c_int;

    pub fn rp_LEDSetState(state: uint32_t) -> c_int;
    pub fn rp_LEDGetState(state: *mut uint32_t) -> c_int;

    pub fn rp_GPIOnSetDirection(direction: uint32_t) -> c_int;
    pub fn rp_GPIOnGetDirection(direction: *mut uint32_t) -> c_int;
    pub fn rp_GPIOnSetState(state: uint32_t) -> c_int;
    pub fn rp_GPIOnGetState(state: *mut uint32_t) -> c_int;
    pub fn rp_GPIOpSetDirection(direction: uint32_t) -> c_int;
    pub fn rp_GPIOpGetDirection(direction: *mut uint32_t) -> c_int;
    pub fn rp_GPIOpSetState(state: uint32_t) -> c_int;
    pub fn rp_GPIOpGetState(state: *mut uint32_t) -> c_int;

    pub fn rp_DpinReset() -> c_int;
    pub fn rp_DpinSetState(pin: rp_dpin_t, state: rp_pinState_t) -> c_int;
    pub fn rp_DpinGetState(pin: rp_dpin_t, state: *mut rp_pinState_t) -> c_int;
    pub fn rp_DpinSetDirection(pin: rp_dpin_t, direction: rp_pinDirection_t) -> c_int;
    pub fn rp_DpinGetDirection(pin: rp_dpin_t, direction: *mut rp_pinDirection_t) -> c_int;

    pub fn rp_ApinReset() -> c_int;
    pub fn rp_ApinGetValue(pin: rp_apin_t, value: *mut c_float) -> c_int;
    pub fn rp_ApinGetValueRaw(pin: rp_apin_t, value: *mut uint32_t) -> c_int;
    pub fn rp_ApinSetValue(pin: rp_apin_t, value: c_float) -> c_int;
    pub fn rp_ApinSetValueRaw(pin: rp_apin_t, value: uint32_t) -> c_int;
    pub fn rp_ApinGetRange(pin: rp_apin_t, min_val: *mut c_float, max_val: *mut c_float) -> c_int;

    pub fn rp_AIpinGetValue(pin: c_uint, value: *mut c_float) -> c_int;
    pub fn rp_AIpinGetValueRaw(pin: c_uint, value: *mut uint32_t) -> c_int;

    pub fn rp_AOpinReset() -> c_int;
    pub fn rp_AOpinGetValue(pin: c_uint, value: *mut c_float) -> c_int;
    pub fn rp_AOpinGetValueRaw(pin: c_uint, value: *mut uint32_t) -> c_int;
    pub fn rp_AOpinSetValue(pin: c_uint, value: c_float) -> c_int;
    pub fn rp_AOpinSetValueRaw(pin: c_uint, value: uint32_t) -> c_int;
    pub fn rp_AOpinGetRange(pin: c_uint, min_val: *mut c_float, max_val: *mut c_float) -> c_int;

    pub fn rp_AcqSetArmKeep(enable: c_int) -> c_int;
    pub fn rp_AcqSetDecimation(decimat: rp_acq_decimation_t) -> c_int;
    pub fn rp_AcqGetDecimation(decimat: *mut rp_acq_decimation_t) -> c_int;
    pub fn rp_AcqGetDecimationFactor(decimation: *mut uint32_t) -> c_int;
    pub fn rp_AcqSetSamplingRate(sampling_rate: rp_acq_sampling_rate_t ) -> c_int;
    pub fn rp_AcqGetSamplingRate(sampling_rate: *mut rp_acq_sampling_rate_t) -> c_int;
    pub fn rp_AcqGetSamplingRateHz(sampling_rate: *mut c_float) -> c_int;
    pub fn rp_AcqSetAveraging(enabled: c_int) -> c_int;
    pub fn rp_AcqGetAveraging(enabled: *mut c_int) -> c_int;
    pub fn rp_AcqSetTriggerSrc(source: rp_acq_trig_src_t) -> c_int;
    pub fn rp_AcqGetTriggerSrc(source: *mut rp_acq_trig_src_t) -> c_int;
    pub fn rp_AcqGetTriggerState(state: *mut rp_acq_trig_state_t) -> c_int;
    pub fn rp_AcqSetTriggerDelay(decimated_data_num: int32_t) -> c_int;
    pub fn rp_AcqGetTriggerDelay(decimated_data_num: *mut int32_t) -> c_int;
    pub fn rp_AcqSetTriggerDelayNs(time_ns: int64_t) -> c_int;
    pub fn rp_AcqGetTriggerDelayNs(time_ns: *mut int64_t) -> c_int;
    pub fn rp_AcqGetPreTriggerCounter(value: *mut uint32_t) -> c_int;
    pub fn rp_AcqSetTriggerLevel(volatage: c_float) -> c_int;
    pub fn rp_AcqGetTriggerLevel(volatage: *mut c_float) -> c_int;
    pub fn rp_AcqSetTriggerHyst(volatage: c_float) -> c_int;
    pub fn rp_AcqGetTriggerHyst(volatage: *mut c_float) -> c_int;
    pub fn rp_AcqSetGain(channel: rp_channel_t, state: rp_pinState_t) -> c_int;
    pub fn rp_AcqGetGain(channel: rp_channel_t, state: *mut rp_pinState_t) -> c_int;
    pub fn rp_AcqGetGainV(channel: rp_channel_t, voltage: *mut c_float) -> c_int;
    pub fn rp_AcqGetWritePointer(pos: *mut uint32_t) -> c_int;
    pub fn rp_AcqGetWritePointerAtTrig(pos: *mut uint32_t) -> c_int;
    pub fn rp_AcqStart() -> c_int;
    pub fn rp_AcqStop() -> c_int;
    pub fn rp_AcqReset() -> c_int;
    pub fn rp_AcqGetNormalizedDataPos(pos: uint32_t) -> uint32_t;
    pub fn rp_AcqGetDataPosRaw(channel: rp_channel_t, start_pos: uint32_t, end_pos: uint32_t, buffer: *mut int16_t, buffer_size: *mut uint32_t) -> c_int;
    pub fn rp_AcqGetDataPosV(channel: rp_channel_t, start_pos: uint32_t, end_pos: uint32_t, buffer: *mut c_float, buffer_size: *mut uint32_t) -> c_int;
    pub fn rp_AcqGetDataRaw(channel: rp_channel_t, pos: uint32_t, size: *mut uint32_t, buffer: *mut int16_t) -> c_int;
    pub fn rp_AcqGetDataRawV2(pos: uint32_t, size: *mut uint32_t, buffer1: *mut int16_t, buffer2: *mut int16_t) -> c_int;
    pub fn rp_AcqGetOldestDataRaw(channel: rp_channel_t, size: *mut uint32_t, buffer: *mut int16_t) -> c_int;
    pub fn rp_AcqGetLatestDataRaw(channel: rp_channel_t, size: *mut uint32_t, buffer: *mut int16_t) -> c_int;
    pub fn rp_AcqGetDataV(channel: rp_channel_t, pos: uint32_t, size: *mut uint32_t, buffer: *mut c_float) -> c_int;
    pub fn rp_AcqGetDataV2(pos: uint32_t, size: *mut uint32_t, buffer1: *mut c_float, buffer2: *mut c_float) -> c_int;
    pub fn rp_AcqGetOldestDataV(channel: rp_channel_t, size: *mut uint32_t, buffer: *mut c_float) -> c_int;
    pub fn rp_AcqGetLatestDataV(channel: rp_channel_t, size: *mut uint32_t, buffer: *mut c_float) -> c_int;
    pub fn rp_AcqGetBufSize(size: *mut uint32_t) -> c_int;

    pub fn rp_GenReset() -> c_int;
    pub fn rp_GenOutEnable(channel: rp_channel_t) -> c_int;
    pub fn rp_GenOutDisable(channel: rp_channel_t) -> c_int;
    pub fn rp_GenOutIsEnabled(channel: rp_channel_t, value: *mut c_int) -> c_int;
    pub fn rp_GenAmp(channel: rp_channel_t, amplitude: c_float) -> c_int;
    pub fn rp_GenGetAmp(channel: rp_channel_t, amplitude: *mut c_float) -> c_int;
    pub fn rp_GenOffset(channel: rp_channel_t, offset: c_float) -> c_int;
    pub fn rp_GenGetOffset(channel: rp_channel_t, offset: *mut c_float) -> c_int;
    pub fn rp_GenFreq(channel: rp_channel_t, frequency: c_float) -> c_int;
    pub fn rp_GenGetFreq(channel: rp_channel_t, frequency: *mut c_float) -> c_int;
    pub fn rp_GenPhase(channel: rp_channel_t, phase: c_float) -> c_int;
    pub fn rp_GenGetPhase(channel: rp_channel_t, phase: *mut c_float) -> c_int;
    pub fn rp_GenWaveform(channel: rp_channel_t, waveform: rp_waveform_t) -> c_int;
    pub fn rp_GenGetWaveform(channel: rp_channel_t, waveform: *mut rp_waveform_t) -> c_int;
    pub fn rp_GenArbWaveform(channel: rp_channel_t, waveform: *mut c_float, length: uint32_t) -> c_int;
    pub fn rp_GenGetArbWaveform(channel: rp_channel_t, waveform: *mut c_float, length: *mut uint32_t) -> c_int;
    pub fn rp_GenDutyCycle(channel: rp_channel_t, ratio: c_float) -> c_int;
    pub fn rp_GenGetDutyCycle(channel: rp_channel_t, ratio: *mut c_float) -> c_int;
    pub fn rp_GenMode(channel: rp_channel_t, mode: rp_gen_mode_t) -> c_int;
    pub fn rp_GenGetMode(channel: rp_channel_t, mode: *mut rp_gen_mode_t) -> c_int;
    pub fn rp_GenBurstCount(channel: rp_channel_t, num: c_int) -> c_int;
    pub fn rp_GenGetBurstCount(channel: rp_channel_t, num: *mut c_int) -> c_int;
    pub fn rp_GenBurstRepetitions(channel: rp_channel_t, repetitions: c_int) -> c_int;
    pub fn rp_GenGetBurstRepetitions(channel: rp_channel_t, repetitions: *mut c_int) -> c_int;
    pub fn rp_GenBurstPeriod(channel: rp_channel_t, period: uint32_t) -> c_int;
    pub fn rp_GenGetBurstPeriod(channel: rp_channel_t, period: *mut uint32_t) -> c_int;
    pub fn rp_GenTriggerSource(channel: rp_channel_t, src: rp_trig_src_t) -> c_int;
    pub fn rp_GenGetTriggerSource(channel: rp_channel_t, src: *mut rp_trig_src_t) -> c_int;
    pub fn rp_GenTrigger(channel: uint32_t) -> c_int;

    pub fn rp_CmnCnvCntToV(field_len: uint32_t, cnts: uint32_t, adc_max_v: c_float, calibScale: uint32_t, calib_dc_off: c_int, user_dc_off: c_float) -> c_float;
}

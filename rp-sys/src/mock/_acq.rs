struct AcqState {
    decimation: rp_acq_decimation_t,
    averaging: bool,
    gain: Vec<rp_pinState_t>,
    trigger_src: rp_acq_trig_src_t,
    trigger_delay: i64,
    trigger_hyst: f32,
    trigger_level: Vec<f32>,
    arm_keep: bool,
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
            trigger_level: vec![1.; 3], // @FIXME
            arm_keep: false, // @FIXME
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_acq_decimation_t {
    RP_DEC_1,
    #[cfg(feature = "v1_04")]
    RP_DEC_2,
    #[cfg(feature = "v1_04")]
    RP_DEC_4,
    RP_DEC_8,
    #[cfg(feature = "v1_04")]
    RP_DEC_16,
    #[cfg(feature = "v1_04")]
    RP_DEC_32,
    RP_DEC_64,
    #[cfg(feature = "v1_04")]
    RP_DEC_128,
    #[cfg(feature = "v1_04")]
    RP_DEC_256,
    #[cfg(feature = "v1_04")]
    RP_DEC_512,
    RP_DEC_1024,
    #[cfg(feature = "v1_04")]
    RP_DEC_2048,
    #[cfg(feature = "v1_04")]
    RP_DEC_4096,
    RP_DEC_8192,
    #[cfg(feature = "v1_04")]
    RP_DEC_16384,
    #[cfg(feature = "v1_04")]
    RP_DEC_32768,
    RP_DEC_65536,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_acq_sampling_rate_t {
    RP_SMP_125M,
    #[cfg(feature = "v1_04")]
    RP_SMP_62_500M,
    #[cfg(feature = "v1_04")]
    RP_SMP_31_250M,
    RP_SMP_15_625M,
    #[cfg(feature = "v1_04")]
    RP_SMP_7_812M,
    #[cfg(feature = "v1_04")]
    RP_SMP_3_906M,
    RP_SMP_1_953M,
    #[cfg(feature = "v1_04")]
    RP_SMP_976_562K,
    #[cfg(feature = "v1_04")]
    RP_SMP_448_281K,
    #[cfg(feature = "v1_04")]
    RP_SMP_244_140K,
    RP_SMP_122_070K,
    #[cfg(feature = "v1_04")]
    RP_SMP_61_035K,
    #[cfg(feature = "v1_04")]
    RP_SMP_30_517K,
    RP_SMP_15_258K,
    #[cfg(feature = "v1_04")]
    RP_SMP_7_629K,
    #[cfg(feature = "v1_04")]
    RP_SMP_3_814K,
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

pub unsafe fn rp_AcqGetAveraging(enabled: *mut bool) -> c_int
{
    unsafe {
        *enabled = state!().acq.averaging;
    }

    ok!()
}

pub unsafe fn rp_AcqGetBufSize(size: *mut u32) -> c_int
{
    unsafe {
        *size = ADC_BUFFER_SIZE;
    }

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
    unsafe {
        *decimation = state!().acq.decimation;
    }

    ok!()
}

pub unsafe fn rp_AcqSetDecimationFactor(decimation: u32) -> c_int
{
    state!().acq.decimation = decimation.into();

    ok!()
}

pub unsafe fn rp_AcqGetDecimationFactor(decimation: *mut u32) -> c_int
{
    unsafe {
        *decimation = state!().acq.decimation.into();
    }

    ok!()
}

pub unsafe fn rp_AcqGetGain(channel: rp_channel_t, state: *mut rp_pinState_t) -> c_int
{
    unsafe {
        *state = state!().acq.gain[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_AcqGetGainV(channel: rp_channel_t, voltage: *mut f32) -> c_int
{
    let mut state = rp_pinState_t::RP_LOW;

    unsafe {
        rp_AcqGetGain(channel, &mut state);
    }

    if state == rp_pinState_t::RP_LOW {
        unsafe {
            *voltage = 1.0;
        }
    }
    else {
        unsafe {
            *voltage = 20.0;
        }
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
    unsafe {
        *value = 10;
    }

    ok!()
}

pub unsafe fn rp_AcqGetSamplingRate(sampling_rate: *mut rp_acq_sampling_rate_t) -> c_int
{
    unsafe {
        *sampling_rate = state!().acq.decimation.into();
    }

    ok!()
}

pub unsafe fn rp_AcqGetSamplingRateHz(sampling_rate: *mut f32) -> c_int
{
    let max_rate: f32 = 125_000_000.;
    let decimation: u32 = state!().acq.decimation.into();

    unsafe {
        *sampling_rate = max_rate / decimation as f32;
    }

    0
}

pub unsafe fn rp_AcqGetTriggerDelay(decimated_data_num: *mut i32) -> c_int
{
    let delay = state!().acq.trigger_delay;
    let decimation: u32 = state!().acq.decimation.into();

    unsafe {
        *decimated_data_num = (delay / ADC_SAMPLE_PERIOD as i64 / decimation as i64) as i32;
    }

    ok!()
}

pub unsafe fn rp_AcqGetTriggerDelayNs(time_ns: *mut i64) -> c_int
{
    unsafe {
        *time_ns = state!().acq.trigger_delay;
    }

    ok!()
}

pub unsafe fn rp_AcqGetTriggerHyst(voltage: *mut f32) -> c_int
{
    unsafe {
        *voltage = state!().acq.trigger_hyst;
    }

    ok!()
}

pub unsafe fn rp_AcqGetTriggerLevel(channel: rp_channel_trigger_t, voltage: *mut f32) -> c_int
{
    unsafe {
        *voltage = state!().acq.trigger_level[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_AcqGetTriggerSrc(source: *mut rp_acq_trig_src_t) -> c_int
{
    unsafe {
        *source = state!().acq.trigger_src;
    }

    ok!()
}

pub unsafe fn rp_AcqGetTriggerState(state: *mut rp_acq_trig_state_t) -> c_int
{
    unsafe {
        *state = rp_acq_trig_state_t::RP_TRIG_STATE_TRIGGERED;
    }

    ok!()
}

pub unsafe fn rp_AcqGetWritePointer(pos: *mut u32) -> c_int
{
    unsafe {
        *pos = 0;
    }

    ok!()
}

pub unsafe fn rp_AcqGetWritePointerAtTrig(pos: *mut u32) -> c_int
{
    unsafe {
        *pos = 0;
    }

    ok!()
}

pub unsafe fn rp_AcqReset() -> c_int
{
    state!().acq = Default::default();

    ok!()
}

pub unsafe fn rp_AcqSetArmKeep(enable: bool) -> c_int
{
    state!().acq.arm_keep = enable;

    ok!()
}

pub unsafe fn rp_AcqGetArmKeep(state: &mut bool) -> c_int
{
    *state = state!().acq.arm_keep;

    ok!()
}

pub unsafe fn rp_AcqSetAveraging(enabled: bool) -> c_int
{
    state!().acq.averaging = enabled;

    ok!()
}

pub unsafe fn rp_AcqGetBufferFillState(state: &mut bool) -> c_int
{
    *state = true;

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

pub unsafe fn rp_AcqSetTriggerLevel(channel: rp_channel_trigger_t, voltage: f32) -> c_int
{
    state!().acq.trigger_level[channel as usize] = voltage;

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

pub unsafe fn rp_AcqUpdateAcqFilter(channel: rp_channel_t) -> c_int
{
    ok!()
}

pub unsafe fn rp_AcqGetFilterCalibValue(channel: rp_channel_t, coef_aa: *mut u32, coef_bb: *mut u32, coef_kk: *mut u32, coef_pp: *mut u32) -> c_int
{
    ok!()
}

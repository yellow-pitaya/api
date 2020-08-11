/**
 * Type representing Input/Output channels in trigger.
 */
pub use crate::rp::rp_channel_trigger_t as Channel;

/**
 * Type representing decimation used at acquiring signal.
 */
pub use crate::rp::rp_acq_decimation_t as Decimation;

/**
 * Type representing acquire signal sampling rate.
 */
pub use crate::rp::rp_acq_sampling_rate_t as SamplingRate;

/**
 * Type representing different trigger sources used at acquiring signal.
 */
pub use crate::rp::rp_acq_trig_src_t as TrigSrc;

/**
 * Type representing different trigger states.
 */
pub use crate::rp::rp_acq_trig_state_t as TrigState;

pub enum Gain {
    LV,
    HV,
}

impl std::convert::Into<super::pin::State> for Gain {
    fn into(self) -> super::pin::State {
        match self {
            Gain::LV => super::pin::State::RP_LOW,
            Gain::HV => super::pin::State::RP_HIGH,
        }
    }
}

impl std::convert::From<super::pin::State> for Gain {
    fn from(state: super::pin::State) -> Self {
        match state {
            super::pin::State::RP_LOW => Gain::LV,
            super::pin::State::RP_HIGH => Gain::HV,
        }
    }
}

impl std::convert::From<String> for Gain {
    fn from(direction: String) -> Self {
        match direction.as_str() {
            "LV" => Gain::LV,
            "HV" => Gain::HV,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<String> for Gain {
    fn into(self) -> String {
        match self {
            Gain::LV => "LV",
            Gain::HV => "HV",
        }.to_owned()
    }
}

/**
 * Enables continous acquirement even after trigger has happened.
 */
pub fn set_arm_keep(enable: bool) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetArmKeep(enable)
    )
}

/**
 * Gets status of continous acquirement even after trigger has happened.
 */
pub fn get_arm_keep() -> crate::Result<bool>
{
    let mut state = false;

    match handle_unsafe!(
        crate::rp::rp_AcqGetArmKeep(&mut state)
    ) {
        Ok(_) => Ok(state),
        Err(err) => Err(err),
    }
}

/**
 * Indicates whether the ADC buffer was full of data. The length of the buffer is determined by the
 * delay. By default, the delay is half the buffer.
 */
pub fn get_buffer_fill_state() -> crate::Result<bool>
{
    let mut state = false;

    match handle_unsafe!(
        crate::rp::rp_AcqGetBufferFillState(&mut state)
    ) {
        Ok(_) => Ok(state),
        Err(err) => Err(err),
    }
}

/**
 * Sets the decimation used at acquiring signal.
 *
 * There is only a set of pre-defined decimation
 *
 * values which can be specified. See the `Decimation` enum values.
 */
pub fn set_decimation(decimat: Decimation) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetDecimation(decimat)
    )
}

/**
 * Gets the decimation used at acquiring signal.
 *
 * There is only a set of pre-defined decimation values which can be specified.
 * See the `Decimation` enum values.
 */
pub fn get_decimation() -> crate::Result<Decimation>
{
    let mut decimat = Decimation::RP_DEC_1;

    match handle_unsafe!(
        crate::rp::rp_AcqGetDecimation(&mut decimat)
    ) {
        Ok(_) => Ok(decimat),
        Err(err) => Err(err),
    }
}

/**
 * Sets the decimation used at acquiring signal.
 */
pub fn acq_set_decimation_factor(decimation: Decimation) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetDecimationFactor(decimation.into())
    )
}

/**
 * Gets the decimation factor used at acquiring signal in a numerical form.
 *
 * Although this method returns an integer value representing the current factor
 * of the decimation, there is only a set of pre-defined decimation factor
 * values which can be returned. See the `Decimation` enum values.
 */
pub fn get_decimation_factor() -> crate::Result<u32>
{
    let mut decimation = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetDecimationFactor(&mut decimation)
    ) {
        Ok(_) => Ok(decimation),
        Err(err) => Err(err),
    }
}

/**
 * Sets the sampling rate for acquiring signal.
 *
 * There is only a set of pre-defined sampling rate values which can be
 * specified. See the `SamplingRate` enum values.
 */
pub fn set_sampling_rate(sampling_rate: SamplingRate ) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetSamplingRate(sampling_rate)
    )
}

/**
 * Gets the sampling rate for acquiring signal.
 *
 * There is only a set of pre-defined sampling rate values which can be
 * returned. See the `SamplingRate` enum values.
 */
pub fn get_sampling_rate() -> crate::Result<SamplingRate>
{
    let mut sampling_rate = SamplingRate::RP_SMP_125M;

    match handle_unsafe!(
        crate::rp::rp_AcqGetSamplingRate(&mut sampling_rate)
    ) {
        Ok(_) => Ok(sampling_rate),
        Err(err) => Err(err),
    }
}

/**
 * Gets the sampling rate for acquiring signal in a numerical form in Hz.
 *
 * Although this method returns a float value representing the current value of
 * the sampling rate, there is only a set of pre-defined sampling rate values
 * which can be returned. See the `SamplingRate` enum values.
 */
pub fn get_sampling_rate_hz() -> crate::Result<f32>
{
    let mut sampling_rate = 0.0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetSamplingRateHz(&mut sampling_rate)
    ) {
        Ok(_) => Ok(sampling_rate),
        Err(err) => Err(err),
    }
}

/**
 * Enables or disables averaging of data between samples.
 *
 * Data between samples can be averaged by setting the averaging flag in the
 * Data decimation register.
 */
pub fn set_averaging(enabled: bool) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetAveraging(enabled)
    )
}

/**
 * Returns information if averaging of data between samples is enabled or
 * disabled.
 *
 * Data between samples can be averaged by setting the averaging flag in the
 * Data decimation register.
 */
pub fn get_averaging() -> crate::Result<bool>
{
    let mut enabled = false;

    match handle_unsafe!(
        crate::rp::rp_AcqGetAveraging(&mut enabled)
    ) {
        Ok(_) => Ok(enabled),
        Err(err) => Err(err),
    }
}

/**
 * Sets the trigger source used at acquiring signal.
 *
 * When acquiring is started, the FPGA waits for the trigger condition on the
 * specified source and when the condition is met, it starts writing the signal
 * to the buffer.
 */
pub fn set_trigger_src(source: TrigSrc) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetTriggerSrc(source)
    )
}

/**
 * Gets the trigger source used at acquiring signal.
 *
 * When acquiring is started, the FPGA waits for the trigger condition on the
 * specified source and when the condition is met, it starts writing the signal
 * to the buffer.
 */
pub fn get_trigger_src() -> crate::Result<TrigSrc>
{
    let mut source = TrigSrc::RP_TRIG_SRC_DISABLED;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerSrc(&mut source)
    ) {
        Ok(_) => Ok(source),
        Err(err) => Err(err),
    }
}

/**
 * Returns the trigger state.
 *
 * Either it is waiting for a trigger to happen, or it has already been
 * triggered.
 *
 * By default it is in the triggered state, which is treated the same as
 * disabled.
 */
pub fn get_trigger_state() -> crate::Result<TrigState>
{
    let mut state = TrigState::RP_TRIG_STATE_TRIGGERED;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerState(&mut state)
    ) {
        Ok(_) => Ok(state),
        Err(err) => Err(err),
    }
}

/**
 * Sets the number of decimated data after trigger written into memory.
 */
pub fn set_trigger_delay(decimated_data_num: i32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetTriggerDelay(decimated_data_num)
    )
}

/**
 * Returns current number of decimated data after trigger written into memory.
 */
pub fn get_trigger_delay() -> crate::Result<i32>
{
    let mut decimated_data_num = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerDelay(&mut decimated_data_num)
    ) {
        Ok(_) => Ok(decimated_data_num),
        Err(err) => Err(err),
    }
}

/**
 * Sets the amount of decimated data in nanoseconds after trigger written into
 * memory.
 */
pub fn set_trigger_delay_ns(time_ns: i64) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetTriggerDelayNs(time_ns)
    )
}

/**
 * Returns the current amount of decimated data in nanoseconds after trigger
 * written into memory.
 */
pub fn get_trigger_delay_ns() -> crate::Result<i64>
{
    let mut time_ns = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerDelayNs(&mut time_ns)
    ) {
        Ok(_) => Ok(time_ns),
        Err(err) => Err(err),
    }
}

/**
 * Returns the number of valid data ponts before trigger.
 */
pub fn get_pre_trigger_counter() -> crate::Result<u32>
{
    let mut value = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetPreTriggerCounter(&mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

/**
 * Sets the trigger threshold value in volts.
 *
 * Makes the trigger when ADC value crosses this value.
 */
pub fn set_trigger_level(channel: Channel, volatage: f32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetTriggerLevel(channel, volatage)
    )
}

/**
 * Gets currently set trigger threshold value in volts.
 */
pub fn get_trigger_level(channel: Channel) -> crate::Result<f32>
{
    let mut volatage = 0.0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerLevel(channel, &mut volatage)
    ) {
        Ok(_) => Ok(volatage),
        Err(err) => Err(err),
    }
}

/**
 * Sets the trigger threshold hysteresis value in volts.
 *
 * Value must be outside to enable the trigger again.
 */
pub fn set_trigger_hyst(volatage: f32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetTriggerHyst(volatage)
    )
}

/**
 * Gets currently set trigger threshold hysteresis value in volts.
 */
pub fn get_trigger_hyst() -> crate::Result<f32>
{
    let mut volatage = 0.0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerHyst(&mut volatage)
    ) {
        Ok(_) => Ok(volatage),
        Err(err) => Err(err),
    }
}

/**
 * Sets the acquire gain state.
 *
 * The gain should be set to the same value as it is set on the Red Pitaya
 * hardware by the LV/HV gain jumpers. LV = 1V; HV = 20V.
 */
pub fn set_gain(channel: super::Channel, gain: Gain) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetGain(channel, gain.into())
    )
}

/**
 * Returns the currently set acquire gain state in the library.
 *
 * It may not be set to the same value as it is set on the Red Pitaya hardware
 * by the LV/HV gain jumpers. LV = 1V; HV = 20V.
 */
pub fn get_gain(channel: super::Channel) -> crate::Result<Gain>
{
    let mut state = super::pin::State::RP_LOW;

    match handle_unsafe!(
        crate::rp::rp_AcqGetGain(channel, &mut state)
    ) {
        Ok(_) => Ok(state.into()),
        Err(err) => Err(err),
    }
}

/**
 * Returns the currently set acquire gain in the library.
 *
 * It may not be set to the same value as it is set on the Red Pitaya hardware
 * by the LV/HV gain jumpers. Returns value in Volts.
 */
pub fn get_gain_v(channel: super::Channel) -> crate::Result<f32>
{
    let mut voltage = 0.0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetGainV(channel, &mut voltage)
    ) {
        Ok(_) => Ok(voltage),
        Err(err) => Err(err),
    }
}

/**
 * Returns current position of ADC write pointer.
 */
pub fn get_write_pointer() -> crate::Result<u32>
{
    let mut pos = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetWritePointer(&mut pos)
    ) {
        Ok(_) => Ok(pos),
        Err(err) => Err(err),
    }
}

/**
 * Returns position of ADC write pointer at time when trigger arrived.
 */
pub fn get_write_pointer_at_trig() -> crate::Result<u32>
{
    let mut pos = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetWritePointerAtTrig(&mut pos)
    ) {
        Ok(_) => Ok(pos),
        Err(err) => Err(err),
    }
}

/**
 * Starts the acquire.
 *
 * Signals coming from the input channels are acquired and written into memory.
 */
pub fn start() -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqStart()
    )
}

/**
 * Stops the acquire.
 */
pub fn stop() -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqStop()
    )
}

/**
 * Resets the acquire writing state machine.
 */
pub fn reset() -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqReset()
    )
}

/**
 * Normalizes the ADC buffer position.
 *
 * Returns the modulo operation of ADC buffer size...
 */
pub fn get_normalized_data_pos(pos: u32) -> u32
{
    unsafe {
        crate::rp::rp_AcqGetNormalizedDataPos(pos)
    }
}

/**
 * Returns the ADC buffer in raw units from start to end position.
 */
pub fn get_data_pos_raw(channel: super::Channel, start_pos: u32, end_pos: u32) -> crate::Result<Vec<i16>>
{
    let mut slice = [0; 16_384];
    let mut length = slice.len() as u32 + 1;

    match handle_unsafe!(
        crate::rp::rp_AcqGetDataPosRaw(channel, start_pos, end_pos, slice.as_mut_ptr(), &mut length)
    ) {
        Ok(_) => {
            let mut vec = slice.to_vec();
            vec.truncate(length as usize);

            Ok(vec)
        },
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in Volt units from start to end position.
 */
pub fn get_data_pos_v(channel: super::Channel, start_pos: u32, end_pos: u32) -> crate::Result<Vec<f32>>
{
    let mut slice = [0.0; 16_384];
    let mut length = slice.len() as u32 + 1;

    match handle_unsafe!(
        crate::rp::rp_AcqGetDataPosV(channel, start_pos, end_pos, slice.as_mut_ptr(), &mut length)
    ) {
        Ok(_) => {
            let mut vec = slice.to_vec();
            vec.truncate(length as usize);

            Ok(vec)
        },
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in raw units from specified position.
 */
pub fn get_data_raw(channel: super::Channel, pos: u32, size: u32) -> crate::Result<Vec<i16>>
{
    let mut length = size;
    let mut slice = [0; 16_384];

    match handle_unsafe!(
        crate::rp::rp_AcqGetDataRaw(channel, pos, &mut length, slice.as_mut_ptr())
    ) {
        Ok(_) => {
            let mut vec = slice.to_vec();
            vec.truncate(length as usize);

            Ok(vec)
        },
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in raw units from specified position.
 */
pub fn get_data_raw_v2(pos: u32, size: u32) -> crate::Result<[Vec<u16>;2]>
{
    let mut length = size;
    let mut slice1 = [0; 16_384];
    let mut slice2 = [0; 16_384];

    match handle_unsafe!(
        crate::rp::rp_AcqGetDataRawV2(pos, &mut length, slice1.as_mut_ptr(), slice2.as_mut_ptr())
    ) {
        Ok(_) => {
            let mut vec1 = slice1.to_vec();
            vec1.truncate(length as usize);

            let mut vec2 = slice2.to_vec();
            vec2.truncate(length as usize);

            Ok([vec1, vec2])
        },
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in raw units from the oldest sample to the newest one.
 *
 * CAUTION: Use this method only when write pointer has stopped (Trigger
 * happened and writing stopped).
 */
pub fn get_oldest_data_raw(channel: super::Channel, size: u32) -> crate::Result<Vec<i16>>
{
    let mut length = size;
    let mut slice = [0; 16_384];

    match handle_unsafe!(
        crate::rp::rp_AcqGetOldestDataRaw(channel, &mut length, slice.as_mut_ptr())
    ) {
        Ok(_) => {
            let mut vec = slice.to_vec();
            vec.truncate(length as usize);

            Ok(vec)
        },
        Err(err) => Err(err),
    }
}

/**
 * Returns the latest ADC buffer samples in raw units.
 */
pub fn get_latest_data_raw(channel: super::Channel, size: u32) -> crate::Result<Vec<i16>>
{
    let mut length = size;
    let mut slice = [0; 16_384];

    match handle_unsafe!(
        crate::rp::rp_AcqGetLatestDataRaw(channel, &mut length, slice.as_mut_ptr())
    ) {
        Ok(_) => {
            let mut vec = slice.to_vec();
            vec.truncate(length as usize);

            Ok(vec)
        },
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in Volt units from specified position.
 */
pub fn get_data_v(channel: super::Channel, pos: u32, size: u32) -> crate::Result<Vec<f32>>
{
    let mut length = size;
    let mut slice = [0.0; 16_384];

    match handle_unsafe!(
        crate::rp::rp_AcqGetDataV(channel, pos, &mut length, slice.as_mut_ptr())
    ) {
        Ok(_) => {
            let mut vec = slice.to_vec();
            vec.truncate(length as usize);

            Ok(vec)
        },
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in Volt units from specified position.
 */
pub fn get_data_v2(pos: u32, size: u32) -> crate::Result<[Vec<f32>;2]>
{
    let mut length = size;
    let mut slice1 = [0.0; 16_384];
    let mut slice2 = [0.0; 16_384];

    match handle_unsafe!(
        crate::rp::rp_AcqGetDataV2(pos, &mut length, slice1.as_mut_ptr(), slice2.as_mut_ptr())
    ) {
        Ok(_) => {
            let mut vec1 = slice1.to_vec();
            vec1.truncate(length as usize);

            let mut vec2 = slice2.to_vec();
            vec2.truncate(length as usize);

            Ok([vec1, vec2])
        },
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in Volt units from the oldest sample to the newest one.
 *
 * CAUTION: Use this method only when write pointer has stopped (Trigger happened and writing stopped).
 */
pub fn get_oldest_data_v(channel: super::Channel, size: u32) -> crate::Result<Vec<f32>>
{
    let mut length = size;
    let mut slice = [0.0; 16_384];

    match handle_unsafe!(
        crate::rp::rp_AcqGetOldestDataV(channel, &mut length, slice.as_mut_ptr())
    ) {
        Ok(_) => {
            let mut vec = slice.to_vec();
            vec.truncate(length as usize);

            Ok(vec)
        },
        Err(err) => Err(err),
    }
}

/**
 * Returns the latest ADC buffer samples in Volt units.
 */
pub fn get_latest_data_v(channel: super::Channel, size: u32) -> crate::Result<Vec<f32>>
{
    let mut length = size;
    let mut slice = [0.0; 16_384];

    match handle_unsafe!(
        crate::rp::rp_AcqGetLatestDataV(channel, &mut length, slice.as_mut_ptr())
    ) {
        Ok(_) => {
            let mut vec = slice.to_vec();
            vec.truncate(length as usize);

            Ok(vec)
        },
        Err(err) => Err(err),
    }
}

pub fn get_buffer_size() -> crate::Result<u32>
{
    let mut size = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetBufSize(&mut size)
    ) {
        Ok(_) => Ok(size),
        Err(err) => Err(err),
    }
}

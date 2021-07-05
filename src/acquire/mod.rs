pub mod trigger;

/**
 * Type representing decimation used at acquiring signal.
 */
pub use crate::rp::rp_acq_decimation_t as Decimation;

/**
 * Type representing acquire signal sampling rate.
 */
pub use crate::rp::rp_acq_sampling_rate_t as SamplingRate;

pub enum Gain {
    LV,
    HV,
}

impl std::convert::From<Gain> for super::pin::State {
    fn from(gain: Gain) -> Self {
        match gain {
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

impl std::convert::From<Gain> for String {
    fn from(gain: Gain) -> Self {
        match gain {
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
#[cfg(feature = "v1_00")]
pub fn arm_keep() -> crate::Result<bool>
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
#[cfg(feature = "v1_03")]
pub fn buffer_fill_state() -> crate::Result<bool>
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
pub fn decimation() -> crate::Result<Decimation>
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
#[cfg(feature = "v1_04")]
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
pub fn decimation_factor() -> crate::Result<u32>
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
pub fn sampling_rate() -> crate::Result<SamplingRate>
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
pub fn sampling_rate_hz() -> crate::Result<f32>
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
pub fn averaging() -> crate::Result<bool>
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
pub fn gain(channel: super::Channel) -> crate::Result<Gain>
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
pub fn gain_v(channel: super::Channel) -> crate::Result<f32>
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
pub fn write_pointer() -> crate::Result<u32>
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
pub fn write_pointer_at_trig() -> crate::Result<u32>
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
pub fn normalized_data_pos(pos: u32) -> u32
{
    unsafe {
        crate::rp::rp_AcqGetNormalizedDataPos(pos)
    }
}

/**
 * Returns the ADC buffer in raw units from start to end position.
 */
pub fn data_pos_raw(channel: super::Channel, start_pos: u32, end_pos: u32) -> crate::Result<Vec<i16>>
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
pub fn data_pos_v(channel: super::Channel, start_pos: u32, end_pos: u32) -> crate::Result<Vec<f32>>
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
pub fn data_raw(channel: super::Channel, pos: u32, size: u32) -> crate::Result<Vec<i16>>
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
pub fn data_raw_v2(pos: u32, size: u32) -> crate::Result<[Vec<u16>;2]>
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
pub fn oldest_data_raw(channel: super::Channel, size: u32) -> crate::Result<Vec<i16>>
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
pub fn latest_data_raw(channel: super::Channel, size: u32) -> crate::Result<Vec<i16>>
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
pub fn data_v(channel: super::Channel, pos: u32, size: u32) -> crate::Result<Vec<f32>>
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
pub fn data_v2(pos: u32, size: u32) -> crate::Result<[Vec<f32>;2]>
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
pub fn oldest_data_v(channel: super::Channel, size: u32) -> crate::Result<Vec<f32>>
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
pub fn latest_data_v(channel: super::Channel, size: u32) -> crate::Result<Vec<f32>>
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

pub fn buffer_size() -> crate::Result<u32>
{
    let mut size = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetBufSize(&mut size)
    ) {
        Ok(_) => Ok(size),
        Err(err) => Err(err),
    }
}

/**
 * Sets the current calibration values from temporary memory to the FPGA filter.
 */
#[cfg(feature = "v1_04")]
pub fn update_filter(channel: super::Channel) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqUpdateAcqFilter(channel)
    )
}

/**
 * Gets the current calibration values from temporary memory to the FPGA filter.
 */
#[cfg(feature = "v1_04")]
pub fn filter_calib_value(channel: super::Channel) -> crate::Result<(u32, u32, u32, u32)>
{
    let mut coef_aa = 0;
    let mut coef_bb = 0;
    let mut coef_kk = 0;
    let mut coef_pp = 0;

    handle_unsafe!(
        crate::rp::rp_AcqGetFilterCalibValue(channel, &mut coef_aa, &mut coef_bb, &mut coef_kk, &mut coef_pp)
    )?;

    Ok((coef_aa, coef_bb, coef_kk, coef_pp))
}

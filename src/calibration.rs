/**
 * Calibration parameters, stored in the EEPROM device
 */
pub use crate::rp::rp_calib_params_t as Params;

/**
 * The type represents the names of the coefficients in the filter.
 */
#[cfg(feature = "v1_04")]
pub use crate::rp::rp_eq_filter_cof_t as CoefficientFilter;

/**
 * Returns calibration settings.
 *
 * These calibration settings are populated only once from EEPROM at `init()`.
 *
 * Each `settings()` call returns the same cached setting values.
 */
#[must_use]
pub fn settings() -> Params {
    unsafe { crate::rp::rp_GetCalibrationSettings() }
}

/**
 * Calibrates input channel offset. This input channel must be grounded to
 * calibrate properly.
 *
 * Calibration data is written to EPROM and repopulated so that `settings()`
 * works properly.
 */
pub fn calibrate_front_end_offset(
    channel: super::Channel,
    gain: super::pin::State,
) -> crate::Result<Params> {
    let mut params = Params::default();

    handle_unsafe!(crate::rp::rp_CalibrateFrontEndOffset(
        channel,
        gain,
        &mut params
    ))
    .map(|_| params)
}

/**
 * Calibrates input channel low voltage scale. Jumpers must be set to LV.
 *
 * This input channel must be connected to stable positive source.
 *
 * Calibration data is written to EPROM and repopulated so that `settings()`
 * works properly.
 */
pub fn calibrate_front_end_scale_lv(
    channel: super::Channel,
    referential_voltage: f32,
) -> crate::Result<Params> {
    let mut params = Params::default();

    handle_unsafe!(crate::rp::rp_CalibrateFrontEndScaleLV(
        channel,
        referential_voltage,
        &mut params
    ))
    .map(|_| params)
}

/**
 * Calibrates input channel high voltage scale. Jumpers must be set to HV.
 *
 * This input channel must be connected to stable positive source.
 *
 * Calibration data is written to EPROM and repopulated so that `settings()`
 * works properly.
 */
pub fn calibrate_front_end_scale_hv(
    channel: super::Channel,
    referential_voltage: f32,
) -> crate::Result<Params> {
    let mut params = Params::default();

    handle_unsafe!(crate::rp::rp_CalibrateFrontEndScaleHV(
        channel,
        referential_voltage,
        &mut params
    ))
    .map(|_| params)
}

/**
 * Calibrates output channel offset.
 *
 * This input channel must be connected to calibrated input channel with came
 * number (CH1 to CH1 and CH2 to CH2).
 *
 * Calibration data is written to EPROM and repopulated so that `settings()`
 * works properly.
 */
pub fn calibrate_back_end_offset(channel: super::Channel) -> crate::Result {
    handle_unsafe!(crate::rp::rp_CalibrateBackEndOffset(channel))
}

/**
 * Calibrates output channel voltage scale.
 *
 * This input channel must be connected to calibrated input channel with came
 * number (CH1 to CH1 and CH2 to CH2).
 *
 * Calibration data is written to EPROM and repopulated so that `settings()`
 * works properly.
 */
pub fn calibrate_back_end_scale(channel: super::Channel) -> crate::Result {
    handle_unsafe!(crate::rp::rp_CalibrateBackEndScale(channel))
}

/**
 * Calibrates output channel.
 *
 * This input channel must be connected to calibrated input channel with came
 * number (CH1 to CH1 and CH2 to CH2).
 *
 * Calibration data is written to EPROM and repopulated so that `settings()`
 * works properly.
 */
pub fn calibrate_back_end(channel: super::Channel) -> crate::Result<Params> {
    let mut params = Params::default();

    handle_unsafe!(crate::rp::rp_CalibrateBackEnd(channel, &mut params))?;

    Ok(params)
}

/**
 * Set default calibration values.
 *
 * Calibration data is written to EPROM and repopulated so that `settings()`
 * works properly.
 */
pub fn reset() -> crate::Result {
    handle_unsafe!(crate::rp::rp_CalibrationReset())
}

/**
 * Set saved calibration values in case of roll-back calibration.
 *
 * Calibration data is written to EPROM and repopulated so that `settings()`
 * works properly.
 */
pub fn set_cached_params() -> crate::Result {
    handle_unsafe!(crate::rp::rp_CalibrationSetCachedParams())
}

/**
 * Write calibration values.
 *
 * Calibration data is written to EPROM and repopulated so that `settings()`
 * works properly.
 */
pub fn write_params(params: Params) -> crate::Result {
    handle_unsafe!(crate::rp::rp_CalibrationWriteParams(params))
}

/**
 * Set calibration values in memory.
 */
#[cfg(feature = "v1_04")]
pub fn set_params(params: Params) -> crate::Result {
    handle_unsafe!(crate::rp::rp_CalibrationSetParams(params))
}

/**
 * Copy factory calibration values into user eeprom.
 */
#[cfg(feature = "v1_04")]
pub fn factory_reset() -> crate::Result {
    handle_unsafe!(crate::rp::rp_CalibrationFactoryReset())
}

/**
 * Calibration parameters, stored in the EEPROM device
 */
pub use rp_sys::rp_calib_params_t as Params;

/**
 * Returns calibration settings.
 *
 * These calibration settings are populated only once from EEPROM at `init()`.
 *
 * Each `get_settings()` call returns the same cached setting values.
 */
pub fn get_settings() -> Params
{
    unsafe {
        rp_sys::rp_GetCalibrationSettings()
    }
}

/**
 * Calibrates input channel offset. This input channel must be grounded to
 * calibrate properly.
 *
 * Calibration data is written to EPROM and repopulated so that `get_settings()`
 * works properly.
 */
pub fn calibrate_front_end_offset(channel: super::Channel, gain: super::pin::State) -> Result<Params, String>
{
    let mut params = Default::default();

    match handle_unsafe!(
        rp_sys::rp_CalibrateFrontEndOffset(channel, gain, &mut params)
    ) {
        Ok(_) => Ok(params),
        Err(err) => Err(err),
    }
}

/**
 * Calibrates input channel low voltage scale. Jumpers must be set to LV.
 *
 * This input channel must be connected to stable positive source.
 *
 * Calibration data is written to EPROM and repopulated so that `get_settings()`
 * works properly.
 */
pub fn calibrate_front_end_scale_lv(channel: super::Channel, referential_voltage: f32) -> Result<Params, String>
{
    let mut params = Default::default();

    match handle_unsafe!(
        rp_sys::rp_CalibrateFrontEndScaleLV(channel, referential_voltage, &mut params)
    ) {
        Ok(_) => Ok(params),
        Err(err) => Err(err),
    }
}

/**
 * Calibrates input channel high voltage scale. Jumpers must be set to HV.
 *
 * This input channel must be connected to stable positive source.
 *
 * Calibration data is written to EPROM and repopulated so that `get_settings()`
 * works properly.
 */
pub fn calibrate_front_end_scale_hv(channel: super::Channel, referential_voltage: f32) -> Result<Params, String>
{
    let mut params = Default::default();

    match handle_unsafe!(
        rp_sys::rp_CalibrateFrontEndScaleHV(channel, referential_voltage, &mut params)
    ) {
        Ok(_) => Ok(params),
        Err(err) => Err(err),
    }
}

/**
 * Calibrates output channel offset.
 *
 * This input channel must be connected to calibrated input channel with came
 * number (CH1 to CH1 and CH2 to CH2).
 *
 * Calibration data is written to EPROM and repopulated so that `get_settings()`
 * works properly.
 */
pub fn calibrate_back_end_offset(channel: super::Channel) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrateBackEndOffset(channel)
    )
}

/**
 * Calibrates output channel voltage scale.
 *
 * This input channel must be connected to calibrated input channel with came
 * number (CH1 to CH1 and CH2 to CH2).
 *
 * Calibration data is written to EPROM and repopulated so that `get_settings()`
 * works properly.
 */
pub fn calibrate_back_end_scale(channel: super::Channel) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrateBackEndScale(channel)
    )
}

/**
 * Calibrates output channel.
 *
 * This input channel must be connected to calibrated input channel with came
 * number (CH1 to CH1 and CH2 to CH2).
 *
 * Calibration data is written to EPROM and repopulated so that `get_settings()`
 * works properly.
 */
pub fn calibrate_back_end(channel: super::Channel, params: *mut Params) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrateBackEnd(channel, params)
    )
}

/**
 * Set default calibration values.
 *
 * Calibration data is written to EPROM and repopulated so that `get_settings()`
 * works properly.
 */
pub fn reset() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrationReset()
    )
}

/**
 * Set saved calibration values in case of roll-back calibration.
 *
 * Calibration data is written to EPROM and repopulated so that `get_settings()`
 * works properly.
 */
pub fn set_cached_params() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrationSetCachedParams()
    )
}

/**
 * Write calibration values.
 *
 * Calibration data is written to EPROM and repopulated so that `get_settings()`
 * works properly.
 */
pub fn write_params(params: Params) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrationWriteParams(params)
    )
}

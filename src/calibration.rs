#[allow(unused_must_use)]

extern crate rp_sys;

pub use rp_sys::rp_calib_params_t as Params;

pub fn get_settings() -> Params
{
    unsafe {
        rp_sys::rp_GetCalibrationSettings()
    }
}

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

pub fn calibrate_back_end_offset(channel: super::Channel) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrateBackEndOffset(channel)
    )
}

pub fn calibrate_back_end_scale(channel: super::Channel) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrateBackEndScale(channel)
    )
}

pub fn calibrate_back_end(channel: super::Channel, params: *mut Params) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrateBackEnd(channel, params)
    )
}

pub fn reset() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrationReset()
    )
}

pub fn set_cached_params() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrationSetCachedParams()
    )
}

pub fn write_params(params: Params) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibrationWriteParams(params)
    )
}

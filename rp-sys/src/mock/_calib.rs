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

pub unsafe fn rp_CalibInit() -> c_int
{
    ok!()
}

pub unsafe fn rp_CalibrateBackEnd(channel: rp_channel_t, out_params: *mut rp_calib_params_t) -> c_int
{
    unsafe {
        *out_params = state!().calib;
    }

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

pub unsafe fn rp_GetDefaultCalibrationSettings() -> rp_calib_params_t {
    rp_calib_params_t {
        magic: 0xAABBCCDD,
        be_ch1_dc_offs: 0,
        be_ch2_dc_offs: 0,
        fe_ch1_lo_offs: 0,
        fe_ch2_lo_offs: 0,
        fe_ch1_hi_offs: 0,
        fe_ch2_hi_offs: 0,
        fe_ch1_fs_g_hi: 42949672,
        fe_ch2_fs_g_hi: 42949672,
        fe_ch1_fs_g_lo: 858993459,
        fe_ch2_fs_g_lo: 858993459,
        be_ch1_fs: 1,
        be_ch2_fs: 1,
    }
}

pub unsafe fn rp_CalibrationFactoryReset() -> c_int
{
    ok!()
}

pub unsafe fn rp_CalibrationSetParams(params: rp_calib_params_t) -> c_int
{
    state!().calib = params;

    ok!()
}

pub unsafe fn rp_GetCalibrationSettings() -> rp_calib_params_t
{
    state!().calib
}

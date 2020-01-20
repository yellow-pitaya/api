pub fn cnv_cnt_to_v(field_len: u32, cnts: u32, adc_max_v: f32, calib_scale: u32, calib_dc_off: i32, user_dc_off: f32) -> f32
{
    unsafe {
        crate::rp::rp_CmnCnvCntToV(field_len, cnts, adc_max_v, calib_scale, calib_dc_off, user_dc_off)
    }
}

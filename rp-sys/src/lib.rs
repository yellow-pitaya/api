#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl ::std::default::Default for rp_calib_params_t {
    fn default() -> Self {
        rp_calib_params_t {
            fe_ch1_fs_g_hi: 0,
            fe_ch2_fs_g_hi: 0,
            fe_ch1_fs_g_lo: 0,
            fe_ch2_fs_g_lo: 0,
            fe_ch1_lo_offs: 0,
            fe_ch2_lo_offs: 0,
            be_ch1_fs: 0,
            be_ch2_fs: 0,
            be_ch1_dc_offs: 0,
            be_ch2_dc_offs: 0,
            magic: 0,
            fe_ch1_hi_offs: 0,
            fe_ch2_hi_offs: 0,
        }
    }
}

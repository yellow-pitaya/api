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

impl ::std::convert::From<String> for rp_dpin_t {
    fn from(s: String) -> Self {
        match s.as_str() {
            "LED0" => rp_dpin_t::RP_LED0,
            "LED1" => rp_dpin_t::RP_LED1,
            "LED2" => rp_dpin_t::RP_LED2,
            "LED3" => rp_dpin_t::RP_LED3,
            "LED4" => rp_dpin_t::RP_LED4,
            "LED5" => rp_dpin_t::RP_LED5,
            "LED6" => rp_dpin_t::RP_LED6,
            "LED7" => rp_dpin_t::RP_LED7,
            "DIO0_P" => rp_dpin_t::RP_DIO0_P,
            "DIO1_P" => rp_dpin_t::RP_DIO1_P,
            "DIO2_P" => rp_dpin_t::RP_DIO2_P,
            "DIO3_P" => rp_dpin_t::RP_DIO3_P,
            "DIO4_P" => rp_dpin_t::RP_DIO4_P,
            "DIO5_P" => rp_dpin_t::RP_DIO5_P,
            "DIO6_P" => rp_dpin_t::RP_DIO6_P,
            "DIO7_P" => rp_dpin_t::RP_DIO7_P,
            "DIO0_N" => rp_dpin_t::RP_DIO0_N,
            "DIO1_N" => rp_dpin_t::RP_DIO1_N,
            "DIO2_N" => rp_dpin_t::RP_DIO2_N,
            "DIO3_N" => rp_dpin_t::RP_DIO3_N,
            "DIO4_N" => rp_dpin_t::RP_DIO4_N,
            "DIO5_N" => rp_dpin_t::RP_DIO5_N,
            "DIO6_N" => rp_dpin_t::RP_DIO6_N,
            "DIO7_N" => rp_dpin_t::RP_DIO7_N,
            _ => unimplemented!(),
        }
    }
}

impl ::std::convert::From<u8> for rp_pinState_t {
    fn from(state: u8) -> Self {
        match state {
            0 => rp_pinState_t::RP_LOW,
            1 => rp_pinState_t::RP_HIGH,
            _ => unimplemented!(),
        }
    }
}

impl ::std::convert::Into<u8> for rp_pinState_t {
    fn into(self) -> u8 {
        match self {
            rp_pinState_t::RP_LOW => 0,
            rp_pinState_t::RP_HIGH => 1,
        }
    }
}

impl ::std::convert::From<String> for rp_pinDirection_t {
    fn from(direction: String) -> Self {
        match direction.as_str() {
            "IN" => rp_pinDirection_t::RP_IN,
            "OUT" => rp_pinDirection_t::RP_OUT,
            _ => unimplemented!(),
        }
    }
}

impl ::std::convert::Into<String> for rp_pinDirection_t {
    fn into(self) -> String {
        match self {
            rp_pinDirection_t::RP_IN => "IN".to_owned(),
            rp_pinDirection_t::RP_OUT => "OUT".to_owned(),
        }
    }
}

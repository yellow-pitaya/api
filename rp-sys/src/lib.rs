#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl std::default::Default for rp_calib_params_t {
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

impl std::convert::From<String> for rp_dpin_t {
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

impl std::convert::From<u8> for rp_pinState_t {
    fn from(state: u8) -> Self {
        match state {
            0 => rp_pinState_t::RP_LOW,
            1 => rp_pinState_t::RP_HIGH,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<u8> for rp_pinState_t {
    fn into(self) -> u8 {
        match self {
            rp_pinState_t::RP_LOW => 0,
            rp_pinState_t::RP_HIGH => 1,
        }
    }
}

impl std::convert::From<String> for rp_pinDirection_t {
    fn from(direction: String) -> Self {
        match direction.as_str() {
            "IN" => rp_pinDirection_t::RP_IN,
            "OUT" => rp_pinDirection_t::RP_OUT,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<String> for rp_pinDirection_t {
    fn into(self) -> String {
        match self {
            rp_pinDirection_t::RP_IN => "IN",
            rp_pinDirection_t::RP_OUT => "OUT",
        }.to_owned()
    }
}

impl std::convert::From<String> for rp_apin_t {
    fn from(pin: String) -> Self {
        match pin.as_str() {
            "AOUT0" => rp_apin_t::RP_AOUT0,
            "AOUT1" => rp_apin_t::RP_AOUT1,
            "AOUT2" => rp_apin_t::RP_AOUT2,
            "AOUT3" => rp_apin_t::RP_AOUT3,
            "AIN0" => rp_apin_t::RP_AIN0,
            "AIN1" => rp_apin_t::RP_AIN1,
            "AIN2" => rp_apin_t::RP_AIN2,
            "AIN3" => rp_apin_t::RP_AIN3,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<String> for rp_apin_t {
    fn into(self) -> String {
        match self {
            rp_apin_t::RP_AOUT0 => "AOUT0",
            rp_apin_t::RP_AOUT1 => "AOUT1",
            rp_apin_t::RP_AOUT2 => "AOUT2",
            rp_apin_t::RP_AOUT3 => "AOUT3",
            rp_apin_t::RP_AIN0 => "AIN0",
            rp_apin_t::RP_AIN1 => "AIN1",
            rp_apin_t::RP_AIN2 => "AIN2",
            rp_apin_t::RP_AIN3 => "AIN3",
        }.to_owned()
    }
}

impl std::convert::From<u32> for rp_acq_decimation_t {
    fn from(decimation: u32) -> Self {
        match decimation {
            1 => rp_acq_decimation_t::RP_DEC_1,
            8 => rp_acq_decimation_t::RP_DEC_8,
            64 => rp_acq_decimation_t::RP_DEC_64,
            1024 => rp_acq_decimation_t::RP_DEC_1024,
            8192 => rp_acq_decimation_t::RP_DEC_8192,
            65536 => rp_acq_decimation_t::RP_DEC_65536,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<u32> for rp_acq_decimation_t {
    fn into(self) -> u32 {
        match self {
            rp_acq_decimation_t::RP_DEC_1 => 1,
            rp_acq_decimation_t::RP_DEC_8 => 8,
            rp_acq_decimation_t::RP_DEC_64 => 64,
            rp_acq_decimation_t::RP_DEC_1024 => 1024,
            rp_acq_decimation_t::RP_DEC_8192 => 8192,
            rp_acq_decimation_t::RP_DEC_65536 => 65536,
        }
    }
}

impl std::convert::From<String> for rp_acq_sampling_rate_t {
    fn from(rate: String) -> Self {
        match rate.as_str() {
            "125000000 Hz" => rp_acq_sampling_rate_t::RP_SMP_125M,
            "15600000 Hz" => rp_acq_sampling_rate_t::RP_SMP_15_625M,
            "1900000 Hz" => rp_acq_sampling_rate_t::RP_SMP_1_953M,
            "103800 Hz" => rp_acq_sampling_rate_t::RP_SMP_122_070K,
            "15200 Hz" => rp_acq_sampling_rate_t::RP_SMP_15_258K,
            "1900 Hz" => rp_acq_sampling_rate_t::RP_SMP_1_907K,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<String> for rp_acq_sampling_rate_t {
    fn into(self) -> String {
        match self {
            rp_acq_sampling_rate_t::RP_SMP_125M => "125MHz",
            rp_acq_sampling_rate_t::RP_SMP_15_625M => "15_6MHz",
            rp_acq_sampling_rate_t::RP_SMP_1_953M => "1_9MHz",
            rp_acq_sampling_rate_t::RP_SMP_122_070K => "103_8kHz",
            rp_acq_sampling_rate_t::RP_SMP_15_258K => "15_2kHz",
            rp_acq_sampling_rate_t::RP_SMP_1_907K => "1_9kHz",
        }.to_owned()
    }
}

impl std::convert::From<String> for rp_acq_trig_src_t {
    fn from(source: String) -> Self {
        match source.as_str() {
            "DISABLED" => rp_acq_trig_src_t::RP_TRIG_SRC_DISABLED,
            "NOW" => rp_acq_trig_src_t::RP_TRIG_SRC_NOW,
            "CH1_PE" => rp_acq_trig_src_t::RP_TRIG_SRC_CHA_PE,
            "CH1_NE" => rp_acq_trig_src_t::RP_TRIG_SRC_CHA_NE,
            "CH2_PE" => rp_acq_trig_src_t::RP_TRIG_SRC_CHB_PE,
            "CH2_NE" => rp_acq_trig_src_t::RP_TRIG_SRC_CHB_NE,
            "EXT_PE" => rp_acq_trig_src_t::RP_TRIG_SRC_EXT_PE,
            "EXT_NE" => rp_acq_trig_src_t::RP_TRIG_SRC_EXT_NE,
            "AWG_PE" => rp_acq_trig_src_t::RP_TRIG_SRC_AWG_PE,
            "AWG_NE" => rp_acq_trig_src_t::RP_TRIG_SRC_AWG_NE,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<String> for rp_acq_trig_src_t {
    fn into(self) -> String {
        match self {
            rp_acq_trig_src_t::RP_TRIG_SRC_DISABLED => "DISABLED",
            rp_acq_trig_src_t::RP_TRIG_SRC_NOW => "NOW",
            rp_acq_trig_src_t::RP_TRIG_SRC_CHA_PE => "CH1_PE",
            rp_acq_trig_src_t::RP_TRIG_SRC_CHA_NE => "CH1_NE",
            rp_acq_trig_src_t::RP_TRIG_SRC_CHB_PE => "CH2_PE",
            rp_acq_trig_src_t::RP_TRIG_SRC_CHB_NE => "CH2_NE",
            rp_acq_trig_src_t::RP_TRIG_SRC_EXT_PE => "EXT_PE",
            rp_acq_trig_src_t::RP_TRIG_SRC_EXT_NE => "EXT_NE",
            rp_acq_trig_src_t::RP_TRIG_SRC_AWG_PE => "AWG_PE",
            rp_acq_trig_src_t::RP_TRIG_SRC_AWG_NE => "AWG_NE",
        }.to_owned()
    }
}

impl std::convert::From<String> for rp_channel_t {
    fn from(channel: String) -> Self {
        match channel.as_str() {
            "SOUR1" => rp_channel_t::RP_CH_1,
            "SOUR2" => rp_channel_t::RP_CH_2,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::From<usize> for rp_channel_t {
    fn from(channel: usize) -> Self {
        match channel {
            1 => rp_channel_t::RP_CH_1,
            2 => rp_channel_t::RP_CH_2,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<String> for rp_channel_t {
    fn into(self) -> String {
        match self {
            rp_channel_t::RP_CH_1 => "SOUR1",
            rp_channel_t::RP_CH_2 => "SOUR2",
        }.to_owned()
    }
}

impl std::convert::Into<String> for rp_waveform_t {
    fn into(self) -> String {
        match self {
            rp_waveform_t::RP_WAVEFORM_SINE => "SINE",
            rp_waveform_t::RP_WAVEFORM_SQUARE => "SQUARE",
            rp_waveform_t::RP_WAVEFORM_TRIANGLE => "TRIANGLE",
            rp_waveform_t::RP_WAVEFORM_RAMP_UP => "SAWU",
            rp_waveform_t::RP_WAVEFORM_RAMP_DOWN => "SAWD",
            rp_waveform_t::RP_WAVEFORM_DC => "DC",
            rp_waveform_t::RP_WAVEFORM_PWM => "PWM",
            rp_waveform_t::RP_WAVEFORM_ARBITRARY => "ARBITRARY",
        }.to_owned()
    }
}


impl std::convert::From<String> for rp_waveform_t {
    fn from(channel: String) -> Self {
        match channel.as_str() {
            "SINE" => rp_waveform_t::RP_WAVEFORM_SINE,
            "SQUARE" => rp_waveform_t::RP_WAVEFORM_SQUARE,
            "TRIANGLE" => rp_waveform_t::RP_WAVEFORM_TRIANGLE,
            "SAWU" => rp_waveform_t::RP_WAVEFORM_RAMP_UP,
            "SAWD" => rp_waveform_t::RP_WAVEFORM_RAMP_DOWN,
            "DC" => rp_waveform_t::RP_WAVEFORM_DC,
            "PWM" => rp_waveform_t::RP_WAVEFORM_PWM,
            "ARBITRARY" => rp_waveform_t::RP_WAVEFORM_ARBITRARY,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::From<String> for rp_gen_mode_t {
    fn from(channel: String) -> Self {
        match channel.as_str() {
            "CONTINUOUS" => rp_gen_mode_t::RP_GEN_MODE_CONTINUOUS,
            "BURST" => rp_gen_mode_t::RP_GEN_MODE_BURST,
            "STREAM" => rp_gen_mode_t::RP_GEN_MODE_STREAM,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<String> for rp_gen_mode_t {
    fn into(self) -> String {
        match self {
            rp_gen_mode_t::RP_GEN_MODE_CONTINUOUS => "CONTINUOUS",
            rp_gen_mode_t::RP_GEN_MODE_BURST => "BURST",
            rp_gen_mode_t::RP_GEN_MODE_STREAM => "STREAM",
        }.to_owned()
    }
}

impl std::convert::From<String> for rp_trig_src_t {
    fn from(channel: String) -> Self {
        match channel.as_str() {
            "INT" => rp_trig_src_t::RP_GEN_TRIG_SRC_INTERNAL,
            "EXT_PE" => rp_trig_src_t::RP_GEN_TRIG_SRC_EXT_PE,
            "EXT_NE" => rp_trig_src_t::RP_GEN_TRIG_SRC_EXT_NE,
            "BURST" => rp_trig_src_t::RP_GEN_TRIG_GATED_BURST,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<String> for rp_trig_src_t {
    fn into(self) -> String {
        match self {
            rp_trig_src_t::RP_GEN_TRIG_SRC_INTERNAL => "INT",
            rp_trig_src_t::RP_GEN_TRIG_SRC_EXT_PE => "EXT_PE",
            rp_trig_src_t::RP_GEN_TRIG_SRC_EXT_NE => "EXT_NE",
            rp_trig_src_t::RP_GEN_TRIG_GATED_BURST => "BURST",
        }.to_owned()
    }
}

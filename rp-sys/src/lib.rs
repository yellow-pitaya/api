#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::missing_safety_doc)]
#![warn(warnings)]

#[cfg(not(feature = "mock"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "mock")]
include!("mock/mod.rs");

#[cfg(not(feature = "v1_04"))]
impl std::default::Default for rp_calib_params_t {
    fn default() -> Self {
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
}

#[cfg(feature = "v1_04")]
impl std::default::Default for rp_calib_params_t {
    fn default() -> Self {
        unsafe { crate::rp_GetDefaultCalibrationSettings() }
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

impl std::convert::From<rp_pinState_t> for u8 {
    fn from(state: rp_pinState_t) -> Self {
        match state {
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

impl std::convert::From<rp_pinDirection_t> for String {
    fn from(direction: rp_pinDirection_t) -> Self {
        match direction {
            rp_pinDirection_t::RP_IN => "IN",
            rp_pinDirection_t::RP_OUT => "OUT",
        }
        .to_owned()
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

impl std::convert::From<rp_apin_t> for String {
    fn from(apin: rp_apin_t) -> Self {
        match apin {
            rp_apin_t::RP_AOUT0 => "AOUT0",
            rp_apin_t::RP_AOUT1 => "AOUT1",
            rp_apin_t::RP_AOUT2 => "AOUT2",
            rp_apin_t::RP_AOUT3 => "AOUT3",
            rp_apin_t::RP_AIN0 => "AIN0",
            rp_apin_t::RP_AIN1 => "AIN1",
            rp_apin_t::RP_AIN2 => "AIN2",
            rp_apin_t::RP_AIN3 => "AIN3",
        }
        .to_owned()
    }
}

impl std::convert::From<u32> for rp_acq_decimation_t {
    fn from(decimation: u32) -> Self {
        match decimation {
            1 => rp_acq_decimation_t::RP_DEC_1,
            #[cfg(feature = "v1_04")]
            2 => rp_acq_decimation_t::RP_DEC_2,
            8 => rp_acq_decimation_t::RP_DEC_8,
            #[cfg(feature = "v1_04")]
            16 => rp_acq_decimation_t::RP_DEC_16,
            #[cfg(feature = "v1_04")]
            32 => rp_acq_decimation_t::RP_DEC_32,
            64 => rp_acq_decimation_t::RP_DEC_64,
            #[cfg(feature = "v1_04")]
            128 => rp_acq_decimation_t::RP_DEC_128,
            #[cfg(feature = "v1_04")]
            256 => rp_acq_decimation_t::RP_DEC_256,
            #[cfg(feature = "v1_04")]
            512 => rp_acq_decimation_t::RP_DEC_512,
            1024 => rp_acq_decimation_t::RP_DEC_1024,
            #[cfg(feature = "v1_04")]
            2048 => rp_acq_decimation_t::RP_DEC_2048,
            #[cfg(feature = "v1_04")]
            #[cfg(feature = "v1_04")]
            4096 => rp_acq_decimation_t::RP_DEC_4096,
            8192 => rp_acq_decimation_t::RP_DEC_8192,
            #[cfg(feature = "v1_04")]
            16384 => rp_acq_decimation_t::RP_DEC_16384,
            #[cfg(feature = "v1_04")]
            32768 => rp_acq_decimation_t::RP_DEC_32768,
            65536 => rp_acq_decimation_t::RP_DEC_65536,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::From<rp_acq_decimation_t> for u32 {
    fn from(decimation: rp_acq_decimation_t) -> Self {
        match decimation {
            rp_acq_decimation_t::RP_DEC_1 => 1,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_2 => 2,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_4 => 4,
            rp_acq_decimation_t::RP_DEC_8 => 8,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_16 => 16,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_32 => 32,
            rp_acq_decimation_t::RP_DEC_64 => 64,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_128 => 128,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_256 => 256,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_512 => 512,
            rp_acq_decimation_t::RP_DEC_1024 => 1024,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_2048 => 2048,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_4096 => 4096,
            rp_acq_decimation_t::RP_DEC_8192 => 8192,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_16384 => 16384,
            #[cfg(feature = "v1_04")]
            rp_acq_decimation_t::RP_DEC_32768 => 32768,
            rp_acq_decimation_t::RP_DEC_65536 => 65536,
        }
    }
}

impl std::convert::From<rp_acq_decimation_t> for rp_acq_sampling_rate_t {
    fn from(decimation: rp_acq_decimation_t) -> Self {
        use rp_acq_decimation_t::*;

        match decimation {
            RP_DEC_1 => Self::RP_SMP_125M,
            #[cfg(feature = "v1_04")]
            RP_DEC_2 => Self::RP_SMP_62_500M,
            #[cfg(feature = "v1_04")]
            RP_DEC_4 => Self::RP_SMP_31_250M,
            RP_DEC_8 => Self::RP_SMP_15_625M,
            #[cfg(feature = "v1_04")]
            RP_DEC_16 => Self::RP_SMP_7_812M,
            #[cfg(feature = "v1_04")]
            RP_DEC_32 => Self::RP_SMP_3_906M,
            RP_DEC_64 => Self::RP_SMP_1_953M,
            #[cfg(feature = "v1_04")]
            RP_DEC_128 => Self::RP_SMP_976_562K,
            #[cfg(feature = "v1_04")]
            RP_DEC_256 => Self::RP_SMP_448_281K,
            #[cfg(feature = "v1_04")]
            RP_DEC_512 => Self::RP_SMP_244_140K,
            RP_DEC_1024 => Self::RP_SMP_122_070K,
            #[cfg(feature = "v1_04")]
            RP_DEC_2048 => Self::RP_SMP_61_035K,
            #[cfg(feature = "v1_04")]
            RP_DEC_4096 => Self::RP_SMP_30_517K,
            RP_DEC_8192 => Self::RP_SMP_15_258K,
            #[cfg(feature = "v1_04")]
            RP_DEC_16384 => Self::RP_SMP_7_629K,
            #[cfg(feature = "v1_04")]
            RP_DEC_32768 => Self::RP_SMP_3_814K,
            RP_DEC_65536 => Self::RP_SMP_1_907K,
        }
    }
}

impl std::convert::From<rp_acq_sampling_rate_t> for rp_acq_decimation_t {
    fn from(sampling_rate: rp_acq_sampling_rate_t) -> Self {
        use rp_acq_sampling_rate_t::*;

        match sampling_rate {
            RP_SMP_125M => Self::RP_DEC_1,
            #[cfg(feature = "v1_04")]
            RP_SMP_62_500M => Self::RP_DEC_2,
            #[cfg(feature = "v1_04")]
            RP_SMP_31_250M => Self::RP_DEC_4,
            RP_SMP_15_625M => Self::RP_DEC_8,
            #[cfg(feature = "v1_04")]
            RP_SMP_7_812M => Self::RP_DEC_16,
            #[cfg(feature = "v1_04")]
            RP_SMP_3_906M => Self::RP_DEC_32,
            RP_SMP_1_953M => Self::RP_DEC_64,
            #[cfg(feature = "v1_04")]
            RP_SMP_976_562K => Self::RP_DEC_128,
            #[cfg(feature = "v1_04")]
            RP_SMP_448_281K => Self::RP_DEC_256,
            #[cfg(feature = "v1_04")]
            RP_SMP_244_140K => Self::RP_DEC_512,
            RP_SMP_122_070K => Self::RP_DEC_1024,
            #[cfg(feature = "v1_04")]
            RP_SMP_61_035K => Self::RP_DEC_2048,
            #[cfg(feature = "v1_04")]
            RP_SMP_30_517K => Self::RP_DEC_4096,
            RP_SMP_15_258K => Self::RP_DEC_8192,
            #[cfg(feature = "v1_04")]
            RP_SMP_7_629K => Self::RP_DEC_16384,
            #[cfg(feature = "v1_04")]
            RP_SMP_3_814K => Self::RP_DEC_32768,
            RP_SMP_1_907K => Self::RP_DEC_65536,
        }
    }
}

impl std::convert::From<String> for rp_acq_sampling_rate_t {
    fn from(rate: String) -> Self {
        use rp_acq_sampling_rate_t::*;

        match rate.as_str() {
            "125000000 Hz" => RP_SMP_125M,
            #[cfg(feature = "v1_04")]
            "62500000 Hz" => RP_SMP_62_500M,
            #[cfg(feature = "v1_04")]
            "31250000 Hz" => RP_SMP_31_250M,
            "15600000 Hz" => RP_SMP_15_625M,
            #[cfg(feature = "v1_04")]
            "7812000 Hz" => RP_SMP_7_812M,
            #[cfg(feature = "v1_04")]
            "3906000 Hz" => RP_SMP_3_906M,
            "1900000 Hz" => RP_SMP_1_953M,
            #[cfg(feature = "v1_04")]
            "976562 Hz" => RP_SMP_976_562K,
            #[cfg(feature = "v1_04")]
            "448281 Hz" => RP_SMP_448_281K,
            #[cfg(feature = "v1_04")]
            "244140 Hz" => RP_SMP_244_140K,
            "103800 Hz" => RP_SMP_122_070K,
            #[cfg(feature = "v1_04")]
            "61035 Hz" => RP_SMP_61_035K,
            #[cfg(feature = "v1_04")]
            "30517 Hz" => RP_SMP_30_517K,
            "15200 Hz" => RP_SMP_15_258K,
            #[cfg(feature = "v1_04")]
            "7629 Hz" => RP_SMP_7_629K,
            #[cfg(feature = "v1_04")]
            "3814 Hz" => RP_SMP_3_814K,
            "1900 Hz" => RP_SMP_1_907K,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::From<rp_acq_sampling_rate_t> for String {
    fn from(rate: rp_acq_sampling_rate_t) -> Self {
        use rp_acq_sampling_rate_t::*;

        match rate {
            RP_SMP_125M => "125MHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_62_500M => "62_500MHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_31_250M => "31_250MHz",
            RP_SMP_15_625M => "15_6MHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_7_812M => "7_812MHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_3_906M => "3_906MHz",
            RP_SMP_1_953M => "1_9MHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_976_562K => "976_562kHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_448_281K => "448_281kHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_244_140K => "244_140kHz",
            RP_SMP_122_070K => "103_8kHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_61_035K => "61_035kHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_30_517K => "30_517kHz",
            RP_SMP_15_258K => "15_2kHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_7_629K => "7_629kHz",
            #[cfg(feature = "v1_04")]
            RP_SMP_3_814K => "3_814kHz",
            RP_SMP_1_907K => "1_9kHz",
        }
        .to_owned()
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

impl std::convert::From<rp_acq_trig_src_t> for String {
    fn from(source: rp_acq_trig_src_t) -> Self {
        match source {
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
        }
        .to_owned()
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

impl std::convert::From<rp_channel_t> for String {
    fn from(channel: rp_channel_t) -> Self {
        match channel {
            rp_channel_t::RP_CH_1 => "SOUR1",
            rp_channel_t::RP_CH_2 => "SOUR2",
        }
        .to_owned()
    }
}

impl std::convert::From<rp_waveform_t> for String {
    fn from(waveform: rp_waveform_t) -> Self {
        match waveform {
            rp_waveform_t::RP_WAVEFORM_SINE => "SINE",
            rp_waveform_t::RP_WAVEFORM_SQUARE => "SQUARE",
            rp_waveform_t::RP_WAVEFORM_TRIANGLE => "TRIANGLE",
            rp_waveform_t::RP_WAVEFORM_RAMP_UP => "SAWU",
            rp_waveform_t::RP_WAVEFORM_RAMP_DOWN => "SAWD",
            rp_waveform_t::RP_WAVEFORM_DC => "DC",
            rp_waveform_t::RP_WAVEFORM_PWM => "PWM",
            rp_waveform_t::RP_WAVEFORM_ARBITRARY => "ARBITRARY",
            #[cfg(feature = "v1_04")]
            rp_waveform_t::RP_WAVEFORM_DC_NEG => "NEG",
            #[cfg(feature = "v1_04")]
            rp_waveform_t::RP_WAVEFORM_SWEEP => "SWEEP",
        }
        .to_owned()
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
            #[cfg(feature = "v1_04")]
            "NEG" => rp_waveform_t::RP_WAVEFORM_DC_NEG,
            #[cfg(feature = "v1_04")]
            "SWEEP" => rp_waveform_t::RP_WAVEFORM_SWEEP,
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

impl std::convert::From<rp_gen_mode_t> for String {
    fn from(mode: rp_gen_mode_t) -> Self {
        match mode {
            rp_gen_mode_t::RP_GEN_MODE_CONTINUOUS => "CONTINUOUS",
            rp_gen_mode_t::RP_GEN_MODE_BURST => "BURST",
            rp_gen_mode_t::RP_GEN_MODE_STREAM => "STREAM",
        }
        .to_owned()
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

impl std::convert::From<rp_trig_src_t> for String {
    fn from(source: rp_trig_src_t) -> Self {
        match source {
            rp_trig_src_t::RP_GEN_TRIG_SRC_INTERNAL => "INT",
            rp_trig_src_t::RP_GEN_TRIG_SRC_EXT_PE => "EXT_PE",
            rp_trig_src_t::RP_GEN_TRIG_SRC_EXT_NE => "EXT_NE",
            rp_trig_src_t::RP_GEN_TRIG_GATED_BURST => "BURST",
        }
        .to_owned()
    }
}

#![warn(warnings)]

use rp_sys as rp;

/**
 * Type representing Input/Output channels.
 */
pub use rp::rp_channel_t as Channel;

macro_rules! handle_unsafe {
    ($e:expr) => {{
        match unsafe { $e } as u32 {
            $crate::rp::RP_OK => Ok(()),
            code => Err(crate::Error::Redpitaya(code)),
        }
    }};
}

macro_rules! convert_string {
    ($e:expr) => {{
        let buffer = unsafe { std::ffi::CStr::from_ptr($e) };

        std::str::from_utf8(buffer.to_bytes()).unwrap().to_owned()
    }};
}

pub mod acquire;
pub mod calibration;
pub mod cmn;
#[cfg(feature = "dma")]
pub mod dma;
pub mod fpga;
pub mod generator;
pub mod gpio;
#[cfg(feature = "la")]
pub mod la;
pub mod led;
pub mod pin;
#[cfg(feature = "v1_04")]
pub mod uart;

mod errors;

pub use errors::*;

/**
 * Initializes the library.
 *
 * It must be called first, before any other library method.
 */
pub fn init() -> Result {
    handle_unsafe!(rp::rp_Init())
}

/**
 * Initializes the library.
 *
 * It must be called first, before any other library method.
 */
#[cfg(feature = "v1_00")]
pub fn init_reset(reset: bool) -> Result {
    handle_unsafe!(rp::rp_InitReset(reset))
}

pub fn calib_init() -> Result {
    handle_unsafe!(rp::rp_CalibInit())
}

/**
 * Releases the library resources.
 *
 * It must be called last, after library is not used anymore. Typically before
 * application exits.
 */
pub fn release() -> Result {
    handle_unsafe!(rp::rp_Release())
}

/**
 * Resets all modules.
 *
 * Typically calles after `init()` application exits.
 */
pub fn reset() -> Result {
    handle_unsafe!(rp::rp_Reset())
}

/**
 * Retrieves the library version number
 */
pub fn version() -> String {
    convert_string!(rp::rp_GetVersion())
}

/**
 * Enable or disables digital loop.
 *
 * This internally connect output to input.
 */
pub fn enable_digital_loop(enable: bool) -> Result {
    handle_unsafe!(rp::rp_EnableDigitalLoop(enable))
}

#[cfg(feature = "la")]
fn fpga_reg_dump(desc: &str, addr: u32, data: &[u8]) -> crate::Result {
    let mut addr = addr;

    print!("\n\r {desc}\n\r");
    print!("\n\r index, addr, value\n\r");

    for x in (0..data.len()).step_by(4) {
        print!("0x{x:04},0x{addr:08x},0x{:02x}{:02x}{:02x}{:02x}\n\r", data[x], data[x + 1], data[x + 2], data[x + 3]);
        addr += 0x4;
    }

    Ok(())
}

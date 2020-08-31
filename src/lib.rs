#![warn(rust_2018_idioms)]

use rp_sys as rp;

/**
 * Type representing Input/Output channels.
 */
pub use rp::rp_channel_t as Channel;

macro_rules! handle_unsafe {
    ($e:expr) => (
        {
            match unsafe { $e } as u32 {
                $crate::rp::RP_OK => Ok(()),
                code => Err(crate::Error::from(code)),
            }
        }
    );
}

macro_rules! convert_string {
    ($e:expr) => (
        {
            let buffer = unsafe {
                std::ffi::CStr::from_ptr($e)
            };

            std::str::from_utf8(buffer.to_bytes())
                .unwrap()
                .to_owned()
        }
    );
}

pub mod calibration;
pub mod fpga;
pub mod led;
pub mod gpio;
pub mod pin;
pub mod acquire;
pub mod generator;
pub mod cmn;

mod result;

pub use result::{Error, Result};

/**
 * Initializes the library.
 *
 * It must be called first, before any other library method.
 */
pub fn init() -> Result<()>
{
    handle_unsafe!(
        rp::rp_Init()
    )
}

/**
 * Initializes the library.
 *
 * It must be called first, before any other library method.
 */
pub fn init_reset(reset: bool) -> Result<()>
{
    handle_unsafe!(
        rp::rp_InitReset(reset)
    )
}

pub fn calib_init() -> Result<()>
{
    handle_unsafe!(
        rp::rp_CalibInit()
    )
}

/**
 * Releases the library resources.
 *
 * It must be called last, after library is not used anymore. Typically before
 * application exits.
 */
pub fn release() -> Result<()>
{
    handle_unsafe!(
        rp::rp_Release()
    )
}

/**
 * Resets all modules.
 *
 * Typically calles after `init()` application exits.
 */
pub fn reset() -> Result<()>
{
    handle_unsafe!(
        rp::rp_Reset()
    )
}

/**
 * Retrieves the library version number
 */
pub fn get_version() -> String
{
    convert_string!(
        rp::rp_GetVersion()
    )
}

/**
 * Enable or disables digital loop.
 *
 * This internally connect output to input.
 */
pub fn enable_digital_loop(enable: bool) -> Result<()>
{
    handle_unsafe!(
        rp::rp_EnableDigitalLoop(enable)
    )
}

#[allow(unused_must_use)]

extern crate rp_sys;
extern crate libc;

/**
 * Type representing Input/Output channels.
 */
pub use rp_sys::rp_channel_t as Channel;

macro_rules! handle_unsafe {
    ($e:expr) => (
        match unsafe { $e } as u32 {
            rp_sys::RP_OK => Ok(()),
            code => Err($crate::get_error(code as i32)),
        }
    );
}

macro_rules! convert_string {
    ($e:expr) => (
        {
            let cstring = unsafe {
                let buffer = $e as *mut libc::c_char;

                std::ffi::CString::from_raw(buffer)
            };

            cstring.into_string()
                .unwrap()
        }
    );
}

#[macro_use]
pub mod calibration;
#[macro_use]
pub mod fpga;
#[macro_use]
pub mod led;
#[macro_use]
pub mod gpio;
#[macro_use]
pub mod pin;
#[macro_use]
pub mod acquire;
#[macro_use]
pub mod generator;
#[macro_use]
pub mod cmn;

/**
 * Initializes the library.
 *
 * It must be called first, before any other library method.
 */
pub fn init() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_Init()
    )
}

pub fn calib_init() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_CalibInit()
    )
}

/**
 * Releases the library resources.
 *
 * It must be called last, after library is not used anymore. Typically before
 * application exits.
 */
pub fn release() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_Release()
    )
}

/**
 * Resets all modules.
 *
 * Typically calles after `init()` application exits.
 */
pub fn reset() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_Reset()
    )
}

/**
 * Retrieves the library version number
 */
pub fn get_version() -> String
{
    convert_string!(
        rp_sys::rp_GetVersion()
    )
}

/**
 * Returns textual representation of error code.
 */
fn get_error(code: i32) -> String
{
    convert_string!(
        rp_sys::rp_GetError(code)
    )
}

/**
 * Enable or disables digital loop.
 *
 * This internally connect output to input.
 */
pub fn enable_digital_loop(enable: bool) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_EnableDigitalLoop(enable)
    )
}

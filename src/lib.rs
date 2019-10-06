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

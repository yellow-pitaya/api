#[allow(unused_must_use)]

extern crate rp_sys;

macro_rules! handle_unsafe {
    ($e:expr) => (
        match unsafe { $e } {
            rp_sys::RP_OK => Ok(()),
            code => Err(get_error(code)),
        }
    );
}

macro_rules! convert_string {
    ($e:expr) => (
        {
            let cstring = unsafe {
                let buffer = $e as *mut u8;

                std::ffi::CString::from_raw(buffer)
            };

            cstring.into_string()
                .unwrap()
        }
    );
}

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

pub fn release() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_Release()
    )
}

pub fn reset() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_Reset()
    )
}

pub fn get_version() -> String
{
    convert_string!(
        rp_sys::rp_GetVersion()
    )
}

fn get_error(code: i32) -> String
{
    convert_string!(
        rp_sys::rp_GetError(code)
    )
}

pub fn enable_digital_loop(enable: bool) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_EnableDigitalLoop(enable as i32)
    )
}

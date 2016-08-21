#[allow(unused_must_use)]

extern crate rp_sys;

pub fn get_id() -> Result<u32, String>
{
    let mut id = 0;

    match handle_unsafe!(
        rp_sys::rp_IdGetID(&mut id)
    ) {
        Ok(_) => Ok(id),
        Err(err) => Err(err),
    }
}

pub fn get_dna() -> Result<u64, String>
{
    let mut dna = 0;

    match handle_unsafe!(
        rp_sys::rp_IdGetDNA(&mut dna)
    ) {
        Ok(_) => Ok(dna),
        Err(err) => Err(err),
    }
}

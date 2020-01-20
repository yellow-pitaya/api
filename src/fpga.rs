#[allow(unused_must_use)]

/**
 * Gets FPGA Synthesized ID.
 */
pub fn get_id() -> crate::Result<u32>
{
    let mut id = 0;

    match handle_unsafe!(
        crate::rp::rp_IdGetID(&mut id)
    ) {
        Ok(_) => Ok(id),
        Err(err) => Err(err),
    }
}

/**
 * Gets FPGA Unique DNA.
 */
pub fn get_dna() -> crate::Result<u64>
{
    let mut dna = 0;

    match handle_unsafe!(
        crate::rp::rp_IdGetDNA(&mut dna)
    ) {
        Ok(_) => Ok(dna),
        Err(err) => Err(err),
    }
}

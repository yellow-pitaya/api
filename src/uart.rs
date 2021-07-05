pub use crate::rp::rp_uart_bits_size_t as BitsSize;
pub use crate::rp::rp_uart_parity_t as Parity;
pub use crate::rp::rp_uart_stop_bits_t as StopBits;

/**
 * Opens the UART device (`/dev/ttyPS1`). Initializes the default settings.
 */
pub fn init() -> crate::Result<()> {
    handle_unsafe!(crate::rp::rp_UartInit())
}

/**
 * Closes device UART.
 */
pub fn release() -> crate::Result<()> {
    handle_unsafe!(crate::rp::rp_UartRelease())
}

/**
 * Reading values into the buffer from the UART device.
 */
pub fn read(size: usize) -> crate::Result<Vec<u8>> {
    let mut len = size as i32;
    let mut buffer = vec![0; size];

    handle_unsafe!(crate::rp::rp_UartRead(buffer.as_mut_ptr(), &mut len))?;

    buffer.resize(len as usize, 0);

    Ok(buffer)
}

/**
 * Writes data to UART.
 */
pub fn write(buffer: &[u8]) -> crate::Result<()> {
    let mut b = buffer.to_vec();

    handle_unsafe!(crate::rp::rp_UartWrite(b.as_mut_ptr(), buffer.len() as i32))
}

/**
 * Set speed for the UART.
 */
pub fn set_speed(speed: i32) -> crate::Result<()> {
    handle_unsafe!(crate::rp::rp_UartSpeed(speed))
}

/**
 * Set character size for the UART.
 */
pub fn set_bits(size: BitsSize) -> crate::Result<()> {
    handle_unsafe!(crate::rp::rp_UartSetBits(size))
}

/**
 * Set stop bits size for the UART.
 */
pub fn set_stop_bits(mode: StopBits) -> crate::Result<()> {
    handle_unsafe!(crate::rp::rp_UartSetStopBits(mode))
}

/**
 * Set parity check mode for the UART.
 */
pub fn set_parity(mode: Parity) -> crate::Result<()> {
    handle_unsafe!(crate::rp::rp_UartSetParityMode(mode))
}

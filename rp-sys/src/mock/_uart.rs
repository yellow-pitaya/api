pub enum rp_uart_bits_size_t {
    RP_UART_CS6,
    RP_UART_CS7,
    RP_UART_CS8,
}

pub enum rp_uart_parity_t {
    RP_UART_NONE,
    RP_UART_EVEN,
    RP_UART_ODD,
    RP_UART_MARK,
    RP_UART_SPACE,
}

pub enum rp_uart_stop_bits_t {
    RP_UART_STOP1,
    RP_UART_STOP2,
}

pub unsafe fn rp_UartInit() -> c_int
{
    ok!()
}

pub unsafe fn rp_UartRelease() -> c_int
{
    ok!()
}

pub unsafe fn rp_UartRead(buffer: *mut c_uchar, size: *mut c_int) -> c_int
{
    todo!()
}

pub unsafe fn rp_UartWrite(buffer: *mut c_uchar, size: c_int) -> c_int
{
    todo!()
}

pub unsafe fn rp_UartSetBits(size: rp_uart_bits_size_t) -> c_int
{
    ok!()
}

pub unsafe fn rp_UartSetParityMode(mode: rp_uart_parity_t) -> c_int
{
    ok!()
}

pub unsafe fn rp_UartSetStopBits(mode: rp_uart_stop_bits_t) -> c_int
{
    ok!()
}

pub unsafe fn rp_UartSpeed(speed: c_int) -> c_int
{
    ok!()
}

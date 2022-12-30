pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "dma")]
    #[error("Value out of range")]
    Eoor,
    #[cfg(feature = "dma")]
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[cfg(feature = "dma")]
    #[error("Mmap error: {0}")]
    Mmap(#[from] mmap_rs::Error),
    #[error("{}", convert_string!(crate::rp::rp_GetError(*.0 as i32)))]
    Redpitaya(u32),
    #[error("{0}")]
    String(#[from] std::str::Utf8Error),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{}", convert_string!(crate::rp::rp_GetError(*.0 as i32)))]
    Redpitaya(u32),
    #[error("{0}")]
    String(#[from] std::str::Utf8Error),
}

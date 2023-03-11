use thiserror::Error;

pub type Result<T> = std::result::Result<T, AxoSlidesError>;

#[derive(Debug, Error)]
pub enum AxoSlidesError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error(transparent)]
    AxoAsset(#[from] axoasset::AxoassetError),

    #[error("{0}")]
    Other(String),
}

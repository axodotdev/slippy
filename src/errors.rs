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

    #[error("failed to read {filedesc} at {path}")]
    FileNotFound { filedesc: String, path: String },

    #[error("File is not markdown")]
    FileNotMD {},

    #[error(transparent)]
    FSExtra(#[from] fs_extra::error::Error),

    #[error("There was an issue minifing CSS")]
    CSSMinificationError {},
    // #[error("{0}")]
    // Other(String),
}

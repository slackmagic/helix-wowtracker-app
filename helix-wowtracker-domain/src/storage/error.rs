use thiserror::Error;

//Define the possible errors
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Creation impossible")]
    CreationImpossible,
    #[error("Connection impossible")]
    ConnectionImpossible,
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("Serde Json error: {source}")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },
    #[error("Tokio error: {source}")]
    Tokio {
        #[from]
        source: tokio::task::JoinError,
    },
}

//Define a generic error type to simplify return.
pub type StorageResult<T> = std::result::Result<T, StorageError>;

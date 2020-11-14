use crate::storage::error::StorageError;
use helix_tracker_lib::business::error::TrackerDomainError;
use std::result::Result;
use thiserror::Error;

//Define the possible errors
#[derive(Error, Debug)]
pub enum WowTrackerDomainError {
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Storage Error")]
    StorageError,
    #[error("Not found error")]
    NotFoundError,
    #[error("Storage error: {source}")]
    Storage {
        #[from]
        source: StorageError,
    },
    #[error("Tracker error: {source}")]
    Tracker {
        #[from]
        source: TrackerDomainError,
    },
}

//Define a generic error type to simplify return.
pub type WowTrackerDomainResult<T> = Result<T, WowTrackerDomainError>;

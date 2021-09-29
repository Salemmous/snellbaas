use thiserror::Error;

#[derive(Error, Debug)]
pub enum SBError {
    #[error("Database connection error")]
    DBConnectionError(),
    #[error("Service error [{service:?}]: {message:?}")]
    ServiceError { message: String, service: String },
    #[error("Internal service error [{service:?}]: {message:?}")]
    InternalServiceError { message: String, service: String },
    #[error("ENV Key Missing: {key:?}")]
    EnvConfigError { key: String },
}

pub type SBResult<T> = std::result::Result<T, SBError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SBError {
    #[error("Database connection error")]
    DBConnectionError(),
    #[error("User service error: {message:?}")]
    UserServiceError { message: String },
    #[error("User internal service error: {message:?}")]
    UserInternalServiceError { message: String },
    #[error("Project service error: {message:?}")]
    ProjectServiceError { message: String },
    #[error("Project internal service error: {message:?}")]
    ProjectInternalServiceError { message: String },
    #[error("ENV Key Missing: {key:?}")]
    EnvConfigError { key: String },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

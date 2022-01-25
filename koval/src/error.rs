use thiserror::Error;

#[derive(Error, Debug)]
pub enum InjectionError {
    #[error("type is not bound in the container")]
    TypeNotBound,
    #[error("couldn't resolve type inner error: {0}")]
    ResolutionFailed(String),
}

pub type ResolutionResult<T> = Result<T, InjectionError>;
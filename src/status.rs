use crate::{Error, StatusCode};
use core::convert::{Infallible, Into};
use std::error::Error as StdError;

/// Provides the `status` method for `Result` and `Option`.
///
/// This trait is sealed and cannot be implemented outside of `http-types`.
pub trait Status<T, E>: private::Sealed {
    /// Wrap the error value with an additional status code.
    fn status<S>(self, status: S) -> Result<T, Error>
    where
        S: Into<StatusCode>;

    /// Wrap the error value with an additional status code that is evaluated
    /// lazily only once an error does occur.
    fn with_status<S, F>(self, f: F) -> Result<T, Error>
    where
        S: Into<StatusCode>,
        F: FnOnce() -> S;
}

impl<T, E> Status<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    /// Wrap the error value with an additional status code.
    ///
    /// [status]: crate::Status
    /// [statuscode]: crate::StatusCode
    fn status<S>(self, status: S) -> Result<T, Error>
    where
        S: Into<StatusCode>,
    {
        self.map_err(|error| {
            let status = status.into();
            Error::new(status, error)
        })
    }

    /// Wrap the error value with an additional status code that is evaluated
    /// lazily only once an error does occur.
    ///
    /// [status]: crate::Status
    /// [statuscode]: crate::StatusCode
    fn with_status<S, F>(self, f: F) -> Result<T, Error>
    where
        S: Into<StatusCode>,
        F: FnOnce() -> S,
    {
        self.map_err(|error| {
            let status = f().into();
            Error::new(status, error)
        })
    }
}

impl<T> Status<T, Infallible> for Option<T> {
    /// Wrap the error value with an additional status code.
    ///
    /// [status]: crate::Status
    /// [statuscode]: crate::StatusCode
    fn status<S>(self, status: S) -> Result<T, Error>
    where
        S: Into<StatusCode>,
    {
        self.ok_or_else(|| {
            let status = status.into();
            Error::from_str(status, "NoneError")
        })
    }

    /// Wrap the error value with an additional status code that is evaluated
    /// lazily only once an error does occur.
    ///
    /// [status]: crate::Status
    /// [statuscode]: crate::StatusCode
    fn with_status<S, F>(self, f: F) -> Result<T, Error>
    where
        S: Into<StatusCode>,
        F: FnOnce() -> S,
    {
        self.ok_or_else(|| {
            let status = f().into();
            Error::from_str(status, "NoneError")
        })
    }
}

pub(crate) mod private {
    pub trait Sealed {}

    impl<T, E> Sealed for Result<T, E> {}
    impl<T> Sealed for Option<T> {}
}

#[cfg(test)]
mod test {
    use super::Status;

    #[test]
    fn construct_shorthand_with_valid_status_code() {
        let _res = Some(()).status(200).unwrap();
        let _res = Some(()).status(666).unwrap();
    }
}

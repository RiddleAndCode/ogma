#[cfg(not(feature = "std"))]
mod error {
    use alloc::string::{String, ToString};
    use core::fmt;

    #[derive(Debug)]
    pub struct Error(String);

    impl<T> From<T> for Error
    where
        T: fmt::Display,
    {
        fn from(err: T) -> Self {
            Error(err.to_string())
        }
    }
}

#[cfg(not(feature = "std"))]
pub use error::Error;

#[cfg(feature = "std")]
pub use failure::Error;

pub type Fallible<T> = core::result::Result<T, Error>;

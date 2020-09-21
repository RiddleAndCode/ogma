use alloc::string::String;
use core::fmt;

/// An error which may occur during running a function
#[derive(Debug)]
pub enum Trap {
    /// The type of the global differs than the requested type
    DowncastError(&'static str),
    /// The script could not execute the given function
    ScriptOutOfBounds,
    /// A variable is missing
    MissingGlobal(String),
    /// Another custom runtime error
    Runtime(String),
}

impl Trap {
    /// Create a runtime error
    pub fn runtime(err: impl ToString) -> Trap {
        Trap::Runtime(err.to_string())
    }
}

impl fmt::Display for Trap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DowncastError(ty_name) => {
                f.write_fmt(format_args!("could not convert to type: {}", ty_name))
            }
            Self::ScriptOutOfBounds => f.write_str("script out of bounds"),
            Self::MissingGlobal(global_name) => f.write_fmt(format_args!(
                "could not find global variable: {}",
                global_name
            )),
            Self::Runtime(err) => f.write_str(err),
        }
    }
}

impl std::error::Error for Trap {}

use alloc::string::String;

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

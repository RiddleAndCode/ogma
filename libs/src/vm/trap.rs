use alloc::string::String;

#[derive(Debug)]
pub enum Trap {
    DowncastError(&'static str),
    ScriptOutOfBounds,
    MissingGlobal(String),
    Runtime(String),
}

impl Trap {
    pub fn runtime(err: impl ToString) -> Trap {
        Trap::Runtime(err.to_string())
    }
}

use alloc::string::String;

#[derive(Debug)]
pub enum Trap {
    DowncastError(&'static str),
    ScriptOutOfBounds,
    MissingGlobal(String),
    Runtime(String),
}

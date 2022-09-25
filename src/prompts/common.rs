use std::io;

#[derive(Debug, PartialEq)]
pub enum PromptType {
    String,
    Bool,
    Int,
    UInt,
    Float,
    UFloat,
    FilePath,
    FilePathExisting,
}

#[derive(Debug)]
pub enum PromptError {
    IO(io::Error),
    KeyboardInterrupt,
    Custom(String),
}

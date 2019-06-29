//! Definitions of custom errors used in clima
use custom_error::custom_error;
use std::io;

custom_error! {pub ProgramError
    NoPathProvided { } = "No Path Provided",
    FileNotFound { path: String } = "File not found: {}",
    NotRegular { path: String } = "Not a regular file: {}",
    Io {source: io::Error} = "IO Error : {:?}",
}

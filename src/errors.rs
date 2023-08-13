//! Definitions of custom errors used in clima
use {
    custom_error::custom_error,
    std::io,
};

custom_error! {pub ProgramError
    NoPathProvided { } = "No Path Provided",
    FileNotFound { path: String } = "File not found: {path}",
    NotRegular { path: String } = "Not a regular file: {path}",
    Network { source: reqwest::Error} = "Error fetching the remote file: {source}",
    Io {source: io::Error} = "IO Error : {source:?}",
    Termimad {source: termimad::Error} = "Termimad Error : {source:?}",
    Hjson { source: deser_hjson::Error } = "Hjson Error: {source}",
}

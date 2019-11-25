/// this module manages reading and translating
/// the arguments passed on launch of the application.
use crate::errors::ProgramError;
use clap;
use std::path::PathBuf;
use std::result::Result;

pub struct AppLaunchArgs {
    pub target: PathBuf,
    pub just_print: bool,
}

/// declare the possible CLI arguments, and gets the values
fn get_cli_args<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("clima")
        .version(env!("CARGO_PKG_VERSION"))
        .author("dystroy <denys.seguret@gmail.com>")
        .about("minimal rough markdown viewer")
        .arg(clap::Arg::with_name("target").help("sets the file to open"))
        .arg(clap::Arg::with_name("print").long("print").short("p").help("just print to stdout"))
        .get_matches()
}

/// return the parsed launch arguments
pub fn read_launch_args() -> Result<AppLaunchArgs, ProgramError> {
    let cli_args = get_cli_args();
    let target = match cli_args.value_of("target") {
        Some(path) => PathBuf::from(path),
        None => {
            return Err(ProgramError::NoPathProvided {});
        }
    };
    if !target.exists() {
        Err(ProgramError::FileNotFound {
            path: format!("{:?}", &target),
        })?;
    }
    if target.is_dir() {
        Err(ProgramError::NotRegular {
            path: format!("{:?}", &target),
        })?;
    }
    let target = target.canonicalize()?;
    let just_print = cli_args.is_present("print");
    Ok(AppLaunchArgs {
        target,
        just_print,
    })
}

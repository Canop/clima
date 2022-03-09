/// this module manages reading and translating
/// the arguments passed on launch of the application.
use {
    crate::errors::ProgramError,
    std::{
        path::PathBuf,
    },
    cfg_if,
};

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
        Some(path) => {
            cfg_if::cfg_if! {
                if #[cfg(feature = "web")] {
                    if path.starts_with("https://") && !PathBuf::from(path).exists() {
                        link_to_tempbuf(path)?
                    } else {
                        PathBuf::from(path)
                    }
                } else {
                    PathBuf::from(path)
                }
            }
        },
        None => {
            return Err(ProgramError::NoPathProvided {});
        }
    };
    if !target.exists() {
        return Err(ProgramError::FileNotFound {
            path: format!("{:?}", &target),
        });
    }
    if target.is_dir() {
        return Err(ProgramError::NotRegular {
            path: format!("{:?}", &target),
        });
    }
    let target = target.canonicalize()?;
    let just_print = cli_args.is_present("print");
    Ok(AppLaunchArgs {
        target,
        just_print,
    })
}

/// Optional feature to take a target whose value is
/// a URL
#[cfg(feature = "web")]
fn link_to_tempbuf(url: &str) -> Result<PathBuf, ProgramError> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use crate::constants::CLIMA_WEB;

    impl From<reqwest::Error> for ProgramError {
        fn from(_: reqwest::Error) -> Self {
            ProgramError::NoPathProvided { }
        }
    }
    
    let temp_path = Path::new(CLIMA_WEB);
    let display = temp_path.display();
    let mut file = match File::create(&temp_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    let res = reqwest::blocking::get(url)?.text(); 
    match res {
        Ok(md) => {
            file.write_all(md.as_bytes())?;
            Ok(temp_path.to_path_buf())
        },
        Err(_) => {
            Err(ProgramError::FileNotFound {
                path: format!("{:?}", &url),
            })
        }
    }
}

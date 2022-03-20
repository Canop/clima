#[macro_use]
extern crate log;

mod cli;
mod errors;
mod open;
mod viewer;

use {
    crate::{
        cli::get_cli_args,
        errors::ProgramError,
    },
    log::LevelFilter,
    std::{
        env,
        fs::File,
        str::FromStr,
    },
};


/// configure the application log according to env variable.
///
/// There's no log unless the CLIMA_LOG environment variable is set to
///  a valid log level (trace, debug, info, warn, error, off)
/// Example:
///      CLIMA_LOG=info clima
/// As clima is a terminal application, we only log to a file (dev.log)
fn configure_log() {
    let level = env::var("CLIMA_LOG").unwrap_or_else(|_| "off".to_string());
    if level == "off" {
        return;
    }
    if let Ok(level) = LevelFilter::from_str(&level) {
        simplelog::WriteLogger::init(
            level,
            simplelog::Config::default(),
            File::create("dev.log").expect("Log file can't be created"),
        )
        .expect("log initialization failed");
        info!(
            "Starting Clima v{} with log level {}",
            env!("CARGO_PKG_VERSION"),
            level
        );
    }
}

/// run the application, and maybe return a launchable
/// which must be run after clima
fn run() -> Result<(), ProgramError> {
    configure_log();
    let cli_args = get_cli_args();
    let target = match cli_args.value_of("target") {
        Some(path) => path.to_string(),
        None => {
            return Err(ProgramError::NoPathProvided {});
        }
    };
    let md_file = target.parse()?;
    viewer::run(md_file, cli_args.is_present("print"))
}

fn main() {
    if let Err(e) = run() {
        warn!("Error: {}", e);
        eprintln!("{}", e);
    };
    info!("bye");
}

#[macro_use]
extern crate cli_log;

mod cli;
mod errors;
mod open;
mod viewer;

use crate::{
    cli::get_cli_args,
    errors::ProgramError,
};

/// run the application, and maybe return a launchable
/// which must be run after clima
fn run() -> Result<(), ProgramError> {
    init_cli_log!();
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

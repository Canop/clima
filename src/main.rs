#[macro_use]
extern crate cli_log;

mod cli;
mod errors;
mod open;
mod viewer;

use {
    crate::{
        cli::get_cli_args,
        errors::ProgramError,
    },
    std::fs,
    termimad::{
        MadSkin,
    },
};

/// run the application, and maybe return a launchable
/// which must be run after clima
fn main() -> Result<(), ProgramError> {
    init_cli_log!();
    let cli_args = get_cli_args();
    let target = match cli_args.value_of("target") {
        Some(path) => path.to_string(),
        None => {
            return Err(ProgramError::NoPathProvided {});
        }
    };
    let md_file = target.parse()?;
    let skin = match cli_args.value_of("skin") {
        Some(path) => {
            let hjson = fs::read_to_string(&path)?;
            deser_hjson::from_str(&hjson)?
        }
        None => make_skin(),
    };
    viewer::run(
        md_file,
        skin,
        cli_args.is_present("print"),
    )
}

fn make_skin() -> MadSkin {
    match terminal_light::luma() {
        Ok(luma) if luma > 0.6 => MadSkin::default_light(),
        Ok(_) => MadSkin::default_dark(),
        Err(_) => MadSkin::default(), // this skin works in both light and dark
    }
}


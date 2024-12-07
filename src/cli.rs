//! this module manages reading and translating
//! the arguments passed on launch of the application.

/// declare the possible CLI arguments, and gets the values
pub fn get_cli_args<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("clima")
        .version(env!("CARGO_PKG_VERSION"))
        .author("dystroy <denys.seguret@gmail.com>")
        .about("minimal rough markdown viewer")
        .arg(
            clap::Arg::with_name("target")
            .help("path or URL of the file to display")
        )
        .arg(
            clap::Arg::with_name("print")
            .long("print")
            .short("p")
            .help("just print to stdout")
        )
        .arg(
            clap::Arg::with_name("skin")
            .long("skin")
            .takes_value(true)
            .help("path to a JSON or Hjson skin file")
        )
        .get_matches()
}

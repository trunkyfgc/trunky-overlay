#![no_std]
extern crate alloc;
use alloc::string::String;

// const HELP: &str = "\
// trunky-overlay

// USAGE:
//   app [OPTIONS] --number NUMBER [INPUT]

// FLAGS:
//   -h, --help            Prints help information

// OPTIONS:
//   --listen ADDR:PORT    Sets the listen address [default: 0.0.0.0:8080]
//   --fallback TRUE/FALSE Serve fallback.html if not found [default: FALSE]
//   --fallback-behavior   ok/redirect/not-found [default: not-found]
//   --no-index TRUE/FALSE Disable serving index.html if path is directory [default: FALSE]

// ARGS:
//   <INPUT>
// ";

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FallbackBehavior {
    Ok,
    Redirect,
    NotFound,
}

pub struct Opt {
    pub listen: String,
    pub fallback: bool,
    pub fallback_behavior: FallbackBehavior,
    pub no_index: bool,
}

pub fn get_parse_opt() -> Opt {
    // let mut parsed_args = pico_args::Arguments::from_env();

    // if parsed_args.contains(["-h", "--help"]) {
    //     print!("{}", HELP);
    //     std::process::exit(0);
    // }

    let args = Opt {
        listen: String::from("0.0.0.0:8080"),
        fallback: false,
        fallback_behavior: FallbackBehavior::NotFound,
        no_index: false,
    };

    args
}

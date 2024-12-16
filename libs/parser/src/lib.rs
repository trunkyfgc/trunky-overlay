use clap::Parser;

#[derive(Clone, Copy, PartialEq, Eq, Hash, clap::ValueEnum)]
pub enum FallbackBehavior {
    Ok,
    Redirect,
    NotFound,
}

#[derive(clap::Parser)]
pub struct Opt {
    #[clap(default_value = "0.0.0.0:8080", help = "Listen address")]
    pub listen: String,
    #[clap(
        long,
        short,
        help = "Serve fallback.html with code 200 if file not found"
    )]
    pub fallback: bool,
    #[clap(
        long,
        short = 'b',
        help = "Serve 404.html with code 404 if file not found",
        default_value = "not-found"
    )]
    pub fallback_behavior: FallbackBehavior,
    #[clap(
        long,
        short = 'i',
        help = "Disable serving index.html if path is directory"
    )]
    pub no_index: bool,
}

pub fn get_parse_opt() -> Opt {
    Opt::parse()
}

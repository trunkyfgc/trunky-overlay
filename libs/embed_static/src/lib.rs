use axum_embed::ServeEmbed;
use parser;
use parser::{get_parse_opt, FallbackBehavior};
use rust_embed::RustEmbed;

#[derive(RustEmbed, Clone)]
#[folder = "../../static/build/"]
pub struct Static;

pub fn new_service() -> ServeEmbed<Static> {
    let opt = get_parse_opt();

    ServeEmbed::<Static>::with_parameters(
        if opt.fallback {
            Some("404.html".to_owned())
        } else {
            None
        },
        match opt.fallback_behavior {
            FallbackBehavior::Ok => axum_embed::FallbackBehavior::Ok,
            FallbackBehavior::Redirect => axum_embed::FallbackBehavior::Redirect,
            FallbackBehavior::NotFound => axum_embed::FallbackBehavior::NotFound,
        },
        if opt.no_index {
            None
        } else {
            Some("index.html".to_owned())
        },
    )
}

use clap::Parser;
use models::{FallbackBehavior, Opt, Static};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    let listener = TcpListener::bind(opt.listen).await?;
    eprintln!("Listening on http://{}", listener.local_addr()?);
    let assets = axum_embed::ServeEmbed::<Static>::with_parameters(
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
    );
    let app = axum::Router::new().nest_service("/", assets);
    axum::serve(listener, app).await?;

    Ok(())
}

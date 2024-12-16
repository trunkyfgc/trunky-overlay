use parser::get_parse_opt;
use socket;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = get_parse_opt();
    let listener = TcpListener::bind(opt.listen).await?;
    eprintln!("Listening on http://{}", listener.local_addr()?);

    let app = axum::Router::new()
        .layer(socket::new_layer())
        .nest_service("/app", embed_static::new_service())
        .layer(CorsLayer::permissive());

    axum::serve(listener, app).await?;

    Ok(())
}

#![no_std]
#![no_main]

use parser::get_parse_opt;
use socket;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[no_mangle]
#[tokio::main]
async fn main() {
    let opt = get_parse_opt();
    let listener = TcpListener::bind(opt.listen).await.unwrap();
    print_no_std::println!("Listening on http://{}", listener.local_addr().unwrap());

    let app = axum::Router::new()
        .layer(socket::new_layer())
        .nest_service("/app", embed_static::new_service())
        .layer(CorsLayer::permissive());

    axum::serve(listener, app).await.unwrap();
}

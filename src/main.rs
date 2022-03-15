use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Extension, handler::Handler, http::Method, routing::get, Router, Server};
use clap::Parser;
use local_ip_address::local_ip;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    handler::{download, index_handler, index_or_content, static_handler, visit_folder},
    model::{Args, StaticServerConfig},
};

mod handler;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    tracing::debug!(?args);

    let local_ip = local_ip().expect("get local ip").to_string();
    let api_url = format!("{}:{}", local_ip, &args.port.to_string());
    tracing::debug!(?api_url);

    let mut root_dir = args.root_dir;
    if root_dir != "/" {
        root_dir = root_dir.trim_end_matches('/').to_string();
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET]);

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/index_or_content", get(index_or_content))
        .route("/file", get(download))
        .route("/folder", get(visit_folder))
        .layer(cors)
        .layer(Extension(Arc::new(StaticServerConfig { root_dir })))
        .fallback(static_handler.into_service());

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port.into()));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("app make service");
}

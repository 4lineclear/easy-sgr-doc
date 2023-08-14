use std::path::PathBuf;

use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;
#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route(
            "/",
            get(|| async { Html(include_str!("../static/html/index.html")) }),
        )
        .nest_service("/assets", ServeDir::new(static_folder));

    Ok(router.into())
}

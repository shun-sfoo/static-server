use std::{ffi::OsStr, path::Path, sync::Arc};

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, Uri},
    response::IntoResponse,
    Json,
};
use tokio::fs;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::model::{PathInfo, StaticFile, StaticServerConfig};

pub async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

pub async fn get_root(Extension(cfg): Extension<Arc<StaticServerConfig>>) -> impl IntoResponse {
    cfg.root_dir.clone()
}

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    tracing::debug!(?uri);
    let path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}

pub async fn root_path(Extension(cfg): Extension<Arc<StaticServerConfig>>) -> impl IntoResponse {
    let root_dir = Path::new(&cfg.root_dir);
    show_path_list(&root_dir).await
}

pub async fn visit_folder(Json(data): Json<PathInfo>) -> impl IntoResponse {
    let path = Path::new(&data.path_uri);
    show_path_list(&path).await
}

pub async fn download_file(Json(data): Json<PathInfo>) -> impl IntoResponse {
    let svc = ServeFile::new(data.path_uri);
    let res = svc.oneshot(Request::new(Body::empty())).await.unwrap();
    res.map(axum::body::boxed)
}

pub async fn folder(axum::extract::Path(path): axum::extract::Path<String>) -> impl IntoResponse {
    let path = Path::new(&path);
    show_path_list(&path).await
}

async fn show_path_list(path: &Path) -> impl IntoResponse {
    let mut dir = fs::read_dir(path).await.unwrap();
    let mut files: Vec<PathInfo> = Vec::new();

    while let Some(child) = dir.next_entry().await.unwrap() {
        //in root: ./main.rs
        let path_uri = child.path().to_string_lossy().to_string();
        let name = child.file_name().to_string_lossy().to_string();
        let ext = Path::new(child.file_name().to_str().unwrap())
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            .to_string();
        let is_file = child.file_type().await.unwrap().is_file();
        let last_modified = child
            .metadata()
            .await
            .unwrap()
            .modified()
            .unwrap()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let format =
            time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second] UTC")
                .unwrap();

        let last_modified = time::OffsetDateTime::from_unix_timestamp(last_modified)
            .unwrap()
            .format(&format)
            .unwrap();

        files.push(PathInfo {
            name,
            path_uri,
            ext,
            is_file,
            last_modified,
        });
    }

    Json(files)
}

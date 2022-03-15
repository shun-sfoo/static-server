use std::{ffi::OsStr, path::Path, sync::Arc};

use axum::{
    body::Body,
    extract::{Extension, Query},
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

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    tracing::debug!(?uri);
    let path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}

pub async fn index_or_content(
    Extension(cfg): Extension<Arc<StaticServerConfig>>,
) -> impl IntoResponse {
    let path = Path::new(&cfg.root_dir);
    let mut dir = fs::read_dir(path).await.unwrap();

    let mut files: Vec<PathInfo> = Vec::new();

    while let Some(child) = dir.next_entry().await.unwrap() {
        let name = child.file_name().to_string_lossy().to_string();
        let path_uri = name.clone();
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

pub async fn download(
    Query(data): Query<PathInfo>,
    Extension(cfg): Extension<Arc<StaticServerConfig>>,
) -> impl IntoResponse {
    tracing::debug!(?data);
    // 去看 notion 文档
    let svc = ServeFile::new((&cfg.root_dir).to_string() + "/" + &data.name);
    let res = svc.oneshot(Request::new(Body::empty())).await.unwrap();
    res.map(axum::body::boxed)
}

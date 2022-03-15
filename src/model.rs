use axum::{
    body::{boxed, Full},
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use clap::Parser;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long, default_value_t = 9000)]
    pub port: u16,
    #[clap(short, long, default_value = ".")]
    pub root_dir: String,
}

pub struct StaticServerConfig {
    pub root_dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathInfo {
    pub name: String,
    pub path_uri: String,
    pub ext: String,
    pub is_file: bool,
    pub last_modified: i64,
}

#[derive(RustEmbed)]
#[folder = "my-app/build/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> axum::response::Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}

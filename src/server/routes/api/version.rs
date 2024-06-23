use std::time::Duration;

use actix_web::{
    http::header::CONTENT_TYPE,
    post,
    rt::time::sleep,
    web::{Data, Query},
    HttpResponse,
};
use mime::TEXT_PLAIN;
use serde::Deserialize;

#[derive(Deserialize)]
struct VersionQuery {
    v: Option<String>,
}

#[post("/api/version")]
pub async fn handle(query: Query<VersionQuery>, version: Data<String>) -> HttpResponse {
    let server_version = version.parse::<String>().unwrap();

    let client_version = query.v.clone();
    let client_version = client_version.unwrap_or_default();

    if server_version == client_version {
        sleep(Duration::from_secs(900)).await;
    }

    HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, TEXT_PLAIN))
        .body(server_version)
}

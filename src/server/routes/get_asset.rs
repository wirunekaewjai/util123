use std::fs::read;

use actix_web::{
    get,
    http::{
        header::{CacheControl, CacheDirective, ETAG},
        StatusCode,
    },
    web, HttpRequest, HttpResponse,
};

use crate::{
    functions::{create_etag, get_file_mime, get_is_etag_not_modified},
    structs::AppState,
};

#[get("/{filename:.*}")]
pub async fn handle(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    let route_path = req.path();

    let mut is_static = true;
    let mut file_path = state.asset_map[route_path]
        .as_str()
        .unwrap_or_default()
        .to_string();

    if file_path.starts_with("/assets/") {
        file_path = format!("./.cache{}", route_path);
    } else if file_path.starts_with("/") {
        file_path = format!("./public{}", route_path);
        is_static = false;
    }

    if file_path.is_empty() {
        return HttpResponse::NotFound().finish();
    }

    let Ok(buffer) = read(&file_path) else {
        return HttpResponse::NotFound().finish();
    };

    if cfg!(debug_assertions) {
        if is_static {
            println!("asset static: {}", file_path);
        } else {
            println!("asset: {}", file_path);
        }
    }

    let mime = get_file_mime(&file_path);
    let mut builder = HttpResponse::Ok();

    builder.content_type(mime);

    if is_static {
        builder.insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(31_536_000),
        ]));
    } else {
        let headers = req.headers();
        let etag = create_etag(&buffer);

        if get_is_etag_not_modified(headers, &etag) {
            builder.status(StatusCode::NOT_MODIFIED);
        }

        builder.insert_header((ETAG, etag));
        builder.insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(5),
            CacheDirective::MustRevalidate,
        ]));
    }

    return builder.body(buffer);
}

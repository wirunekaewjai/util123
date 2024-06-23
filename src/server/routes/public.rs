use std::fs::read;

use actix_web::{
    get,
    http::{
        header::{CacheControl, CacheDirective, ETAG},
        StatusCode,
    },
    HttpRequest, HttpResponse,
};

use crate::functions;

#[get("/{filename:.*}")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let route_path = req.path();
    let file_path = format!("./public{}", route_path);

    let Ok(buffer) = read(&file_path) else {
        return HttpResponse::NotFound().finish();
    };

    if cfg!(debug_assertions) {
        println!("file: {}", file_path);
    }

    let headers = req.headers();
    let etag = functions::create_etag(&buffer);

    let mut builder = HttpResponse::Ok();

    if functions::get_is_etag_not_modified(headers, &etag) {
        builder.status(StatusCode::NOT_MODIFIED);
    }

    builder.content_type(functions::get_file_mime(&file_path));
    builder.insert_header((ETAG, etag));
    builder.insert_header(CacheControl(vec![
        CacheDirective::Public,
        CacheDirective::MaxAge(5),
        CacheDirective::MustRevalidate,
    ]));

    return builder.body(buffer);
}

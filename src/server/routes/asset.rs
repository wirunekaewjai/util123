use std::fs::read;

use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective},
    web, HttpRequest, HttpResponse,
};

use crate::{functions, structs};

#[get("/assets/{filename:.*}")]
pub async fn handle(req: HttpRequest, state: web::Data<structs::AppState>) -> HttpResponse {
    let route_path = req.path();
    let file_path = state.asset_map[route_path]
        .as_str()
        .unwrap_or_default()
        .to_string();

    if file_path.is_empty() {
        return HttpResponse::NotFound().finish();
    }

    let Ok(buffer) = read(&file_path) else {
        return HttpResponse::NotFound().finish();
    };

    if cfg!(debug_assertions) {
        println!("asset: {}", file_path);
    }

    HttpResponse::Ok()
        .content_type(functions::get_file_mime(&file_path))
        .insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(31_536_000),
        ]))
        .body(buffer)
}

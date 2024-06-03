use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective, ETAG},
    web, HttpRequest, HttpResponse,
};
use jetpack::functions::http::{create_etag, get_is_etag_not_modified};
use mime::TEXT_HTML;
use serde_json::Value;

use crate::{
    structs::AppState,
    views::{self, pages},
};

#[get("/utils/{name}")]
pub async fn handle(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let name = path.into_inner();
    let Some(page) = get_page(&state.hashmap, &name) else {
        return HttpResponse::NotFound().finish();
    };

    let html = views::doc(&state.hashmap, "Utility 123", page);
    let buffer = html.into_bytes();

    let headers = req.headers();
    let etag = create_etag(&buffer);

    let mut builder = match get_is_etag_not_modified(headers, &etag) {
        true => HttpResponse::NotModified(),
        false => HttpResponse::Ok(),
    };

    builder.content_type(TEXT_HTML);
    builder.insert_header((ETAG, etag));
    builder.insert_header(CacheControl(vec![
        CacheDirective::Public,
        CacheDirective::MaxAge(0),
        CacheDirective::MustRevalidate,
    ]));

    return builder.body(buffer);
}

fn get_page(map: &Value, name: &str) -> Option<Vec<String>> {
    if name == "base64" {
        return Some(vec![
            //
            views::topbar(map),
            views::heading(map, "fa-solid-codecompare", "Base64 Encode / Decode"),
            pages::base64(),
        ]);
    }

    if name == "qrcode" {
        return Some(vec![
            //
            views::topbar(map),
            views::heading(map, "fa-solid-qrcode", "QR Code Generator"),
            pages::qrcode(),
        ]);
    }

    if name == "sha" {
        return Some(vec![
            //
            views::topbar(map),
            views::heading(map, "fa-solid-hashtag", "SHA Hash"),
            pages::sha(),
        ]);
    }

    return None;
}

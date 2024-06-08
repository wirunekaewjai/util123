use actix_web::{
    http::header::{CacheControl, CacheDirective, ETAG},
    HttpRequest, HttpResponse,
};
use mime::TEXT_HTML;

use super::{create_etag, get_is_etag_not_modified};

pub fn send_html_response(req: &HttpRequest, html: &str) -> HttpResponse {
    let buffer = html.as_bytes().to_vec();

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

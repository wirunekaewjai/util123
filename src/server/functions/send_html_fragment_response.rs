use actix_web::{
    http::header::{CacheControl, CacheDirective},
    HttpResponse,
};
use mime::TEXT_PLAIN;

pub fn send_html_fragment_response(html: &str) -> HttpResponse {
    let buffer = html.as_bytes().to_vec();
    let mut builder = HttpResponse::Ok();

    builder.content_type(TEXT_PLAIN);
    builder.insert_header(CacheControl(vec![CacheDirective::NoStore]));

    return builder.body(buffer);
}

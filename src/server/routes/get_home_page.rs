use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective, ETAG},
    web, HttpRequest, HttpResponse,
};
use jetpack::http::{create_etag, get_is_etag_not_modified};
use mime::TEXT_HTML;

use crate::{structs::AppState, views};

#[get("/")]
pub async fn handle(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    let items = vec![
        //
        views::topbar(&state.hashmap),
        views::heading(&state.hashmap, "fa-solid-list", "Utilities"),
        views::utility_list(
            &state.hashmap,
            vec![
                //
                ("qrcode", "fa-solid-qrcode", "QR Code Generator"),
                ("sha", "fa-solid-hashtag", "SHA Hash"),
                ("base64", "fa-solid-codecompare", "Base64 Encode / Decode"),
            ],
        ),
    ];

    let html = views::doc(&state.hashmap, "Utility 123", items);
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

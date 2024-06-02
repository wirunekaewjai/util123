use actix_web::{get, HttpRequest, HttpResponse};
use mime::TEXT_HTML;

use crate::views;

#[get("/")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let items = vec![
        //
        views::topbar(),
        views::heading("fa-solid-list", "Utilities"),
        views::utility_list(vec![
            //
            ("qrcode", "fa-solid-qrcode", "QR Code Generator"),
            ("sha", "fa-solid-hashtag", "SHA Hash"),
            ("base64", "fa-solid-codecompare", "Base64 Encode / Decode"),
        ]),
    ];

    let html = views::doc("Utility 123", items);
    let buffer = html.into_bytes();

    let mut builder = HttpResponse::Ok();

    builder.content_type(TEXT_HTML);

    jetpack::bind_etag_header(&mut builder, &req, &buffer);

    return builder.body(buffer);
}

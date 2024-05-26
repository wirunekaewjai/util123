use actix_web::{get, HttpRequest, HttpResponse};
use mime::TEXT_HTML;

use crate::{
    functions::create_etag_response,
    views::{self, icons, pages},
};

#[get("/utils/base64")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let items = vec![
        //
        views::topbar(),
        views::heading(icons::codecompare(), "Base64 Encode / Decode"),
        pages::base64(),
    ];

    let html = views::doc("Utility 123", items);

    return create_etag_response(&req, TEXT_HTML, html.into_bytes());
}

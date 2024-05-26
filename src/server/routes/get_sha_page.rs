use actix_web::{get, HttpRequest, HttpResponse};
use mime::TEXT_HTML;

use crate::{functions::create_etag_response, views};

#[get("/utils/sha")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let items = vec![
        //
        views::topbar(),
        views::heading(&views::icons::icon_hashtag(), "SHA Hash"),
        views::pages::sha(),
    ];

    let html = views::doc("Utility 123", items);

    return create_etag_response(&req, TEXT_HTML, html.into_bytes());
}

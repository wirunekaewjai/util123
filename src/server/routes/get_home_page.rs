use actix_web::{get, HttpRequest, HttpResponse};
use mime::TEXT_HTML;

use crate::{functions::create_etag_response, views};

#[get("/")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let items = vec![
        //
        views::topbar(),
        views::heading(&views::icons::icon_list(), "Utilities"),
        views::utility_list(vec![
            //
            views::UtilityListItem {
                id: "qrcode".into(),
                icon: views::icons::icon_qrcode(),
                name: "QR Code Generator".into(),
            },
            views::UtilityListItem {
                id: "sha".into(),
                icon: views::icons::icon_hashtag(),
                name: "SHA Hash".into(),
            },
            views::UtilityListItem {
                id: "base64".into(),
                icon: views::icons::icon_codecompare(),
                name: "Base64 Encode / Decode".into(),
            },
        ]),
    ];

    let html = views::doc("Utility 123", items);

    return create_etag_response(&req, TEXT_HTML, html.into_bytes());
}

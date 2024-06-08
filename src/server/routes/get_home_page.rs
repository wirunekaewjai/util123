use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::{functions::send_html_response, structs::AppState, views};

#[get("/")]
pub async fn handle(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    let items = vec![
        //
        views::topbar(),
        views::heading("fa-list", "Utilities"),
        views::utility_list(
            //
            vec![
                //
                ("qrcode", "fa-qrcode", "QR Code Generator"),
                ("sha", "fa-hashtag", "SHA Hash"),
                ("base64", "fa-code-compare", "Base64 Encode / Decode"),
            ],
        ),
    ];

    let html = views::doc(&state.asset_map, "Utility 123", items);
    return send_html_response(&req, &html);
}

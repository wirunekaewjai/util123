use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::{
    functions::send_html_response,
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
    let Some(page) = get_page(&name) else {
        return HttpResponse::NotFound().finish();
    };

    let html = views::doc(&state.asset_map, "Utility 123", page);
    return send_html_response(&req, &html);
}

fn get_page(name: &str) -> Option<Vec<String>> {
    if name == "base64" {
        return Some(vec![
            //
            views::topbar(),
            views::heading("fa-code-compare", "Base64 Encode / Decode"),
            pages::base64(),
        ]);
    }

    if name == "qrcode" {
        return Some(vec![
            //
            views::topbar(),
            views::heading("fa-qrcode", "QR Code Generator"),
            pages::qrcode(),
        ]);
    }

    if name == "sha" {
        return Some(vec![
            //
            views::topbar(),
            views::heading("fa-hashtag", "SHA Hash"),
            pages::sha(),
        ]);
    }

    return None;
}

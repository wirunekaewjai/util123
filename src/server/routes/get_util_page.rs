use actix_web::{get, web, HttpRequest, HttpResponse};
use mime::TEXT_HTML;

use crate::views::{self, icons, pages};

#[get("/utils/{name}")]
pub async fn handle(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();
    let Some(page) = get_page(&name) else {
        return HttpResponse::NotFound().finish();
    };

    let html = views::doc("Utility 123", page);

    return jetpack::create_etag_response(&req, &TEXT_HTML, html.into_bytes());
}

fn get_page(name: &str) -> Option<Vec<String>> {
    if name == "base64" {
        return Some(vec![
            //
            views::topbar(),
            views::heading(icons::codecompare(), "Base64 Encode / Decode"),
            pages::base64(),
        ]);
    }

    if name == "qrcode" {
        return Some(vec![
            //
            views::topbar(),
            views::heading(icons::qrcode(), "QR Code Generator"),
            pages::qrcode(),
        ]);
    }

    if name == "sha" {
        return Some(vec![
            //
            views::topbar(),
            views::heading(icons::hashtag(), "SHA Hash"),
            pages::sha(),
        ]);
    }

    return None;
}

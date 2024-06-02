use actix_web::{get, web, HttpRequest, HttpResponse};
use mime::TEXT_HTML;

use crate::views::{self, pages};

#[get("/utils/{name}")]
pub async fn handle(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();
    let Some(page) = get_page(&name) else {
        return HttpResponse::NotFound().finish();
    };

    let html = views::doc("Utility 123", page);
    let buffer = html.into_bytes();

    let mut builder = HttpResponse::Ok();

    builder.content_type(TEXT_HTML);

    jetpack::bind_etag_header(&mut builder, &req, &buffer);

    return builder.body(buffer);
}

fn get_page(name: &str) -> Option<Vec<String>> {
    if name == "base64" {
        return Some(vec![
            //
            views::topbar(),
            views::heading("fa-solid-codecompare", "Base64 Encode / Decode"),
            pages::base64(),
        ]);
    }

    if name == "qrcode" {
        return Some(vec![
            //
            views::topbar(),
            views::heading("fa-solid-qrcode", "QR Code Generator"),
            pages::qrcode(),
        ]);
    }

    if name == "sha" {
        return Some(vec![
            //
            views::topbar(),
            views::heading("fa-solid-hashtag", "SHA Hash"),
            pages::sha(),
        ]);
    }

    return None;
}

use actix_web::{get, web, HttpRequest, HttpResponse};
use html_to_string_macro::html;

use crate::{functions, structs, views};

#[get("/utils/{name}")]
pub async fn handle(
    req: HttpRequest,
    state: web::Data<structs::AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let name = path.into_inner();
    let Some(page) = get_page(&name) else {
        return HttpResponse::NotFound().finish();
    };

    if functions::get_is_fragment_rendering(&req) {
        let html = html!(
            <body>
                {page.join("")}
            </body>
        );

        return functions::send_html_fragment_response(&html);
    }

    let html = views::doc(&state.asset_map, "Utility 123", page);
    functions::send_html_response(&req, &html)
}

fn get_page(name: &str) -> Option<Vec<String>> {
    if name == "base64" {
        return Some(vec![
            //
            views::topbar(),
            views::heading("fa-code-compare", "Base64 Encode / Decode"),
            views::pages::base64(),
        ]);
    }

    if name == "gzip" {
        return Some(vec![
            //
            views::topbar(),
            views::heading("fa-file-zipper", "Gzip Compress / Decompress"),
            views::pages::gzip(),
        ]);
    }

    if name == "qrcode" {
        return Some(vec![
            //
            views::topbar(),
            views::heading("fa-qrcode", "QR Code Generator"),
            views::pages::qrcode(),
        ]);
    }

    if name == "sha" {
        return Some(vec![
            //
            views::topbar(),
            views::heading("fa-hashtag", "SHA Hash"),
            views::pages::sha(),
        ]);
    }

    return None;
}

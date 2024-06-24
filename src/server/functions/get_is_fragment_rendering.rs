use actix_web::HttpRequest;

pub fn get_is_fragment_rendering(req: &HttpRequest) -> bool {
    super::get_request_header_string_value(&req, "X-Fragment").unwrap_or_default() == "1"
}

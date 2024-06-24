use actix_web::HttpRequest;

pub fn get_request_header_string_value(req: &HttpRequest, key: &str) -> Option<String> {
    let Some(value) = req.headers().get(key) else {
        return None;
    };

    let Ok(value) = value.to_str() else {
        return None;
    };

    Some(value.to_string())
}

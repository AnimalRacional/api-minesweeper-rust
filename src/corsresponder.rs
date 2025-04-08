use rocket::http::Header;

#[derive(Responder)]
#[response(content_type = "application/json")]
pub struct CORSResponder {
    body: String,
    corsheader: Header<'static>,
}

impl CORSResponder {
    pub fn new(st: String) -> CORSResponder {
        CORSResponder {
            body: st,
            corsheader: Header::new("Access-Control-Allow-Origin", "*"),
        }
    }
}

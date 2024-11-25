pub struct ApiKey<'r> {
    pub key: &'r str,
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;
    async fn from_request(req: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        // this is very silly but good enough for now
        let api_key = match std::env::var("AUTH_API_KEY") {
            Ok(var) => var,
            Err(_) => {
                if cfg!(debug_assertions) {
                    "DEV".to_string()
                } else {
                    "".to_string()
                }
            },
        };
        if api_key == "" {
            return rocket::request::Outcome::Error((rocket::http::Status::Unauthorized, ApiKeyError::Invalid));
        }
        let is_valid = |key: &str| -> bool {
            key == api_key
        };

        match req.headers().get_one("X-API-KEY") {
            None => rocket::request::Outcome::Error((rocket::http::Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => rocket::request::Outcome::Success(ApiKey { key: key }),
            Some(key) if !is_valid(key) => rocket::request::Outcome::Error((rocket::http::Status::Unauthorized, ApiKeyError::Invalid)),
            Some(_) => rocket::request::Outcome::Error((rocket::http::Status::InternalServerError, ApiKeyError::Invalid)),
        }
    }
}

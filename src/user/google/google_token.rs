use rocket::{Request, request};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;

pub struct GoogleToken {
    pub value: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GoogleToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                Outcome::Success(GoogleToken{value: token.replace("Bearer ", "").to_string()})
            },
            None => Outcome::Failure((Status::BadRequest, ()))
        }
    }
}
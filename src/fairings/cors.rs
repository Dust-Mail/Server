use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::outcome::Outcome;
use rocket::{Request, Response, State};

use crate::state::Config;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        match request.guard::<&State<Config>>().await {
            Outcome::Success(state) => {
                let cors_origin = state.cors_origin();

                response.set_header(Header::new("Access-Control-Allow-Origin", cors_origin));
                response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET"));
                response.set_header(Header::new(
                    "Access-Control-Allow-Headers",
                    "Content-Type, Authorization",
                ));
                response.set_header(Header::new("Access-Control-Max-Age", "86400"));
                response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

                if request.method() == Method::Options {
                    response.set_status(Status::NoContent);
                }
            }
            _ => {}
        }
    }
}

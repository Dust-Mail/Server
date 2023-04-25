use rocket::{http::CookieJar, serde::json::serde_json::from_str as json_from_str};

use crate::{
    guards::RateLimiter,
    oauth2::AccessTokenResponse,
    types::{ErrResponse, Error, ErrorKind, OkResponse, ResponseResult},
};

use super::redirect::OAUTH2_ACCESS_TOKEN_COOKIE_NAME;

#[get("/user")]
pub fn handle_user(
    cookie_jar: &CookieJar<'_>,
    _rate_limiter: RateLimiter,
) -> ResponseResult<AccessTokenResponse> {
    match cookie_jar.get_private(OAUTH2_ACCESS_TOKEN_COOKIE_NAME) {
        Some(cookie) => {
            let user: AccessTokenResponse = json_from_str(cookie.value())
                .map_err(|err| ErrResponse::from(Error::from(err)).into())?;

            Ok(OkResponse::new(user))
        }
        None => Err(ErrResponse::new(
            ErrorKind::Unauthorized,
            "Missing oAuth session cookie",
        )),
    }
}

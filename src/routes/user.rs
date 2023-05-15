use rocket::{http::CookieJar, serde::Serialize, State};

use crate::{
    state::GlobalUserSessions,
    types::{OkResponse, ResponseResult},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub enum UserResponse {
    LoggedIn { sessions: Vec<String> },
    NoSession,
}

#[get("/user")]
pub fn user(
    cookie_jar: &CookieJar,
    user_sessions: &State<GlobalUserSessions>,
) -> ResponseResult<UserResponse> {
    let user_response = match cookie_jar.get_private("session") {
        Some(cookie) => {
            let user_token = cookie.value().to_string();

            let user_session = user_sessions.get(user_token);

            let session_tokens = user_session.session_tokens();

            UserResponse::LoggedIn {
                sessions: session_tokens,
            }
        }
        None => UserResponse::NoSession,
    };

    let ok_response = OkResponse::new(user_response);

    Ok(ok_response)
}

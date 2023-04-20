use crate::{
    guards::{RateLimiter, User},
    types::{ErrResponse, Error, OkResponse, ResponseResult},
};

#[post("/logout?<session_token>")]
pub async fn logout(
    session_token: String,
    user: User,
    _rate_limiter: RateLimiter,
) -> ResponseResult<String> {
    user.mail_sessions().remove(&session_token);

    let incoming_session = user
        .mail_sessions()
        .get_incoming(&session_token)
        .map_err(|err| ErrResponse::from(err).into())?;

    let mut incoming_session_lock = incoming_session.lock().await;

    incoming_session_lock
        .logout()
        .await
        .map_err(|err| ErrResponse::from(Error::from(err)).into())?;

    Ok(OkResponse::new(String::from("Logout successfull")))
}

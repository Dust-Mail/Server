use reqwest::Url;
use rocket::{
    http::{Cookie, CookieJar},
    serde::{
        json::{to_string as to_json, Json},
        Deserialize,
    },
    time::{Duration, OffsetDateTime},
    State,
};

use crate::{
    http::HttpClient,
    oauth2::OAuth2,
    state::Config,
    types::{ErrResponse, Error, ErrorKind, OkResponse, ResponseResult, Result},
};

pub const OAUTH2_ACCESS_TOKEN_COOKIE_NAME: &str = "oauth2_access_token";

#[derive(Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub enum ApplicationType {
    Desktop,
    Web,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct OAuthState {
    provider: String,
    application: ApplicationType,
}

impl OAuthState {
    fn provider(&self) -> &str {
        &self.provider
    }

    fn application_type(&self) -> &ApplicationType {
        &self.application
    }
}

fn create_redirect_uri<U: AsRef<str>>(external_host: U) -> Result<Url> {
    let base_url = Url::parse(external_host.as_ref()).map_err(|_| {
        Error::new(
            ErrorKind::InternalError,
            "external_host config option is not a valid url",
        )
    })?;

    let url = base_url
        .join("mail/oauth2/redirect")
        .map_err(|_| Error::new(ErrorKind::InternalError, "Failed to create redirect uri"))?;

    Ok(url)
}

#[get("/redirect?<code>&<state>&<scope>&<error>")]
pub async fn handle_redirect(
    code: Option<String>,
    state: Json<OAuthState>,
    scope: Option<String>,
    error: Option<String>,
    config: &State<Config>,
    cookie_jar: &CookieJar<'_>,
    http_client: &State<HttpClient>,
) -> ResponseResult<String> {
    if code.is_some() && scope.is_some() {
        let provider = match config.oauth2().providers().get(state.provider()) {
            Some(provider) => provider,
            None => {
                return Err(ErrResponse::new(
                    ErrorKind::BadRequest,
                    "Could not find requested oauth provider",
                ))
            }
        };

        let redirect_uri = create_redirect_uri(config.external_host())
            .map_err(|err| ErrResponse::from(err).into())?;

        let token_url = provider.token_url();
        let secret_token = provider.secret_token();
        let public_token = provider.public_token();
        let code = match code {
            Some(code) => code,
            None => unreachable!(),
        };

        let access_token_response = OAuth2::get_access_token(
            &http_client,
            token_url,
            code.as_str(),
            redirect_uri.to_string(),
            public_token,
            secret_token,
        )
        .await
        .map_err(|err| ErrResponse::from(err).into())?;

        let access_token_json = to_json(&access_token_response)
            .map_err(|err| ErrResponse::from(Error::from(err)).into())?;

        match state.application_type() {
            ApplicationType::Web => match cookie_jar.get_private(OAUTH2_ACCESS_TOKEN_COOKIE_NAME) {
                Some(_cookie) => {}
                None => {
                    let cookie = Cookie::build(OAUTH2_ACCESS_TOKEN_COOKIE_NAME, access_token_json)
                        .http_only(true)
                        .expires(OffsetDateTime::now_utc().saturating_add(Duration::minutes(5)))
                        .finish();

                    cookie_jar.add_private(cookie);
                }
            },
            ApplicationType::Desktop => {}
        };

        Ok(OkResponse::new(String::from(
            "You can now close this window",
        )))
    } else if error.is_some() {
        Err(ErrResponse::new(ErrorKind::BadRequest, "yeet"))
    } else {
        Err(ErrResponse::new(
            ErrorKind::BadRequest,
            "Missing required params",
        ))
    }
}

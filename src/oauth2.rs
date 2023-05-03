use std::collections::HashMap;

use crate::{
    http::HttpClient,
    types::{Error, ErrorKind, Result},
};

use reqwest::{header::CONTENT_TYPE, Method};
use rocket::serde::{json::from_slice as parse_json_from_slice, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde", rename_all = "snake_case")]
pub struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: usize,
    refresh_token: Option<String>,
}

impl AccessTokenResponse {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}

pub async fn get_access_token<S: AsRef<str>>(
    http_client: &HttpClient,
    token_url: S,
    code: S,
    redirect_uri: S,
    client_id: S,
    client_secret: &Option<String>,
) -> Result<AccessTokenResponse> {
    let mut form_data = HashMap::new();

    form_data.insert("grant_type", "authorization_code");
    form_data.insert("code", code.as_ref());
    form_data.insert("redirect_uri", redirect_uri.as_ref());
    form_data.insert("client_id", client_id.as_ref());

    match client_secret.as_ref() {
        Some(client_secret) => {
            form_data.insert("client_secret", client_secret);
        }
        None => {}
    }

    let url: reqwest::Url = token_url.as_ref().parse().map_err(|_| {
        Error::new(
            ErrorKind::Parse,
            "Failed to parse external oauth2 token url",
        )
    })?;

    let response = http_client
        .request(Method::POST, url)
        .form(&form_data)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await?;

    let status = response.status();

    let bytes = response.bytes().await?;

    if status.is_success() {
        let access_token_response: AccessTokenResponse =
            parse_json_from_slice(&bytes).map_err(|err| {
                Error::new(
                    ErrorKind::Parse,
                    format!("Invalid response when fetching oauth access token: {}", err,),
                )
            })?;

        Ok(access_token_response)
    } else {
        let slice = bytes.as_ref();

        let response = std::str::from_utf8(slice).map_err(|err| {
            Error::new(
                ErrorKind::Parse,
                format!(
                    "Failed to parse error response when fetching oauth access token: {}",
                    err,
                ),
            )
        })?;

        Err(Error::new(
            ErrorKind::Oauth2,
            format!("Failed to fetch oauth2 access token: {}", response),
        ))
    }
}

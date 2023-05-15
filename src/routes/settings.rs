use rocket::{serde::Serialize, State};

use crate::{
    state::{AuthType, Config},
    types::{OkResponse, ResponseResult},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]

pub struct SettingsResponse {
    authorization: bool,
    authorization_type: Option<AuthType>,
    mail_proxy: bool,
}

#[get("/settings")]
pub fn settings(config: &State<Config>) -> ResponseResult<SettingsResponse> {
    let auth_enabled = config.authorization().is_some();
    let mail_proxy_enabled = config.mail_proxy().is_some();
    let authorization_type = config
        .authorization()
        .map(|auth_config| auth_config.auth_type().clone());

    let settings_response = SettingsResponse {
        authorization: auth_enabled,
        authorization_type,
        mail_proxy: mail_proxy_enabled,
    };

    Ok(OkResponse::new(settings_response))
}

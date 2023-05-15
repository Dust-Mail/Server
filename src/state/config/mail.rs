use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Mail {
    allowed_servers: Option<Vec<String>>,
    disallowed_servers: Option<Vec<String>>,
}

impl Mail {
    pub fn allowed_servers(&self) -> Option<&Vec<String>> {
        self.allowed_servers.as_ref()
    }

    pub fn disallowed_servers(&self) -> Option<&Vec<String>> {
        self.disallowed_servers.as_ref()
    }
}

impl Default for Mail {
    fn default() -> Self {
        Self {
            allowed_servers: None,
            disallowed_servers: None,
        }
    }
}

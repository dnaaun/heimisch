//! Authentication related types and functions.

use github_api::apis::configuration::Configuration;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;
use std::fmt;

/// The data necessary to authenticate as a Github App
#[derive(Clone)]
pub struct AppAuth {
    /// Github's app ID
    pub app_id: u64,
    /// The app's RSA private key
    pub key: EncodingKey,
}

impl fmt::Debug for AppAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppAuth")
            .field("app_id", &self.app_id)
            .finish_non_exhaustive()
    }
}

/// Create a JSON Web Token that can be used to authenticate an a GitHub application.
///
/// See: https://docs.github.com/en/developers/apps/getting-started-with-apps/setting-up-your-development-environment-to-create-a-github-app#authenticating-as-a-github-app
pub fn create_jwt(
    github_app_id: u64,
    key: &EncodingKey,
) -> Result<String, jsonwebtoken::errors::Error> {
    #[derive(Serialize)]
    struct Claims {
        iss: String,
        iat: usize,
        exp: usize,
    }

    let now = jiff::Timestamp::now().as_jiff_duration().as_secs() as usize;

    // Github only allows JWTs that expire in the next 10 minutes.
    // The token is issued 60 seconds in the past and expires in 9 minutes,
    // to allow some clock drift.
    let claims = Claims {
        iss: github_app_id.to_string(),
        iat: now - 60,
        exp: now + (9 * 60),
    };

    let header = Header::new(Algorithm::RS256);

    jsonwebtoken::encode(&header, &claims, key)
}

impl AppAuth {
    /// Currently we don't cache these, but we could if we want to avoid
    /// an RSA signature operation per App-authorized API call.
    pub fn generate_bearer_token(&self) -> Result<String, jsonwebtoken::errors::Error> {
        create_jwt(self.app_id, &self.key)
    }
}

pub trait WithAppAuth {
    fn with_app_auth(self, app_auth: AppAuth) -> Result<Configuration, jsonwebtoken::errors::Error>;
}

impl WithAppAuth for Configuration {
    fn with_app_auth(self, app_auth: AppAuth) -> Result<Configuration, jsonwebtoken::errors::Error> {
        let thingy = app_auth.generate_bearer_token()?;
        Ok(Self {
            bearer_access_token: Some(format!("Bearer {thingy}")),
            ..self
        })
    }
}

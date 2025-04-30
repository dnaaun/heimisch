mod github_auth;

use std::{env, net::Ipv4Addr};

use dotenvy::dotenv;
use github_auth::AppAuth;
use jsonwebtoken::EncodingKey;
use url::Url;

#[derive(Debug, Clone, Default)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone)]
pub struct GithubApiConfig {
    pub app_auth: AppAuth,
    pub client_id: String,
    pub client_secret: String,
    pub api_root: Url,
    pub non_api_root: Url,
}

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub host: Ipv4Addr,
    pub db: DatabaseConfig,
    pub github_api: GithubApiConfig,
}

impl Config {
    pub fn get_gh_api_conf_with_access_token(
        &self,
        bearer_access_token: impl Into<Option<String>>,
    ) -> github_api::apis::configuration::Configuration {
        github_api::apis::configuration::Configuration {
            user_agent: Some("Heimisch".into()),
            bearer_access_token: bearer_access_token.into(),
            base_path: self.github_api.api_root.clone(),
            client: Default::default(),
        }
    }

    pub fn get_gh_api_conf_with_app_auth(&self) -> github_api::apis::configuration::Configuration {
        github_api::apis::configuration::Configuration {
            user_agent: Some("Heimisch".into()),
            bearer_access_token: Some(self.github_api.app_auth.generate_bearer_token().expect(
            "When `Config` was initialized, we should have checked that `generate_bearer_token()` doesn't crash")),
            base_path: self.github_api.api_root.clone(),
            client: Default::default(),
        }
    }
}

pub async fn init_config() -> Config {
    if let Err(err) = dotenv() {
        // Only log a warning if we're not in CI
        if env::var("CI").is_err() {
            panic!("Warning: Could not load .env file: {}", err);
        }
    }

    let host = env::var("HOST").unwrap().parse().unwrap();
    let port = env::var("PORT").unwrap().parse().unwrap();

    let db = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    let github_api = GithubApiConfig {
        app_auth: AppAuth {
            app_id: env::var("GITHUB_HEIMISCH_APP_ID").unwrap().parse().unwrap(),
            key: EncodingKey::from_rsa_pem(
                env::var("GITHUB_HEIMISCH_PRIVATE_KEY_PEM")
                    .expect("")
                    .as_bytes(),
            )
            .expect("Reading Github App Private Key failed"),
        },
        client_id: env::var("GITHUB_HEIMISCH_CLIENT_ID").expect(""),
        client_secret: env::var("GITHUB_HEIMISCH_CLIENT_SECRET").expect(""),
        api_root: Url::parse("https://api.github.com").expect(""),
        non_api_root: Url::parse("https://github.com").expect(""),
    };

    // RTI: We depend upon this not crashing in the future and just do `.expect("")`.
    github_api
        .app_auth
        .generate_bearer_token()
        .expect("App Auth settings invalid");

    Config {
        port,
        host,
        db,
        github_api,
    }
}

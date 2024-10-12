use std::env;

use dotenvy::dotenv;
use jsonwebtoken::EncodingKey;
use url::Url;

use crate::github_auth::AppAuth;

#[derive(Debug, Clone, Default)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone)]
pub struct GithubApiConfig {
    pub app_auth: AppAuth,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone)]
pub struct Config {
    pub db: DatabaseConfig,
    pub github_api: GithubApiConfig,
    pub heimisch_domain_url: Url,
}

pub async fn init_config() -> Config {
    dotenv().expect("");

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
    };

    Config {
        db,
        github_api,
        heimisch_domain_url: Url::parse(&env::var("HEIMISCH_DOMAIN_NAME").expect("")).expect(""),
    }
}

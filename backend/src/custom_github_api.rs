use serde::Deserialize;
use shared::types::{
    installation::InstallationId, installation_access_token_row::InstallationAccessToken,
};
use url::Url;
use utils::{ExecuteNicely, JsonNicely};

use crate::error::Result;

pub async fn get_user_access_token(
    code: &str,
    client_id: String,
    client_secret: String,
) -> Result<String> {
    let mut github_url = Url::parse("https://github.com/login/oauth/access_token").expect("");
    github_url.query_pairs_mut().extend_pairs([
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("code", code),
    ]);

    #[derive(Deserialize)]
    struct ATResp {
        access_token: String,
    }

    let client = reqwest::Client::new();
    let ATResp { access_token } = client
        .execute_nicely(
            client
                .get(github_url.as_str())
                .header("Accept", "application/json")
                .build()
                .unwrap(),
        )
        .await?
        .json_nicely::<ATResp>()
        .await?;

    Ok(access_token)
}

pub async fn get_installation_access_token(
    installation_id: InstallationId,
    signed_bearer_token: &str,
) -> Result<InstallationAccessToken> {
    let github_url = Url::parse(&format!(
        "https://api.github.com/app/installations/{installation_id}/access_tokens"
    ))
    .expect("");
    let client = reqwest::Client::new();
    let req = client
        .post(github_url)
        .header(
            http::header::AUTHORIZATION,
            format!("Bearer {signed_bearer_token}"),
        )
        .header(http::header::USER_AGENT, "Heimisch")
        .body("{}") // body cpied unquestioningly from what octocrab does.
        .build()
        .expect("");

    Ok(client.execute_nicely(req).await?.json_nicely().await?)
}

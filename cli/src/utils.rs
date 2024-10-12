use deadpool_diesel::sqlite::Pool;

use crate::{auth::login_logic, db::get_user_access_token, error::Result};

/// Will do the login prompt if the user is not currently logged in.
#[allow(unused)]
pub async fn get_user_access_token_ensured(pool: &Pool) -> Result<String> {
    let user_access_token = get_user_access_token(pool).await?;

    match user_access_token {
        Some(user_access_token) => Ok(user_access_token),
        None => login_logic(pool).await,
    }
}

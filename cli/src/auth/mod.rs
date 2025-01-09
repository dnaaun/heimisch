use std::io::{stdin, stdout, Write};

use deadpool_diesel::sqlite::Pool;

use crate::{
    consts::HEIMISCH_API_DOMAIN, db::{delete_access_token_if_exists, set_access_token}, error::Result
};

pub async fn login(pool: &Pool) -> Result<()> {
    login_logic(pool).await?;
    Ok(())
}

/// Returns the access token.
pub async fn login_logic(pool: &Pool) -> Result<String> {
    // NOTE: Polish.
    println!("Follow the URL below to authenticate Heimisch CLI with Github.");
    let url = HEIMISCH_API_DOMAIN.with(|i| i.join("/api/auth/initiate").expect(""));
    println!("{}", url.as_str());
    print!("You should eventually be asked to copy back an access token. Please paste that here: ");
    stdout().flush().expect("");

    let mut access_token = String::new();
    stdin()
        .read_line(&mut access_token)
        .expect("Failed to read line");
    let access_token = access_token.trim().to_owned();

    set_access_token(pool, access_token.clone()).await?;

    Ok(access_token)
}

pub async fn logout(pool: &Pool) -> Result<()> {
    delete_access_token_if_exists(pool).await?;
    println!("Logged out!");
    Ok(())
}

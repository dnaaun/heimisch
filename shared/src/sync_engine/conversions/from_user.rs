use crate::avail::Avail;

pub fn from_user(api_user: github_api::models::User) -> crate::types::user::User {
    let github_api::models::User {
        avatar_url,
        deleted,
        email,
        events_url,
        followers_url,
        following_url,
        gists_url,
        gravatar_id,
        html_url,
        id,
        login,
        name,
        node_id,
        organizations_url,
        received_events_url,
        repos_url,
        site_admin,
        starred_url,
        subscriptions_url,
        r#type,
        url,
    } = api_user;

    crate::types::user::User {
        avatar_url: Avail::from_option(avatar_url),
        deleted: Avail::from_option(deleted),
        email: Avail::from_option(email),
        events_url: Avail::from_option(events_url),
        followers_url: Avail::from_option(followers_url),
        following_url: Avail::from_option(following_url),
        gists_url: Avail::from_option(gists_url),
        gravatar_id: gravatar_id.into(),
        html_url: Avail::from_option(html_url),
        id: i64::from(id).into(),
        login,
        name: name.into(),
        node_id: Avail::from_option(node_id),
        organizations_url: Avail::from_option(organizations_url),
        received_events_url: Avail::from_option(received_events_url),
        repos_url: Avail::from_option(repos_url),
        site_admin: Avail::from_option(site_admin),
        starred_url: Avail::from_option(starred_url),
        subscriptions_url: Avail::from_option(subscriptions_url),
        r#type: Avail::from_option(r#type),
        url: Avail::from_option(url),
        user_view_type: Avail::No,
        bio: Avail::No,
        blog: Avail::No,
        business_plus: Avail::No,
        collaborators: Avail::No,
        company: Avail::No,
        created_at: Avail::No,
        disk_usage: Avail::No,
        followers: Avail::No,
        following: Avail::No,
        hireable: Avail::No,
        ldap_dn: Avail::No,
        location: Avail::No,
        notification_email: Avail::No,
        owned_private_repos: Avail::No,
        private_gists: Avail::No,
        public_gists: Avail::No,
        public_repos: Avail::No,
        total_private_repos: Avail::No,
        twitter_username: Avail::No,
        two_factor_authentication: Avail::No,
        updated_at: Avail::No,
        starred_at: Avail::No,
    }
}

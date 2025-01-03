use crate::avail::Avail;

pub fn from_nullable_simple_user(
    api_user: github_api::models::NullableSimpleUser,
) -> crate::types::user::User {
    let github_api::models::NullableSimpleUser {
        avatar_url,
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
        user_view_type,
        starred_at,
    } = api_user;

    crate::types::user::User {
        avatar_url: avatar_url.into(),
        email: Avail::from_option(email),
        events_url: events_url.into(),
        followers_url: followers_url.into(),
        following_url: following_url.into(),
        gists_url: gists_url.into(),
        gravatar_id: gravatar_id.into(),
        html_url: html_url.into(),
        id: id.into(),
        login,
        name: Avail::from_option(name),
        node_id: node_id.into(),
        organizations_url: organizations_url.into(),
        received_events_url: received_events_url.into(),
        repos_url: repos_url.into(),
        site_admin: site_admin.into(),
        starred_url: starred_url.into(),
        subscriptions_url: subscriptions_url.into(),
        r#type: r#type.into(),
        url: url.into(),
        user_view_type: user_view_type.into(),
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
        deleted: Avail::No,
        starred_at: starred_at.into(),
    }
}

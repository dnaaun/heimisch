use std::convert::Infallible;

use crate::{avail::Avail, sync_engine::conversions::ToDb};

impl ToDb for github_api::models::User2 {
    type DbType = crate::types::user::User;

    type Error = Infallible;

    type OtherChanges = ();

    type Args = ();

    fn try_to_db_type_and_other_changes(
        self,
        _args: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let github_api::models::User2 {
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
            user_view_type,
        } = self;
        Ok((
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
                starred_at: Avail::No,
            },
            (),
        ))
    }
}

impl ToDb for github_api::models::User1 {
    type DbType = crate::types::user::User;

    type Error = Infallible;

    type OtherChanges = ();

    type Args = ();

    fn try_to_db_type_and_other_changes(
        self,
        _args: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let github_api::models::User1 {
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
            user_view_type,
        } = self;

        Ok((
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
                id: id.into(),
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
                starred_at: Avail::No,
            },
            (),
        ))
    }
}

impl ToDb for github_api::models::User {
    type DbType = crate::types::user::User;

    type Error = Infallible;

    type OtherChanges = ();

    type Args = ();

    fn try_to_db_type_and_other_changes(
        self,
        _args: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
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
        } = self;
        Ok((
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
            },
            (),
        ))
    }
}

impl ToDb for github_api::models::NullableSimpleUser {
    type DbType = crate::types::user::User;

    type Error = Infallible;

    type OtherChanges = ();

    type Args = ();

    fn try_to_db_type_and_other_changes(
        self,
        _args: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
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
        } = self;

        Ok((
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
            },
            (),
        ))
    }
}

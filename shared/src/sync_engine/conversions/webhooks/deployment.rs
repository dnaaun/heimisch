use std::convert::Infallible;

use github_webhook_body::*;

use crate::{
    avail::Avail,
    sync_engine::conversions::{InfallibleToDbNoOtherChanges, ToDb},
    types,
};

impl ToDb for DeploymentProtectionRuleRequestedPullRequestsState {
    type Args = ();
    type OtherChanges = ();
    type DbType = github_api::models::milestone::OpenOrClosed;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,

        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use github_api::models::milestone::OpenOrClosed::*;
        let state = match self {
            DeploymentProtectionRuleRequestedPullRequestsState::Closed => Closed,
            DeploymentProtectionRuleRequestedPullRequestsState::Open => Open,
        };
        Ok((state, Default::default()))
    }
}

impl ToDb for DeploymentStatusCreatedDeploymentCreatorType {
    type Args = ();
    type OtherChanges = ();
    type DbType = github_api::models::user::Type;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,

        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use github_api::models::user::Type::*;
        Ok((
            match self {
                DeploymentStatusCreatedDeploymentCreatorType::Bot => Bot,
                DeploymentStatusCreatedDeploymentCreatorType::Organization => Organization,
                DeploymentStatusCreatedDeploymentCreatorType::User => User,
            },
            Default::default(),
        ))
    }
}

impl ToDb for DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions {
    type Args = ();
    type OtherChanges = ();
    type DbType = github_api::models::app_1_permissions::ReadOrWrite;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,

        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use github_api::models::app_1_permissions::ReadOrWrite::*;
        Ok((
            match self {
                DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions::Read => {
                    Read
                }
                DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions::Write => {
                    Write
                }
            },
            Default::default(),
        ))
    }
}
impl ToDb for DeploymentStatusCreatedDeploymentCreator {
    type Args = ();
    type OtherChanges = ();
    type DbType = types::user::User;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,

        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let DeploymentStatusCreatedDeploymentCreator {
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
            type_,
            url,
            user_view_type,
        } = self;
        let id = types::user::UserId::from(id);
        Ok((
            types::user::User {
                avatar_url: Avail::from_option(avatar_url),
                bio: Avail::No,
                blog: Avail::No,
                business_plus: Avail::No,
                collaborators: Avail::No,
                company: Avail::No,
                created_at: Avail::No,
                disk_usage: Avail::No,
                deleted: Avail::from_option(deleted),
                email: email.into(),
                events_url: Avail::from_option(events_url),
                followers: Avail::No,
                followers_url: Avail::from_option(followers_url),
                following: Avail::No,
                following_url: Avail::from_option(following_url),
                gists_url: Avail::from_option(gists_url),
                gravatar_id: gravatar_id.into(),
                hireable: Avail::No,
                html_url: Avail::from_option(html_url),
                id,
                ldap_dn: Avail::No,
                location: Avail::No,
                login,
                name: name.into(),
                node_id: Avail::from_option(node_id),
                notification_email: Avail::No,
                organizations_url: Avail::from_option(organizations_url),
                owned_private_repos: Avail::No,
                private_gists: Avail::No,
                public_gists: Avail::No,
                public_repos: Avail::No,
                received_events_url: Avail::from_option(received_events_url),
                repos_url: Avail::from_option(repos_url),
                site_admin: Avail::from_option(site_admin),
                starred_url: Avail::from_option(starred_url),
                subscriptions_url: Avail::from_option(subscriptions_url),
                total_private_repos: Avail::No,
                twitter_username: Avail::No,
                two_factor_authentication: Avail::No,
                r#type: Avail::from_option(type_.map(|t| t.to_db_type(()))),
                updated_at: Avail::No,
                url: Avail::from_option(url),
                user_view_type: user_view_type.into(),
                starred_at: Avail::No,
            },
            Default::default(),
        ))
    }
}
impl ToDb for DeploymentReviewApprovedReviewersReviewer {
    type Args = ();
    type OtherChanges = ();
    type DbType = types::user::User;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,

        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let DeploymentReviewApprovedReviewersReviewer {
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
            type_,
            url,
        } = self;
        let id = types::user::UserId::from(id);
        Ok((
            types::user::User {
                avatar_url: Avail::from_option(avatar_url),
                bio: Avail::No,
                blog: Avail::No,
                business_plus: Avail::No,
                collaborators: Avail::No,
                company: Avail::No,
                created_at: Avail::No,
                disk_usage: Avail::No,
                deleted: Avail::from_option(deleted),
                email: email.into(),
                events_url: Avail::from_option(events_url),
                followers: Avail::No,
                followers_url: Avail::from_option(followers_url),
                following: Avail::No,
                following_url: Avail::from_option(following_url),
                gists_url: Avail::from_option(gists_url),
                gravatar_id: gravatar_id.into(),
                hireable: Avail::No,
                html_url: Avail::from_option(html_url),
                id,
                ldap_dn: Avail::No,
                location: Avail::No,
                login,
                name: name.into(),
                node_id: Avail::from_option(node_id),
                notification_email: Avail::No,
                organizations_url: Avail::from_option(organizations_url),
                owned_private_repos: Avail::No,
                private_gists: Avail::No,
                public_gists: Avail::No,
                public_repos: Avail::No,
                received_events_url: Avail::from_option(received_events_url),
                repos_url: Avail::from_option(repos_url),
                site_admin: Avail::from_option(site_admin),
                starred_url: Avail::from_option(starred_url),
                subscriptions_url: Avail::from_option(subscriptions_url),
                total_private_repos: Avail::No,
                twitter_username: Avail::No,
                two_factor_authentication: Avail::No,
                r#type: Avail::from_option(type_.map(|t| t.to_db_type(()))),
                updated_at: Avail::No,
                url: Avail::from_option(url),
                user_view_type: Avail::No,
                starred_at: Avail::No,
            },
            Default::default(),
        ))
    }
}

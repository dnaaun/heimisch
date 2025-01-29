use std::convert::Infallible;

use futures::future::OptionFuture;
use github_webhook_body::*;

use crate::{
    avail::Avail,
    sync_engine::{
        changes::{AddChanges, Changes},
        conversions::{conversion_error::ConversionError, InfallibleToDbNoOtherChanges, ToDb},
    },
    types,
};

impl ToDb for MilestoneClosedMilestoneCreator {
    type Args = ();
    type OtherChanges = ();
    type DbType = types::user::User;

    type Error = Infallible;

    async fn try_to_db_type_and_other_changes(
        self,

        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let MilestoneClosedMilestoneCreator {
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
                r#type: Avail::from_option(
                    OptionFuture::from(type_.map(|t| t.to_db_type(()))).await,
                ),
                updated_at: Avail::No,
                url: Avail::from_option(url),
                user_view_type: user_view_type.into(),
                starred_at: Avail::No,
            },
            Default::default(),
        ))
    }
}

impl ToDb for MilestoneClosedMilestone {
    type Args = ();
    type OtherChanges = Changes;
    type DbType = types::milestone::Milestone;

    type Error = ConversionError;

    async fn try_to_db_type_and_other_changes(
        self,
        _: Self::Args,
    ) -> Result<(Self::DbType, Changes), Self::Error> {
        let MilestoneClosedMilestone {
            closed_at,
            closed_issues,
            created_at,
            creator,
            description,
            due_on,
            html_url,
            id,
            labels_url,
            node_id,
            number,
            open_issues,
            state,
            title,
            updated_at,
            url,
        } = self;
        let creator = OptionFuture::from(creator.map(|c| c.to_db_type(()))).await;

        let milestone = types::milestone::Milestone {
            closed_at: Avail::Yes(closed_at.map(|d| (d).parse()).transpose()?),
            closed_issues: closed_issues.into(),
            created_at: Avail::Yes((created_at).parse()?),
            creator_id: creator.as_ref().map(|c| c.id).into(),
            description: description.into(),
            due_on: Avail::Yes(due_on.map(|d| serde_json::from_str(&d)).transpose()?),
            html_url: html_url.into(),
            id: types::milestone::MilestoneId::from(id),
            labels_url: labels_url.into(),
            node_id: node_id.into(),
            number: number.into(),
            open_issues: open_issues.into(),
            state: state.to_db_type(()).await.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
        };

        let mut changes = Changes::default();
        changes.add(creator)?;

        Ok((milestone, changes))
    }
}

impl ToDb for MilestoneCreatedMilestone {
    type Args = ();
    type OtherChanges = Changes;
    type DbType = types::milestone::Milestone;

    type Error = ConversionError;

    async fn try_to_db_type_and_other_changes(
        self,
        _: Self::Args,
    ) -> Result<(Self::DbType, Changes), Self::Error> {
        let MilestoneCreatedMilestone {
            closed_at,
            closed_issues,
            created_at,
            creator,
            description,
            due_on,
            html_url,
            id,
            labels_url,
            node_id,
            number,
            open_issues,
            state,
            title,
            updated_at,
            url,
        } = self;
        let creator = OptionFuture::from(creator.map(|c| c.to_db_type(()))).await;

        let milestone = types::milestone::Milestone {
            closed_at: Avail::Yes(closed_at.map(|d| (d).parse()).transpose()?),
            closed_issues: closed_issues.into(),
            created_at: Avail::Yes((created_at).parse()?),
            creator_id: creator.as_ref().map(|c| c.id).into(),
            description: description.into(),
            due_on: Avail::Yes(due_on.map(|d| serde_json::from_str(&d)).transpose()?),
            html_url: html_url.into(),
            id: types::milestone::MilestoneId::from(id),
            labels_url: labels_url.into(),
            node_id: node_id.into(),
            number: number.into(),
            open_issues: open_issues.into(),
            state: state.to_db_type(()).await.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
        };

        let mut changes = Changes::default();
        changes.add(creator)?;

        Ok((milestone, changes))
    }
}

impl ToDb for MilestoneClosedMilestoneCreatorType {
    type Args = ();
    type OtherChanges = ();
    type DbType = github_api::models::user::Type;

    type Error = Infallible;

    async fn try_to_db_type_and_other_changes(
        self,
        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use github_api::models::user::Type::*;
        Ok((
            match self {
                MilestoneClosedMilestoneCreatorType::Bot => Bot,
                MilestoneClosedMilestoneCreatorType::Organization => Organization,
                MilestoneClosedMilestoneCreatorType::User => User,
                MilestoneClosedMilestoneCreatorType::Mannequin => Mannequin,
            },
            Default::default(),
        ))
    }
}

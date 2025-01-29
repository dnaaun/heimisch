use std::convert::Infallible;

use github_webhook_body::*;

use crate::{sync_engine::conversions::ToDb, types};

impl ToDb for DiscussionAnsweredAnswerReactions {
    type Args = ();
    type OtherChanges = ();
    type DbType = github_api::models::Reactions;

    type Error = Infallible;

    async fn try_to_db_type_and_other_changes(
        self,

        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let DiscussionAnsweredAnswerReactions {
            confused,
            eyes,
            heart,
            hooray,
            laugh,
            minus1,
            plus1,
            rocket,
            total_count,
            url,
        } = self;
        Ok((
            github_api::models::Reactions {
                plus_1: plus1,
                _1: minus1,
                confused,
                eyes,
                heart,
                hooray,
                laugh,
                rocket,
                total_count,
                url,
            },
            Default::default(),
        ))
    }
}

impl ToDb for DiscussionLabeledLabel {
    type Args = ();
    type OtherChanges = ();
    type DbType = types::label::Label;

    type Error = Infallible;

    async fn try_to_db_type_and_other_changes(
        self,

        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        todo!()
    }
}

impl ToDb for DiscussionAnsweredAnswerAuthorAssociation {
    type Args = ();
    type OtherChanges = ();
    type DbType = github_api::models::AuthorAssociation;

    type Error = Infallible;

    async fn try_to_db_type_and_other_changes(
        self,

        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use github_api::models::AuthorAssociation::*;
        Ok((
            match self {
                DiscussionAnsweredAnswerAuthorAssociation::Collaborator => Collaborator,
                DiscussionAnsweredAnswerAuthorAssociation::Contributor => Contributor,
                DiscussionAnsweredAnswerAuthorAssociation::FirstTimer => FirstTimer,
                DiscussionAnsweredAnswerAuthorAssociation::FirstTimeContributor => {
                    FirstTimeContributor
                }
                DiscussionAnsweredAnswerAuthorAssociation::Mannequin => Mannequin,
                DiscussionAnsweredAnswerAuthorAssociation::Member => Member,
                DiscussionAnsweredAnswerAuthorAssociation::None => None,
                DiscussionAnsweredAnswerAuthorAssociation::Owner => Owner,
            },
            Default::default(),
        ))
    }
}

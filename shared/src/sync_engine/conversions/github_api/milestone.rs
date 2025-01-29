use crate::{
    avail::MergeError,
    sync_engine::{
        changes::{AddChanges, Changes},
        conversions::{InfallibleToDbNoOtherChanges, MapToFuture, ToDb},
    },
};

impl ToDb for github_api::models::Milestone1 {
    type DbType = crate::types::milestone::Milestone;

    type Error = MergeError;

    type OtherChanges = Changes;

    type Args = ();

    async fn try_to_db_type_and_other_changes(
        self,
        _args: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let github_api::models::Milestone1 {
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

        let db_creator = creator.map_to_future(|c| c.to_db_type(())).await;

        let db_milestone = crate::types::milestone::Milestone {
            closed_at: closed_at.into(),
            closed_issues: i64::from(closed_issues).into(),
            created_at: created_at.into(),
            creator_id: db_creator.as_ref().map(|c| c.id).into(),
            description: description.into(),
            due_on: due_on.into(),
            html_url: html_url.into(),
            id: i64::from(id).into(),
            labels_url: labels_url.into(),
            node_id: node_id.into(),
            number: i64::from(number).into(),
            open_issues: i64::from(open_issues).into(),
            state: state.into(),
            title: title.into(),
            updated_at: updated_at.into(),
            url: url.into(),
        };

        let mut changes = Changes::default();
        changes.add(db_creator)?;

        Ok((db_milestone, changes))
    }
}

use crate::{
    avail::MergeError,
    sync_engine::changes::{AddChanges, Changes},
    types::milestone::MilestoneId,
};

use super::from_user2::from_user2;

pub fn from_milestone1(
    api_milestone: github_api::models::Milestone1,
) -> Result<(MilestoneId, Changes), MergeError> {
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
    } = api_milestone;

    let db_creator = creator.map(|c| from_user2(*c));

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

    let id = db_milestone.id;

    let mut changes = Changes::default();
    changes.add(db_milestone)?.add(db_creator)?;

    Ok((id, changes))
}

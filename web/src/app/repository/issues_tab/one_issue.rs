use leptos::prelude::*;
use shared::types::repository::RepositoryId;

use crate::app::routing::ParamsOwnerNameRepoNameIssueNumber;

#[allow(non_snake_case)]
pub fn OneIssue(
    #[allow(unused_variables)] child_component: impl Fn(()) -> AnyView + Send + Sync,
    #[allow(unused_variables)] captures: ParamsOwnerNameRepoNameIssueNumber,
    #[allow(unused_variables)] repository_id: Signal<RepositoryId>,
) -> impl IntoView {
    "Sup yo!!"
}

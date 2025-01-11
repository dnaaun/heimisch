use leptos::prelude::*;
use shared::types::repository::RepositoryId;

use crate::app::routing::Part1OwnerNamePart2RepoNamePart3PullsCaptures;

pub fn PullRequestsTab(
    #[allow(unused_variables)] child_component: impl Fn(()) -> AnyView + Send + Sync,
    #[allow(unused_variables)] captures: Memo<Part1OwnerNamePart2RepoNamePart3PullsCaptures>,
    #[allow(unused_variables)] arg_from_parent: Signal<RepositoryId>,
) -> impl IntoView {
    view! { <div>pull request tab</div> }
}

use leptos::prelude::*;
use shared::types::repository::RepositoryId;

use crate::app::routing::Part1OwnerNamePart2RepoNamePart3PullsCaptures;

pub fn PullRequestsTab(
    #[allow(unused_variables)] child_component: impl Fn(()) -> AnyView + Send + Sync,
    #[allow(unused_variables)] captures: Memo<Part1OwnerNamePart2RepoNamePart3PullsCaptures>,
    #[allow(unused_variables)] repository_id: Signal<RepositoryId>,
) -> impl IntoView {
    Effect::new(move || tracing::info!("repo id in Pull requests tab: {:?}", repository_id.get()));
    view! { <div>pull request tab</div> }
}

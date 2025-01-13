pub mod list;
pub mod one_issue;

use leptos::prelude::*;
use list::IssuesList;
use shared::types::repository::RepositoryId;

use crate::app::routing::ParamsOwnerNameRepoName;

use super::{ArgFromParent, Outlet, RouteParams};

#[allow(non_snake_case)]
fn IssuesTab(
    #[allow(unused_variables)] child_component: impl Fn(Signal<RepositoryId>) -> AnyView + Send + Sync,
    #[allow(unused_variables)] repository_id: Signal<RepositoryId>,
) -> impl IntoView {
    child_component(repository_id)
}

#[allow(non_snake_case)]
pub fn IssuesTabEmpty(
    params: RouteParams<ParamsOwnerNameRepoName>,
    ArgFromParent(repository_id): ArgFromParent<Signal<RepositoryId>>,
) -> impl IntoView {
    view! { <IssuesList repository_id owner_name=params.owner_name repo_name=params.repo_name /> }
}

#[allow(non_snake_case)]
pub fn IssuesTabWithIssues(
    outlet: Outlet<Signal<RepositoryId>, impl IntoView>,
    repository_id: Signal<RepositoryId>,
) -> impl IntoView {
    IssuesTab(outlet, repository_id)
}

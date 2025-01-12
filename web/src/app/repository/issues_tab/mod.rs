pub mod list;
pub mod one_issue;

use leptos::prelude::*;
use list::IssuesList;
use shared::types::repository::RepositoryId;

use super::{
    Part1OwnerNamePart2RepoNamePart3EmptyCaptures, Part1OwnerNamePart2RepoNamePart3IssuesCaptures,
};

#[allow(non_snake_case)]
fn IssuesTab(
    #[allow(unused_variables)] child_component: impl Fn(Signal<RepositoryId>) -> AnyView + Send + Sync,
    #[allow(unused_variables)] repository_id: Signal<RepositoryId>,
) -> impl IntoView {
    child_component(repository_id)
}

#[allow(non_snake_case)]
pub fn IssuesTabEmpty(
    #[allow(unused_variables)] child_component: impl Fn(Signal<RepositoryId>) -> AnyView + Send + Sync,
    #[allow(unused_variables)] captures: Memo<Part1OwnerNamePart2RepoNamePart3EmptyCaptures>,
    #[allow(unused_variables)] repository_id: Signal<RepositoryId>,
) -> impl IntoView {
    let owner_name = Signal::derive(move || {
        captures
            .get()
            .prev_captures
            .get()
            .prev_captures
            .get()
            .owner_name
    });
    let repo_name = Signal::derive(move || {
        captures
            .get()
            .prev_captures
            .get()
            .repo_name
    });

    view! { <IssuesList repository_id owner_name repo_name /> }

}

#[allow(non_snake_case)]
pub fn IssuesTabWithIssues(
    #[allow(unused_variables)] child_component: impl Fn(Signal<RepositoryId>) -> AnyView + Send + Sync,
    #[allow(unused_variables)] captures: Memo<Part1OwnerNamePart2RepoNamePart3IssuesCaptures>,
    #[allow(unused_variables)] repository_id: Signal<RepositoryId>,
) -> impl IntoView {
    IssuesTab(child_component, repository_id)
}

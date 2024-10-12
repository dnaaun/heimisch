use leptos::prelude::*;
use shared::types::repository::RepositoryId;

#[component]
pub fn PullRequestsTab(_repository_id: RepositoryId) -> impl IntoView {
    view! {
        <div>
            pull request tab
        </div>
    }
}

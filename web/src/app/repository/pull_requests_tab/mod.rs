use leptos::prelude::*;
use shared::types::repository::RepositoryId;

#[component]
pub fn PullRequestsTab(
    #[allow(unused)]
    #[prop(into)]
    repository_id: Signal<RepositoryId>,
) -> impl IntoView {
    view! { <div>pull request tab</div> }
}

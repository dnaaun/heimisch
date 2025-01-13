pub mod list;
pub mod one_issue;

use leptos::prelude::*;
use shared::types::repository::RepositoryId;

use super::{ArgFromParent, Outlet};

#[allow(non_snake_case)]
pub fn IssuesTab(
    outlet: Outlet<Signal<RepositoryId>, impl IntoView>,
    ArgFromParent(repository_id): ArgFromParent<Signal<RepositoryId>>,
) -> impl IntoView {
    outlet.call(repository_id)
}

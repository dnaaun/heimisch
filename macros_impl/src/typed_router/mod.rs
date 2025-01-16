pub mod main_model;
pub mod parsing;
pub mod try_from_slashed_impl;
pub mod write_output;

#[cfg(test)]
const TEST_STR: &str = r#"
{
    fallback: NotFound,
    view: Sidebar,
    children: [
        {
            path: "/auth",
            view: Auth
        },
        {
            path: "/{owner_name}",
            children: [
                {
                    path: "/{repo_name}",
                    layout: RepositoryPage,
                    view: IssuesTab,
                    will_pass: Signal<RepositoryId>,
                    children: [
                        {
                            path: "/pulls",
                            view: PullRequestsTab
                        },
                        {
                            path: "/issues",
                            will_pass: Signal<RepositoryId>,
                            view: IssuesList,
                            children: [ {
                                    path: "/{issue_number}",
                                    view: OneIssue
                                }
                            ]
                        },
                    ]
                }
            ]
        }
    ]
}
"#;

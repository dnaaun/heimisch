pub mod main_model;
pub mod parsing;
pub mod try_from_slashed_impl;
pub mod write_output;

#[cfg(test)]
const TEST_STR: &str = r#"
fallback: NotFound,
{

    {
        path: "/auth",
        view: Auth
    },
    {
        path: "/",
        view: Sidebar,
        children: [
            {
                path: "/{owner_name}",
                children: [
                    {
                        path: "/{repo_name}"
                        view: RepositoryPage,
                        will_pass: Signal<RepositoryId>,
                        children: [
                            {
                                path: "/pulls",
                                view: PullRequestsTab
                            },
                            {
                                path: "/",
                                view: IssuesTab
                            },
                            {
                                path: "/issues",
                                will_pass: Signal<RepositoryId>,
                                children: [
                                    {
                                        path: "/",
                                        view: IssuesList
                                    },
                                    {
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
}
"#;

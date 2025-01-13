pub mod main_model;
pub mod parsing;
pub mod try_from_slashed_impl;
pub mod enum_impl;

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
                        will_pass: RepositoryId,
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

pub mod main_model;
pub mod parsing;

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

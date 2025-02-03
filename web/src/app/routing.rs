use crate::app::{
    repository::{
        issues_tab::{list::IssuesList, one_issue::OneIssue},
        RepositoryPage,
    },
    sidebar::Sidebar,
};

use super::{
    not_found::NotFound,
    repository::{issues_tab::new_issue::NewIssue, pull_requests_tab::PullRequestsTab, RepositoryPageWillPass},
};
use crate::app::auth::Auth;

use zwang_router::zwang_routes;

zwang_routes! {{
    view: Sidebar,
    children: [
        {
            path: "/auth",
            view: Auth
        },
        {
            path: "/{owner_name}",
            layout: Sidebar,
            children: [
                {
                    path: "/{repo_name}",
                    layout: RepositoryPage,
                    view: IssuesList,
                    will_pass: RepositoryPageWillPass,
                    children: [
                        {
                            path: "/pulls",
                            view: PullRequestsTab
                        },
                        {
                            path: "/issues",
                            will_pass: RepositoryPageWillPass,
                            view: IssuesList,
                            children: [
                                {
                                    path: "/new",
                                    view: NewIssue
                                },
                                {
                                    path: "/{issue_number}",
                                    view: OneIssue
                                },
                            ]
                        },
                    ]
                }
            ]
        }
    ]
}}

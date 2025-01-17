use syn::{parse::Parse, parse_macro_input};
use write_output::write_output;

pub mod main_model;
pub mod parsing;
pub mod try_from_slashed_impl;
pub mod write_output;

pub fn zwang_routes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as parsing::Part);
    main_model::Parts::try_from(parsed)
        .map(write_output)
        .flatten()
        .unwrap_or_else(|x| x.into_compile_error())
        .into()
}

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

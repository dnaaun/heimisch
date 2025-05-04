mod path;
mod routes;

use routes::{main_model, parsing, write_output::write_output};
use syn::parse_macro_input;

pub fn zwang_routes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as parsing::Part);
    main_model::Parts::try_from(parsed)
        .and_then(write_output)
        .unwrap_or_else(|x| x.into_compile_error())
        .into()
}

pub use path::zwang_url;

#[cfg(test)]
const TEST_STR: &str = r#"
{
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
                    will_pass: RepositoryPageContext,
                    children: [
                        {
                            path: "/pulls",
                            view: PullRequestsTab
                        },
                        {
                            path: "/issues",
                            will_pass: RepositoryPageContext,
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
}
"#;

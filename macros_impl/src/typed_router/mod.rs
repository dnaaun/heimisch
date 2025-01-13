use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, Ident, LitStr, Token};

// Represents a collection of routes with an optional fallback
struct Routes {
    fallback: Ident,
    routes: Vec<Route>,
}

// Represents an individual route
#[derive(Debug)]
struct Route {
    path: Vec<PathSegment>,
    view: Option<Ident>,
    children: Option<Vec<Route>>,
    will_pass: Option<Ident>
}

// Represents static versus parameterized path segments
#[derive(Debug)]
enum PathSegment {
    Static(String),
    Param(String),
}

// Function to parse path segments
fn parse_path(input: ParseStream) -> Result<Vec<PathSegment>> {
    let path: LitStr = input.parse()?;
    let content = path.value();
    let segments = content
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| {
            if s.starts_with('{') && s.ends_with('}') {
                PathSegment::Param(s[1..s.len() - 1].to_owned())
            } else {
                PathSegment::Static(s.to_owned())
            }
        })
        .collect();
    Ok(segments)
}

// Parsing logic for individual routes
impl Parse for Route {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut path = None;
        let mut view = None;
        let mut will_pass = None;
        let mut children = None;

        let inside_braces;
        braced!(inside_braces in input);
        loop {
            let _ = inside_braces.parse::<Token![,]>();
            if inside_braces.is_empty() {
                break
            }

            let ident: Ident = inside_braces.parse()?;
            let _ = inside_braces.parse::<Token![:]>();
            match &*ident.to_string() {
                "path" => {
                    if path.is_some() {
                        return Err(inside_braces.error("Found `path` specified twice."));
                    }
                    path = Some(parse_path(&inside_braces)?);
                }
                "view" => {
                    if view.is_some() {
                        return Err(inside_braces.error("Found `view` specified twice."));
                    }
                    view = Some(inside_braces.parse()?);
                }
                "will_pass" => {
                    if will_pass.is_some() {
                        return Err(inside_braces.error("Found `will_pass` specified twice."));
                    }
                    will_pass = Some(inside_braces.parse()?);
                }
                "children" => {
                    if children.is_some() {
                        return Err(inside_braces.error("Found `children` specified twice."));
                    }
                    let content;
                    syn::bracketed!(content in inside_braces);
                    children = Some(
                        content
                            .parse_terminated(Route::parse, Token![,])?
                            .into_iter()
                            .collect(),
                    );
                }
                key @ _ => {
                    return Err(inside_braces.error(format!("unexpected key found: '{key}'")));
                }
            }
        }

        Ok(Route {
            path: path.expect("`path` not specified."),
            view,
            children,
            will_pass
        })
    }
}

pub fn parse_fallback(input: ParseStream) -> Result<Ident> {
    if Ident::parse(input)?.to_string() != "fallback" {
        panic!("Expected fallback.");
    }
    let _ = input.parse::<Token![:]>()?;
    let ident = input.parse();
    let _ = input.parse::<Token![,]>();
    ident
}

// Parsing logic for multiple routes and fallback
impl Parse for Routes {
    fn parse(input: ParseStream) -> Result<Self> {
        let fallback = parse_fallback(input)?;
        let content;
        braced!(content in input);

        let routes = content
            .parse_terminated(Route::parse, Token![,])?
            .into_iter()
            .collect();

        Ok(Routes { fallback, routes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn test_parse_routes_with_fallback() {
        let routes_str = r#"
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

        let parsed: Routes = parse_str(routes_str).expect("Unable to parse routes");
        assert_eq!(parsed.fallback.to_string(), "NotFound");
    }
}

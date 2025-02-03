use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use syn::*;

// Represents an individual route
#[derive(Debug)]
pub struct Part {
    pub path: Option<(PathSegment, Span)>,
    pub view: Option<Ident>,
    // pub fallback: Option<Ident>,
    pub layout: Option<Ident>,
    pub sub_parts: Vec<Part>,
    pub will_pass: Option<Type>,
    pub span: Span,
}

// Represents static versus parameterized path segments
#[derive(Debug)]
pub enum PathSegment {
    Static(String),
    Param(String),
}

impl std::ops::Deref for PathSegment {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        match self {
            PathSegment::Static(d) => d,
            PathSegment::Param(d) => d,
        }
    }
}

impl std::fmt::Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PathSegment::Static(s) => s,
            PathSegment::Param(s) => s,
        })
    }
}

// Function to parse path segments
fn parse_path(input: ParseStream) -> Result<(PathSegment, Span)> {
    let path: LitStr = input.parse()?;
    let span = path.span();
    let content = &path.value();
    let mut content_chars = content.chars();
    if content_chars.next() != Some('/') || content_chars.any(|x| x == '/') {
        return Err(Error::new(
            span,
            "`path` must start with a slash, and contain none thereafter.",
        ));
    }
    let s = &content[1..];

    let segment = if s.starts_with('{') && s.ends_with('}') {
        PathSegment::Param(s[1..s.len() - 1].to_owned())
    } else {
        PathSegment::Static(s.to_owned())
    };
    Ok((segment, span))
}

// Parsing logic for individual routes
impl Parse for Part {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut path = None;
        let mut view = None;
        let mut layout = None;
        let mut will_pass = None;
        let mut children = None;
        // let mut fallback = None;

        let inside_braces;
        braced!(inside_braces in input);
        loop {
            let ident: Ident = inside_braces.parse()?;
            let _ = inside_braces.parse::<Token![:]>();
            match &*ident.to_string() {
                "path" => {
                    if path.is_some() {
                        return Err(input.error("Found `path` specified twice."));
                    }
                    path = Some(parse_path(&inside_braces)?);
                }
                // "fallback" => {
                //     if fallback.is_some() {
                //         return Err(input.error("Found `fallback` specified twice."));
                //     }
                //     fallback = inside_braces.parse()?;
                // }
                "view" => {
                    if view.is_some() {
                        return Err(input.error("Found `view` specified twice."));
                    }
                    view = inside_braces.parse()?;
                }
                "layout" => {
                    if layout.is_some() {
                        return Err(input.error("Found `layout` specified twice."));
                    }
                    layout = inside_braces.parse()?;
                }
                "will_pass" => {
                    if will_pass.is_some() {
                        return Err(input.error("Found `will_pass` specified twice."));
                    }
                    will_pass = Some(inside_braces.parse()?);
                }
                "children" => {
                    if children.is_some() {
                        return Err(input.error("Found `children` specified twice."));
                    }
                    let content;
                    syn::bracketed!(content in inside_braces);
                    children = Some(
                        content
                            .parse_terminated(Part::parse, Token![,])?
                            .into_iter()
                            .collect(),
                    );
                }
                key => {
                    return Err(input.error(format!("unexpected key found: '{key}'")));
                }
            }

            match inside_braces.parse::<Token![,]>() {
                Ok(_) => (),
                Err(e) => {
                    if inside_braces.is_empty() {
                        break;
                    } else {
                        println!("{}", inside_braces);
                        return Err(e);
                    }
                }
            }
        }

        Ok(Part {
            path,
            view,
            // fallback,
            layout,
            sub_parts: children.unwrap_or_default(),
            will_pass,
            span: input.span(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::zwang_router::TEST_STR;

    use super::*;
    use syn::parse_str;

    #[test]
    fn test_parse_routes_with_fallback() {
        let parsed: Part = parse_str(TEST_STR).expect("Unable to parse routes");
        println!("{parsed:#?}");
    }
}

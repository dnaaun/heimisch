use leptos::prelude::*;
use leptos_router::{components::Router, hooks::use_location};
use shared::types::repository::RepositoryId;
use std::{fmt::Display, ops::Deref, str::FromStr};

use crate::app::{repository::RepositoryPage, sidebar::Sidebar};

use super::{
    auth::Auth,
    repository::{issues_tab::IssuesTab, pull_requests_tab::PullRequestsTab},
};

trait RouteComponent<PassedFromParent> {
    type ToPassToChild;
    fn render(
        &self,
        passed_from_parent: PassedFromParent,
        child_component: Box<dyn Fn(Self::ToPassToChild) -> AnyView + Send + Sync>,
    ) -> AnyView;
}

/// Will error out if first char is not a slash.
/// split_to_two_at_non_initial_slash("/hi/hello/asdf") == Ok(("/hi", "/hello/asdf")).
/// split_to_two_at_non_initial_slash("hi/hello/asdf") == Err(()).
/// split_to_two_at_non_initial_slash("/hi") == ("/hi", "/").
/// split_to_two_at_non_initial_slash("hi") == Err(())).
/// split_to_two_at_non_initial_slash("/") == ("/", "/")
fn split_path_at_slash(path: &str) -> Result<(&str, &str), String> {
    if !path.starts_with('/') {
        return Err("path doesn't start with /".to_owned());
    }
    let slash_idx = match path[1..].find('/') {
        Some(i) => i + 1,
        None => return Ok((path, "/")),
    };

    Ok((&path[..slash_idx], &path[slash_idx..]))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TopLevel {
    Auth,
    Empty(TopLevelEmpty),
}

impl RouteComponent<()> for TopLevel {
    type ToPassToChild = ();

    fn render(
        &self,
        _passed_from_parent: (),
        _child_component: Box<dyn Fn(Self::ToPassToChild) -> AnyView + Send + Sync>,
    ) -> AnyView {
        match self {
            TopLevel::Auth => view! { <Auth /> }.into_any(),
            TopLevel::Empty(top_level_empty) => top_level_empty
                .render((), Box::new(|_| ().into_any()))
                .into_any(),
        }
    }
}

impl Display for TopLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            TopLevel::Auth => "/auth".to_owned(),
            TopLevel::Empty(child) => child.to_string(),
        })
    }
}

impl FromStr for TopLevel {
    type Err = String;

    /// Assumption: `s` starts with a slash.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(unused_variables)]
        let (head, tail) = split_path_at_slash(s)?;

        match head {
            "/auth" => Ok(Self::Auth),
            _ => Ok(Self::Empty(s.parse()?)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TopLevelEmpty {
    Empty,
    OwnerName(TopLevelEmptyOwnerName),
}

impl RouteComponent<()> for TopLevelEmpty {
    type ToPassToChild = ();

    fn render(
        &self,
        _passed_from_parent: (),
        _child_component: Box<dyn Fn(Self::ToPassToChild) -> AnyView + Send + Sync>,
    ) -> AnyView {
        let this = self.clone();
        let child_component = Box::new(move |_| match this.clone() {
            TopLevelEmpty::Empty => ().into_any(),
            TopLevelEmpty::OwnerName(top_level_empty_owner_name) => {
                let top_level_empty_owner_name_repo_name_child =
                    top_level_empty_owner_name.child.child.clone();
                let child_component = Box::new(move |r_id| {
                    top_level_empty_owner_name_repo_name_child
                        .render(r_id, Box::new(|_| ().into_any()))
                        .into_any()
                });
                top_level_empty_owner_name
                    .render((), child_component)
                    .into_any()
            }
        }) as Box<dyn Fn(()) -> AnyView + Send + Sync>;

        view! { <Sidebar child_component /> }.into_any()
    }
}

impl FromStr for TopLevelEmpty {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (head, tail) = split_path_at_slash(s)?;
        match head {
            "/" => Ok(Self::Empty),
            _ => {
                let captured = head.chars().skip(1).collect();
                Ok(Self::OwnerName(TopLevelEmptyOwnerName {
                    captured,
                    child: tail.parse()?,
                }))
            }
        }
    }
}

impl Display for TopLevelEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            TopLevelEmpty::Empty => "/".to_owned(),
            TopLevelEmpty::OwnerName(TopLevelEmptyOwnerName { captured, child }) => {
                format!("/{}{}", captured, child)
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TopLevelEmptyOwnerName {
    pub captured: String,
    pub child: TopLevelEmptyOwnerNameRepoName,
}

impl Display for TopLevelEmptyOwnerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&format!("/{}{}", self.captured, self.child))
    }
}

impl FromStr for TopLevelEmptyOwnerName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = split_path_at_slash(s)?;

        Ok(Self {
            captured: head.chars().skip(1).collect(),
            child: tail.parse()?,
        })
    }
}

impl RouteComponent<()> for TopLevelEmptyOwnerName {
    type ToPassToChild = RepositoryId;

    fn render(
        &self,
        _passed_from_parent: (),
        child_component: Box<dyn Fn(RepositoryId) -> AnyView + Send + Sync>,
    ) -> AnyView {
        view! { <RepositoryPage path_so_far=self.clone() child_component /> }.into_any()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TopLevelEmptyOwnerNameRepoName {
    pub captured: String,
    pub child: TopLevelEmptyOwnerNameRepoNameChild,
}

impl FromStr for TopLevelEmptyOwnerNameRepoName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = split_path_at_slash(s)?;

        let captured = head.chars().skip(1).collect();
        Ok(Self {
            captured,
            child: tail.parse()?,
        })
    }
}

impl Display for TopLevelEmptyOwnerNameRepoName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&format!("/{}{}", self.captured, self.child))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TopLevelEmptyOwnerNameRepoNameChild {
    Issues,
    Pulls,
}

impl FromStr for TopLevelEmptyOwnerNameRepoNameChild {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "/issues" => Self::Issues,
            "/pulls" => Self::Pulls,
            _ => return Err("expected one of issues or pulls".to_owned()),
        })
    }
}

impl Display for TopLevelEmptyOwnerNameRepoNameChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&match self {
            TopLevelEmptyOwnerNameRepoNameChild::Issues => "/issues".to_owned(),
            TopLevelEmptyOwnerNameRepoNameChild::Pulls => "/pulls".to_owned(),
        })
    }
}

impl RouteComponent<RepositoryId> for TopLevelEmptyOwnerNameRepoNameChild {
    type ToPassToChild = ();

    fn render(
        &self,
        repository_id: RepositoryId,
        _child_component: Box<dyn Fn(Self::ToPassToChild) -> AnyView + Send + Sync>,
    ) -> AnyView {
        match self {
            TopLevelEmptyOwnerNameRepoNameChild::Issues => {
                view! { <IssuesTab repository_id /> }.into_any()
            }
            TopLevelEmptyOwnerNameRepoNameChild::Pulls => {
                view! { <PullRequestsTab repository_id /> }.into_any()
            }
        }
    }
}

#[component]
pub fn Routed() -> impl IntoView {
    view! {
        <Router>
            {{
                let pathname = use_location().pathname;
                let top_level = Memo::new(move |_| pathname.read().parse::<TopLevel>());
                Effect::new(move || tracing::info!("{:?}", top_level.get()));
                match top_level.read().deref() {
                    Ok(top_level) => top_level.render((), Box::new(|_| ().into_any())),
                    Err(_) => todo!(),
                }
            }}
        </Router>
    }
}

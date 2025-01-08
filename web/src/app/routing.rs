use leptos::prelude::*;
use leptos_router::hooks::use_location;
use shared::types::repository::RepositoryId;
use std::{fmt::Display, str::FromStr};

use crate::app::{repository::{issues_tab::IssuesTab, pull_requests_tab::PullRequestsTab}, sidebar::Sidebar};

use super::{auth::Auth, not_found::NotFound, repository::RepositoryPage};

/// Will error out if first char is not a slash.
/// split_to_two_at_non_initial_slash("/hi/hello/asdf") == Ok(("/hi", Some("/hello/asdf"))).
/// split_to_two_at_non_initial_slash("hi/hello/asdf") == Err(()).
/// split_to_two_at_non_initial_slash("/hi") == ("/hi", None).
/// split_to_two_at_non_initial_slash("hi") == Err(())).
fn split_path_at_slash(path: &str) -> Result<(&str, Option<&str>), String> {
    if !path.starts_with('/') {
        return Err("path doesn't start with /".to_owned());
    }
    let slash_idx = match path[1..].find('/') {
        Some(i) => i + 1,
        None => return Ok((path, None)),
    };

    Ok((&path[..slash_idx], Some(&path[slash_idx..])))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TopLevel {
    Empty,
    Auth,
    OwnerName(TopLevelOwnerName),
}

impl Display for TopLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            TopLevel::Auth => "/auth".to_owned(),
            TopLevel::OwnerName(top_level_owner_name) => top_level_owner_name.to_string(),
            TopLevel::Empty => "/".to_owned(),
        })
    }
}

impl FromStr for TopLevel {
    type Err = String;

    /// Assumption: `s` starts with a slash.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, _tail) = match split_path_at_slash(s) {
            Ok(p) => p,
            Err(_) => return Ok(Self::Empty),
        };

        match head {
            "/" => Ok(Self::Empty),
            "/auth" => Ok(Self::Auth),
            _ => Ok(Self::OwnerName(s.parse()?)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TopLevelOwnerName {
    pub captured: String,
    pub child: TopLevelOwnerNameRepoName,
}

impl Display for TopLevelOwnerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&format!("/{}{}", self.captured, self.child))
    }
}

impl FromStr for TopLevelOwnerName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = split_path_at_slash(s)?;

        Ok(Self {
            captured: head.chars().skip(1).collect(),
            child: tail.ok_or(format!("after {} : no tail", head))?.parse()?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TopLevelOwnerNameRepoName {
    pub captured: String,
    pub child: TopLevelOwnerNameRepoNameChild,
}

impl FromStr for TopLevelOwnerNameRepoName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = split_path_at_slash(s)?;

        Ok(Self {
            captured: head.to_owned(),
            child: tail.ok_or(format!("after {head} : no tail"))?.parse()?,
        })
    }
}

impl Display for TopLevelOwnerNameRepoName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&format!("/{}{}", self.captured, self.child))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TopLevelOwnerNameRepoNameChild {
    Issues,
    Pulls,
}

impl FromStr for TopLevelOwnerNameRepoNameChild {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "/issues" => Self::Issues,
            "/pulls" => Self::Pulls,
            _ => return Err("expected one of issues or pulls".to_owned()),
        })
    }
}

impl Display for TopLevelOwnerNameRepoNameChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&match self {
            TopLevelOwnerNameRepoNameChild::Issues => "/issues".to_owned(),
            TopLevelOwnerNameRepoNameChild::Pulls => "/pulls".to_owned(),
        })
    }
}

#[component]
pub fn Routed() -> impl IntoView {
    let top_level = Memo::new(|_| use_location().pathname.read().parse::<TopLevel>());
    Effect::new(move || tracing::info!("{:?}", top_level.get()));
    move || {
        match top_level.get() {
        Ok(top_level) => match top_level {
            TopLevel::Empty => {
            let child_component: Box<dyn Fn() -> AnyView + Send + Sync> = Box::new(|| ().into_any());
                view! { 
                    <Sidebar child_component />
                }.into_any()
            },
            TopLevel::Auth => view! { <Auth /> }.into_any(),
            TopLevel::OwnerName(TopLevelOwnerName {
                captured: owner_name,
                child:
                    TopLevelOwnerNameRepoName {
                        captured: repo_name,
                        child,
                    },
            }) => {
                let child_component: Box<dyn Fn(RepositoryId) -> AnyView + Send + Sync> = match &child
                {
                    TopLevelOwnerNameRepoNameChild::Issues => {
                        Box::new(|repository_id| view! { <IssuesTab repository_id /> }.into_any())
                    }
                    TopLevelOwnerNameRepoNameChild::Pulls => {
                        Box::new(|repository_id| view! { <PullRequestsTab repository_id /> }.into_any())
                    }
                };
                view! { <RepositoryPage owner_name repo_name active_tab=child child_component=child_component /> }
            }
            .into_any(),
        },
        Err(_) => view! { <NotFound /> }.into_any(),
    }
    }
}

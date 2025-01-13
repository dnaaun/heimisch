use crate::app::{
    not_found::NotFound,
    repository::{
        issues_tab::{list::IssuesList, one_issue::OneIssue, IssuesTab},
        RepositoryPage,
    },
    sidebar::Sidebar,
};

use super::{
    super::{auth::Auth, repository::pull_requests_tab::PullRequestsTab},
    use_pathname, MemoExt, RoutableComponent, RouteToView, RoutingInfoForComponent,
};

use super::slashed_and_segmented::{split_slashed, Slashed};
use leptos::prelude::*;
use shared::types::repository::RepositoryId;
use std::{fmt::Display, sync::Arc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NoPart;

impl<'a> TryFrom<Slashed<'a>> for NoPart {
    type Error = String;

    fn try_from(value: Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        if value.non_slash_len() == 0 {
            return Ok(Self);
        } else {
            return Err(format!("non slash length is not 0 in '{value}'"));
        }
    }
}

// Ensure `NoPart` has a `Display` implementation if it is a custom type
impl std::fmt::Display for NoPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement according to its actual structure and desired string representation.
        write!(f, "")
    }
}

fn empty_component<I>(_i: I) -> impl IntoView {
    ().into_any()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part1 {
    Auth,
    Empty,
    OwnerName {
        owner_name: String,
        child: Part1OwnerNamePart2,
    },
}

impl Display for Part1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part1::Auth => write!(f, "/auth"),
            Part1::Empty => write!(f, "/"),
            Part1::OwnerName { owner_name, child } => {
                write!(f, "/{}{}", owner_name, child)
            }
        }
    }
}

impl<'a> TryFrom<Slashed<'a>> for Part1 {
    type Error = String;

    fn try_from(value: Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = split_slashed(value);

        Ok(match head.non_slash() {
            "auth" => {
                NoPart::try_from(tail)?;
                Self::Auth
            }
            "" => {
                NoPart::try_from(tail)?;
                Self::Empty
            }
            owner_name @ _ => Self::OwnerName {
                owner_name: owner_name.to_owned(),
                child: tail.try_into()?,
            },
        })
    }
}

impl Part1 {
    fn get_only(&self) -> Part1Only {
        match self {
            Part1::Auth => Part1Only::Auth,
            Part1::Empty => Part1Only::Empty,
            Part1::OwnerName { .. } => Part1Only::OwnerName,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Part1Only {
    Auth,
    Empty,
    OwnerName,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParamsOwnerName {
    pub owner_name: Memo<String>,
}

impl RouteToView for Memo<Result<Part1, String>> {
    type PrevParams = ();

    type ArgFromParent = ();

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl IntoView {
        let ok_memo = Memo::new(move |_| self.get().ok());
        let this_part_only = Memo::new(move |_| self.get().map(|i| i.get_only()));
        move || {
            let _ = this_part_only.get(); // Very weirdly, if I onlt don't .track(), this sometimes,
                                          // doesn't work.
            match self.get_untracked() {
                Ok(_) => Memo::new(move |_| ok_memo.get().unwrap())
                    .render(arg_from_parent, prev_params)
                    .into_any(),
                Err(_) => view! { <NotFound /> }.into_any(),
            }
        }
    }
}

impl RouteToView for Memo<Part1> {
    type PrevParams = ();
    type ArgFromParent = ();

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl IntoView {
        let params_owner_name = Memo::new(move |_| match self.get() {
            Part1::OwnerName { owner_name, .. } => Some(owner_name),
            _ => None,
        });
        let part1_owner_name_child_memo = Memo::new(move |_| match self.get() {
            Part1::OwnerName { child, .. } => Some(child),
            _ => None,
        });

        let this_part_only = Memo::new(move |_| self.get().get_only());

        move || {
            let _ = this_part_only.get(); // Very weirdly, if I .track(), this sometimes doesn't
                                          // work.
            match self.get_untracked() {
                Part1::Auth => {
                    let params = prev_params;
                    let outlet: Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        Arc::new(empty_component);
                    let info = RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };
                    Auth.into_view_with_route_info(info).into_any()
                }
                Part1::Empty => {
                    let outlet: Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        Arc::new(empty_component);
                    let params = prev_params;
                    let info = RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    Sidebar.into_view_with_route_info(info).into_any()
                }
                Part1::OwnerName { .. } => {
                    let params = ParamsOwnerName {
                        owner_name: params_owner_name.unwrap(),
                    };
                    let child_memo = part1_owner_name_child_memo.unwrap();
                    let outlet: Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        Arc::new(move |arg_from_parent| child_memo.render(arg_from_parent, params));
                    let info = RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    Sidebar.into_view_with_route_info(info).into_any()
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part1OwnerNamePart2 {
    RepoName {
        repo_name: String,
        child: Part1OwnerNamePart2RepoNamePart3,
    },
}

impl Display for Part1OwnerNamePart2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part1OwnerNamePart2::RepoName { repo_name, child } => {
                write!(f, "/{}{}", repo_name, child)
            }
        }
    }
}

impl<'a> TryFrom<Slashed<'a>> for Part1OwnerNamePart2 {
    type Error = String;

    fn try_from(value: Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = split_slashed(value);

        Ok(match head.non_slash() {
            repo_name @ _ => Self::RepoName {
                repo_name: repo_name.to_owned(),
                child: tail.try_into()?,
            },
        })
    }
}

impl Part1OwnerNamePart2 {
    fn get_only(&self) -> Part1OwnerNamePart2Only {
        match self {
            Part1OwnerNamePart2::RepoName { .. } => Part1OwnerNamePart2Only::RepoName,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Part1OwnerNamePart2Only {
    RepoName,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParamsOwnerNameRepoName {
    pub owner_name: Memo<String>,
    pub repo_name: Memo<String>,
}

impl RouteToView for Memo<Part1OwnerNamePart2> {
    type PrevParams = ParamsOwnerName;
    type ArgFromParent = ();

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl IntoView {
        let params_repo_name = Memo::new(move |_| match self.get() {
            Part1OwnerNamePart2::RepoName { repo_name, .. } => Some(repo_name),
        });
        let part1_owner_name_part2_repo_child_memo = Memo::new(move |_| match self.get() {
            Part1OwnerNamePart2::RepoName { child, .. } => Some(child),
        });

        let this_part_only = Memo::new(move |_| self.get().get_only());

        move || {
            let _ = this_part_only.get(); // Very weirdly, if I onlt don't .track(), this sometimes,
                                          // doesn't work.
            match self.get_untracked() {
                Part1OwnerNamePart2::RepoName { .. } => {
                    let params = ParamsOwnerNameRepoName {
                        owner_name: prev_params.owner_name,
                        repo_name: params_repo_name.unwrap(),
                    };
                    let child_memo = part1_owner_name_part2_repo_child_memo.unwrap();
                    let outlet: Arc<dyn Fn(Signal<RepositoryId>) -> _ + Send + Sync + 'static> =
                        Arc::new(move |arg_from_parent| child_memo.render(arg_from_parent, params));
                    let info = RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    RepositoryPage.into_view_with_route_info(info).into_any()
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part1OwnerNamePart2RepoNamePart3 {
    Empty,
    Pulls,
    Issues(Part1OwnerNamePart2RepoNamePart3Issues),
}

impl Display for Part1OwnerNamePart2RepoNamePart3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part1OwnerNamePart2RepoNamePart3::Empty => write!(f, "/"),
            Part1OwnerNamePart2RepoNamePart3::Pulls => write!(f, "/pulls"),
            Part1OwnerNamePart2RepoNamePart3::Issues(issues_part) => {
                write!(f, "/issues{}", issues_part)
            }
        }
    }
}

impl<'a> TryFrom<Slashed<'a>> for Part1OwnerNamePart2RepoNamePart3 {
    type Error = String;

    fn try_from(value: Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = split_slashed(value);

        match head.non_slash() {
            "" => Ok(Self::Empty),
            "pulls" => Ok(Self::Pulls),
            "issues" => Ok(Self::Issues(tail.try_into()?)),
            other => Err(format!("Unrecognized path segment: '{}'", other)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Part1OwnerNamePart2RepoNamePart3Only {
    Empty,
    Pulls,
    Issues,
}

impl Part1OwnerNamePart2RepoNamePart3 {
    fn get_only(&self) -> Part1OwnerNamePart2RepoNamePart3Only {
        match self {
            Part1OwnerNamePart2RepoNamePart3::Empty => Part1OwnerNamePart2RepoNamePart3Only::Empty,
            Part1OwnerNamePart2RepoNamePart3::Pulls => Part1OwnerNamePart2RepoNamePart3Only::Pulls,
            Part1OwnerNamePart2RepoNamePart3::Issues(..) => {
                Part1OwnerNamePart2RepoNamePart3Only::Issues
            }
        }
    }
}

impl RouteToView for Memo<Part1OwnerNamePart2RepoNamePart3> {
    type PrevParams = ParamsOwnerNameRepoName;

    type ArgFromParent = Signal<RepositoryId>;

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl IntoView {
        let part1_owner_name_part2_repo_name_part3_issues_child_memo =
            Memo::new(move |_| match self.get() {
                Part1OwnerNamePart2RepoNamePart3::Issues(child) => Some(child),
                _ => None,
            });

        let this_part_only = Memo::new(move |_| self.get().get_only());

        move || {
            let _ = this_part_only.get(); // Very weirdly, if I don't .track(), this sometimes,
                                          // doesn't work.
            match self.get_untracked() {
                Part1OwnerNamePart2RepoNamePart3::Empty => {
                    let params = prev_params;
                    let outlet: Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        Arc::new(empty_component);

                    let info = RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };
                    IssuesList.into_view_with_route_info(info).into_any()
                }
                Part1OwnerNamePart2RepoNamePart3::Pulls => {
                    let params = prev_params;
                    let outlet: Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        Arc::new(empty_component);

                    let info = RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };
                    PullRequestsTab.into_view_with_route_info(info).into_any()
                }
                Part1OwnerNamePart2RepoNamePart3::Issues(_) => {
                    let child_memo =
                        part1_owner_name_part2_repo_name_part3_issues_child_memo.unwrap();

                    let outlet: Arc<dyn Fn(Signal<RepositoryId>) -> _ + Send + Sync + 'static> =
                        Arc::new(move |arg_from_parent| {
                            child_memo.render(arg_from_parent, prev_params)
                        });
                    let params = prev_params;

                    let info = RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    IssuesTab.into_view_with_route_info(info).into_any()
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part1OwnerNamePart2RepoNamePart3Issues {
    Empty,
    IssueNumber { issue_number: String },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParamsOwnerNameRepoNameIssueNumber {
    pub owner_name: Memo<String>,
    pub repo_name: Memo<String>,
    pub issue_number: Memo<String>,
}

impl std::fmt::Display for Part1OwnerNamePart2RepoNamePart3Issues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part1OwnerNamePart2RepoNamePart3Issues::Empty => write!(f, ""),
            Part1OwnerNamePart2RepoNamePart3Issues::IssueNumber { issue_number } => {
                write!(f, "/{}", issue_number)
            }
        }
    }
}

impl<'a> TryFrom<Slashed<'a>> for Part1OwnerNamePart2RepoNamePart3Issues {
    type Error = String;

    fn try_from(value: Slashed) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = split_slashed(value);
        Ok(match head.non_slash() {
            "" => Self::Empty,
            issue_number @ _ => {
                NoPart::try_from(tail)?;
                Self::IssueNumber {
                    issue_number: issue_number.to_owned(),
                }
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Part1OwnerNamePart2RepoNamePart3IssuesOnly {
    Empty,
    IssueNumber,
}

impl Part1OwnerNamePart2RepoNamePart3Issues {
    fn get_only(&self) -> Part1OwnerNamePart2RepoNamePart3IssuesOnly {
        match self {
            Part1OwnerNamePart2RepoNamePart3Issues::Empty => {
                Part1OwnerNamePart2RepoNamePart3IssuesOnly::Empty
            }
            Part1OwnerNamePart2RepoNamePart3Issues::IssueNumber { .. } => {
                Part1OwnerNamePart2RepoNamePart3IssuesOnly::IssueNumber
            }
        }
    }
}

impl RouteToView for Memo<Part1OwnerNamePart2RepoNamePart3Issues> {
    type PrevParams = ParamsOwnerNameRepoName;

    type ArgFromParent = Signal<RepositoryId>;

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl IntoView {
        let params_issue_number = Memo::new(move |_| match self.get() {
            Part1OwnerNamePart2RepoNamePart3Issues::IssueNumber { issue_number, .. } => {
                Some(issue_number)
            }
            _ => None,
        });

        let this_part_only = Memo::new(move |_| self.get().get_only());

        move || {
            let _ = this_part_only.get(); // Very weirdly, if I don't .track(), this sometimes,
                                          // doesn't work.
            match *self.read_untracked() {
                Part1OwnerNamePart2RepoNamePart3Issues::Empty => {
                    let params = prev_params;
                    let outlet: Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        Arc::new(empty_component);
                    let info = RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    IssuesList.into_view_with_route_info(info).into_any()
                }
                Part1OwnerNamePart2RepoNamePart3Issues::IssueNumber { .. } => {
                    let params = ParamsOwnerNameRepoNameIssueNumber {
                        owner_name: prev_params.owner_name,
                        repo_name: prev_params.repo_name,
                        issue_number: params_issue_number.unwrap(),
                    };
                    let outlet: Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        Arc::new(empty_component);
                    let info = RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };
                    OneIssue.into_view_with_route_info(info).into_any()
                }
            }
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct ParsedPath<T: Sync + Send + 'static>(pub Memo<Result<T, String>>);

impl<T: Sync + Send + 'static> std::ops::Deref for ParsedPath<T> {
    type Target = Memo<Result<T, String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[component]
pub fn Routed() -> impl IntoView {
    let pathname = use_pathname();
    let part1 = Memo::new(move |_| {
        let pathname = pathname.get();
        let slashed =
            Slashed::new(&pathname).expect("pathname doesn't start with a slash is weird");
        Part1::try_from(slashed)
    });
    provide_context(ParsedPath(part1));
    part1.render((), ())
}

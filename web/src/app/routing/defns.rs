use crate::app::{
    not_found::NotFound,
    repository::{
        issues_tab::{
            list::IssuesListEmpty, one_issue::OneIssue, IssuesTabEmpty, IssuesTabWithIssues,
        },
        RepositoryPage,
    },
    sidebar::{SidebarEmpty, SidebarOwnerName},
};

use super::{
    super::{auth::Auth, repository::pull_requests_tab::PullRequestsTab},
    use_pathname, MemoExt,
};

use super::slashed_and_segmented::{split_slashed, Slashed};
use leptos::prelude::*;
use shared::types::repository::RepositoryId;
use std::fmt::Display;

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

fn empty_component<I>(_i: I) -> AnyView {
    ().into_any()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part1 {
    Auth,
    Empty,
    OwnerName {
        owner_name: String,
        child_parts: Part1OwnerNamePart2,
    },
}

impl Display for Part1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part1::Auth => write!(f, "/auth"),
            Part1::Empty => write!(f, "/"),
            Part1::OwnerName {
                owner_name,
                child_parts,
            } => {
                write!(f, "/{}{}", owner_name, child_parts)
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
                child_parts: tail.try_into()?,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part1AuthCaptures {
    pub prev_captures: Memo<()>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part1EmptyCaptures {
    pub prev_captures: Memo<()>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part1OwnerNameCaptures {
    pub prev_captures: Memo<()>,
    pub owner_name: String,
}

trait RouteToComponent {
    type PrevCaptures: Sync + Send + 'static;
    type ArgFromParent;
    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_captures: Memo<Self::PrevCaptures>,
    ) -> AnyView;
}

impl RouteToComponent for Memo<Result<Part1, String>> {
    type PrevCaptures = ();

    type ArgFromParent = ();

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_captures: Memo<Self::PrevCaptures>,
    ) -> AnyView {
        let ok_memo = Memo::new(move |_| self.get().ok());
        let this_part_only = Memo::new(move |_| self.get().map(|i| i.get_only()));
        (move || {
            let _ = this_part_only.get(); // Very weirdly, if I onlt don't .track(), this sometimes,
                                          // doesn't work.
            match self.get_untracked() {
                Ok(_) => Memo::new(move |_| ok_memo.get().unwrap())
                    .render(arg_from_parent, prev_captures),
                Err(_) => view! { <NotFound /> }.into_any(),
            }
        })
        .into_any()
    }
}

impl RouteToComponent for Memo<Part1> {
    type PrevCaptures = ();
    type ArgFromParent = ();

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_captures: Memo<Self::PrevCaptures>,
    ) -> AnyView {
        let part1_auth_captures = Memo::new(move |_| match self.get() {
            Part1::Auth => {
                let captures = Part1AuthCaptures { prev_captures };
                Some(captures)
            }
            _ => None,
        });
        let part1_empty_captures = Memo::new(move |_| match self.get() {
            Part1::Empty => {
                let captures = Part1EmptyCaptures { prev_captures };
                Some(captures)
            }
            _ => None,
        });
        let part1_owner_name_captures = Memo::new(move |_| match self.get() {
            Part1::OwnerName { owner_name, .. } => {
                let captures = Part1OwnerNameCaptures {
                    prev_captures,
                    owner_name,
                };
                Some(captures)
            }
            _ => None,
        });
        let part1_owner_name_child_parts = Memo::new(move |_| match self.get() {
            Part1::OwnerName { child_parts, .. } => Some(child_parts),
            _ => None,
        });

        let this_part_only = Memo::new(move |_| self.get().get_only());

        (move || {
            let _ = this_part_only.get(); // Very weirdly, if I onlt don't .track(), this sometimes,
                                          // doesn't work.
            Effect::new(move || tracing::info!("redrawing part1"));
            match self.get_untracked() {
                Part1::Auth => {
                    let captures = part1_auth_captures.unwrap();
                    Auth(empty_component, captures, arg_from_parent).into_any()
                }
                Part1::Empty => {
                    let captures = part1_empty_captures.unwrap();
                    SidebarEmpty(empty_component, captures, arg_from_parent).into_any()
                }
                Part1::OwnerName { .. } => {
                    let captures = part1_owner_name_captures.unwrap();
                    let child_parts_memo = part1_owner_name_child_parts.unwrap();
                    let child_component =
                        move |arg_from_parent| child_parts_memo.render(arg_from_parent, captures);
                    SidebarOwnerName(child_component, captures, arg_from_parent).into_any()
                }
            }
        })
        .into_any()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part1OwnerNamePart2 {
    RepoName {
        repo_name: String,
        child_parts: Part1OwnerNamePart2RepoNamePart3,
    },
}

impl Display for Part1OwnerNamePart2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part1OwnerNamePart2::RepoName {
                repo_name,
                child_parts,
            } => {
                write!(f, "/{}{}", repo_name, child_parts)
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
                child_parts: tail.try_into()?,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part1OwnerNamePart2RepoNameCaptures {
    pub prev_captures: Memo<Part1OwnerNameCaptures>,
    pub repo_name: String,
}

impl RouteToComponent for Memo<Part1OwnerNamePart2> {
    type PrevCaptures = Part1OwnerNameCaptures;
    type ArgFromParent = ();

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_captures: Memo<Self::PrevCaptures>,
    ) -> AnyView {
        let part1_owner_name_part2_repo_name_captures = Memo::new(move |_| match self.get() {
            Part1OwnerNamePart2::RepoName { repo_name, .. } => {
                let captures = Part1OwnerNamePart2RepoNameCaptures {
                    prev_captures,
                    repo_name,
                };
                Some(captures)
            }
        });
        let part1_owner_name_part2_repo_child_parts = Memo::new(move |_| match self.get() {
            Part1OwnerNamePart2::RepoName { child_parts, .. } => Some(child_parts),
        });

        let this_part_only = Memo::new(move |_| self.get().get_only());

        (move || {
            let _ = this_part_only.get(); // Very weirdly, if I onlt don't .track(), this sometimes,
                                          // doesn't work.
            match self.get_untracked() {
                Part1OwnerNamePart2::RepoName { .. } => {
                    let captures = part1_owner_name_part2_repo_name_captures.unwrap();
                    let child_parts_memo = part1_owner_name_part2_repo_child_parts.unwrap();
                    let child_component =
                        move |arg_from_parent| child_parts_memo.render(arg_from_parent, captures);
                    RepositoryPage(child_component, captures, arg_from_parent, child_parts_memo)
                }
            }
        })
        .into_any()
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part1OwnerNamePart2RepoNamePart3EmptyCaptures {
    pub prev_captures: Memo<Part1OwnerNamePart2RepoNameCaptures>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part1OwnerNamePart2RepoNamePart3PullsCaptures {
    pub prev_captures: Memo<Part1OwnerNamePart2RepoNameCaptures>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part1OwnerNamePart2RepoNamePart3IssuesCaptures {
    pub prev_captures: Memo<Part1OwnerNamePart2RepoNameCaptures>,
}

impl RouteToComponent for Memo<Part1OwnerNamePart2RepoNamePart3> {
    type PrevCaptures = Part1OwnerNamePart2RepoNameCaptures;

    type ArgFromParent = Signal<RepositoryId>;

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_captures: Memo<Self::PrevCaptures>,
    ) -> AnyView {
        let part1_owner_name_part2_repo_name_part3_empty_captures =
            Memo::new(move |_| match self.get() {
                Part1OwnerNamePart2RepoNamePart3::Empty => {
                    let captures = Part1OwnerNamePart2RepoNamePart3EmptyCaptures { prev_captures };
                    Some(captures)
                }
                _ => None,
            });

        let part1_owner_name_part2_repo_name_part3_pulls_captures =
            Memo::new(move |_| match self.get() {
                Part1OwnerNamePart2RepoNamePart3::Empty => {
                    let captures = Part1OwnerNamePart2RepoNamePart3PullsCaptures { prev_captures };
                    Some(captures)
                }
                _ => None,
            });

        let part1_owner_name_part2_repo_name_part3_issues_captures =
            Memo::new(move |_| match self.get() {
                Part1OwnerNamePart2RepoNamePart3::Issues(_) => {
                    let captures = Part1OwnerNamePart2RepoNamePart3IssuesCaptures { prev_captures };
                    Some(captures)
                }
                _ => None,
            });

        let part1_owner_name_part2_repo_name_part3_issues_child_parts =
            Memo::new(move |_| match self.get() {
                Part1OwnerNamePart2RepoNamePart3::Issues(child_parts) => Some(child_parts),
                _ => None,
            });

        let this_part_only = Memo::new(move |_| self.get().get_only());

        (move || {
            let _ = this_part_only.get(); // Very weirdly, if I onlt don't .track(), this sometimes,
                                          // doesn't work.
            match self.get_untracked() {
                Part1OwnerNamePart2RepoNamePart3::Empty => {
                    let captures = part1_owner_name_part2_repo_name_part3_empty_captures.unwrap();
                    IssuesTabEmpty(empty_component, captures, arg_from_parent).into_any()
                }
                Part1OwnerNamePart2RepoNamePart3::Pulls => {
                    let captures = part1_owner_name_part2_repo_name_part3_pulls_captures.unwrap();
                    PullRequestsTab(empty_component, captures, arg_from_parent).into_any()
                }
                Part1OwnerNamePart2RepoNamePart3::Issues(_) => {
                    let captures = part1_owner_name_part2_repo_name_part3_issues_captures.unwrap();

                    let child_parts_memo =
                        part1_owner_name_part2_repo_name_part3_issues_child_parts.unwrap();

                    let child_component =
                        move |arg_from_parent| child_parts_memo.render(arg_from_parent, captures);
                    IssuesTabWithIssues(child_component, captures, arg_from_parent).into_any()
                }
            }
        })
        .into_any()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part1OwnerNamePart2RepoNamePart3Issues {
    Empty,
    IssueNumber { issue_number: String },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part1OwnerNamePart2RepoNamePart3IssuesEmptyCaptures {
    pub prev_captures: Memo<Part1OwnerNamePart2RepoNamePart3IssuesCaptures>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part1OwnerNamePart2RepoNamePart3IssuesIssueNumberCaptures {
    pub prev_captures: Memo<Part1OwnerNamePart2RepoNamePart3IssuesCaptures>,
    pub issue_number: String,
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

impl RouteToComponent for Memo<Part1OwnerNamePart2RepoNamePart3Issues> {
    type PrevCaptures = Part1OwnerNamePart2RepoNamePart3IssuesCaptures;

    type ArgFromParent = Signal<RepositoryId>;

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_captures: Memo<Self::PrevCaptures>,
    ) -> AnyView {
        let part1_owner_name_part2_repo_name_part3_issues_empty_captures =
            Memo::new(move |_| match self.get() {
                Part1OwnerNamePart2RepoNamePart3Issues::Empty => {
                    Some(Part1OwnerNamePart2RepoNamePart3IssuesEmptyCaptures { prev_captures })
                }
                _ => None,
            });

        let part1_owner_name_part2_repo_name_part3_issues_issue_number_captures =
            Memo::new(move |_| match self.get() {
                Part1OwnerNamePart2RepoNamePart3Issues::IssueNumber { issue_number, .. } => {
                    Some(Part1OwnerNamePart2RepoNamePart3IssuesIssueNumberCaptures {
                        prev_captures,
                        issue_number,
                    })
                }
                _ => None,
            });

        let this_part_only = Memo::new(move |_| self.get().get_only());

        (move || {
            let _ = this_part_only.get(); // Very weirdly, if I onlt don't .track(), this sometimes,
                                          // doesn't work.
            match *self.read_untracked() {
                Part1OwnerNamePart2RepoNamePart3Issues::Empty => {
                    let captures =
                        part1_owner_name_part2_repo_name_part3_issues_empty_captures.unwrap();
                    IssuesListEmpty(empty_component, captures, arg_from_parent).into_any()
                }
                Part1OwnerNamePart2RepoNamePart3Issues::IssueNumber { .. } => {
                    let captures =
                        part1_owner_name_part2_repo_name_part3_issues_issue_number_captures
                            .unwrap();

                    OneIssue(empty_component, captures, arg_from_parent).into_any()
                }
            }
        })
        .into_any()
    }
}

#[component]
pub fn Routed() -> impl IntoView {
    let pathname = use_pathname();
    let part1 = Memo::new(move |_| {
        let pathname = pathname.get();
        let slashed =
            Slashed::new(&pathname).expect("pathname doesn't start with a slash is weird");
        tracing::info!("{:?}", Part1::try_from(slashed.clone()));
        Part1::try_from(slashed)
    });
    part1.render((), Memo::new(|_| ()))
}

use crate::app::{
    repository::{
        issues_tab::{list::IssuesList, one_issue::OneIssue, IssuesTab},
        RepositoryPage,
    },
    sidebar::Sidebar,
};

use super::{not_found::NotFound, repository::pull_requests_tab::PullRequestsTab};
use crate::app::auth::Auth;

use shared::types::repository::RepositoryId;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Root {
    Auth,
    Empty,
    OwnerName {
        owner_name: String,
        child: RootOwnerName,
    },
}

impl std::fmt::Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auth => write!(f, "/auth"),
            Self::Empty => write!(f, "/"),
            Self::OwnerName { owner_name, child } => {
                write!(f, "/{}{}", owner_name, child)
            }
        }
    }
}

impl<'a> TryFrom<::zwang_router::Slashed<'a>> for Root {
    type Error = String;

    fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);

        Ok(match head.non_slash() {
            "auth" => {
                ::zwang_router::NoPart::try_from(tail)?;
                Self::Auth
            }
            "" => {
                ::zwang_router::NoPart::try_from(tail)?;
                Self::Empty
            }
            owner_name @ _ => Self::OwnerName {
                owner_name: owner_name.to_owned(),
                child: tail.try_into()?,
            },
        })
    }
}

impl Root {
    fn get_only(&self) -> RootOnly {
        match self {
            Root::Auth => RootOnly::Auth,
            Root::Empty => RootOnly::Empty,
            Root::OwnerName { .. } => RootOnly::OwnerName,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RootOnly {
    Auth,
    Empty,
    OwnerName,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParamsOwnerName {
    pub owner_name: ::leptos::prelude::Memo<String>,
}

trait RouteToView {
    type PrevParams: Sync + Send + 'static;
    type ArgFromParent;
    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView;
}

impl RouteToView for ::leptos::prelude::Memo<Result<Root, String>> {
    type PrevParams = ();

    type ArgFromParent = ();

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let ok_memo =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).ok());
        let this_part_only = ::leptos::prelude::Memo::new(move |_| {
            ::leptos::prelude::Get::get(&self).map(|i| i.get_only())
        });
        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only); // Very weirdly, if I onlt don't .track(), this sometimes,
                                                                  // doesn't work.
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                Ok(_) => ::leptos::prelude::IntoAny::into_any(
                    ::leptos::prelude::Memo::new(move |_| {
                        ::leptos::prelude::Get::get(&ok_memo).unwrap()
                    })
                    .render(arg_from_parent, prev_params),
                ),
                Err(_) => {
                    ::leptos::prelude::IntoAny::into_any(::leptos::prelude::view! { <NotFound /> })
                }
            }
        }
    }
}

impl RouteToView for ::leptos::prelude::Memo<Root> {
    type PrevParams = ();
    type ArgFromParent = ();

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let params_owner_name =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                Root::OwnerName { owner_name, .. } => Some(owner_name),
                _ => None,
            });
        let root_owner_name_child_memo =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                Root::OwnerName { child, .. } => Some(child),
                _ => None,
            });

        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());

        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only); // Very weirdly, if I .track(), this sometimes doesn't
                                                                  // work.
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                Root::Auth => {
                    let params = prev_params;
                    let outlet: std::sync::Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        std::sync::Arc::new(::zwang_router::empty_component);
                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };
                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(Auth, info),
                    )
                }
                Root::Empty => {
                    let outlet: std::sync::Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        std::sync::Arc::new(::zwang_router::empty_component);
                    let params = prev_params;
                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(Sidebar, info),
                    )
                }
                Root::OwnerName { .. } => {
                    let params = ParamsOwnerName {
                        owner_name: ::zwang_router::MemoExt::unwrap(params_owner_name),
                    };
                    let child_memo = ::zwang_router::MemoExt::unwrap(root_owner_name_child_memo);
                    let outlet: std::sync::Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        std::sync::Arc::new(move |arg_from_parent| {
                            child_memo.render(arg_from_parent, params)
                        });
                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(Sidebar, info),
                    )
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RootOwnerName {
    RepoName {
        repo_name: String,
        child: RootOwnerNameRepoName,
    },
}

impl std::fmt::Display for RootOwnerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RootOwnerName::RepoName { repo_name, child } => {
                write!(f, "/{}{}", repo_name, child)
            }
        }
    }
}

impl<'a> TryFrom<::zwang_router::Slashed<'a>> for RootOwnerName {
    type Error = String;

    fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);

        Ok(match head.non_slash() {
            repo_name @ _ => Self::RepoName {
                repo_name: repo_name.to_owned(),
                child: tail.try_into()?,
            },
        })
    }
}

impl RootOwnerName {
    fn get_only(&self) -> Part1OwnerNamePart2Only {
        match self {
            RootOwnerName::RepoName { .. } => Part1OwnerNamePart2Only::RepoName,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Part1OwnerNamePart2Only {
    RepoName,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParamsOwnerNameRepoName {
    pub owner_name: ::leptos::prelude::Memo<String>,
    pub repo_name: ::leptos::prelude::Memo<String>,
}

impl RouteToView for ::leptos::prelude::Memo<RootOwnerName> {
    type PrevParams = ParamsOwnerName;
    type ArgFromParent = ();

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let params_repo_name =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                RootOwnerName::RepoName { repo_name, .. } => Some(repo_name),
            });
        let root_owner_name_repo_child_memo =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                RootOwnerName::RepoName { child, .. } => Some(child),
            });

        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());

        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only); // Very weirdly, if I .track(), this sometimes doesn't
                                                                  // work.
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                RootOwnerName::RepoName { .. } => {
                    let params = ParamsOwnerNameRepoName {
                        owner_name: prev_params.owner_name,
                        repo_name: ::zwang_router::MemoExt::unwrap(params_repo_name),
                    };
                    let child_memo =
                        ::zwang_router::MemoExt::unwrap(root_owner_name_repo_child_memo);
                    let outlet: std::sync::Arc<
                        dyn Fn(::leptos::prelude::Signal<RepositoryId>) -> _
                            + Send
                            + Sync
                            + 'static,
                    > = std::sync::Arc::new(move |arg_from_parent| {
                        child_memo.render(arg_from_parent, params)
                    });
                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(
                            RepositoryPage,
                            info,
                        ),
                    )
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RootOwnerNameRepoName {
    Empty,
    Pulls,
    Issues(RootOwnerNameRepoNameIssues),
}

impl std::fmt::Display for RootOwnerNameRepoName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "/"),
            Self::Pulls => write!(f, "/pulls"),
            Self::Issues(child) => {
                write!(f, "/issues{}", child)
            }
        }
    }
}

impl<'a> TryFrom<::zwang_router::Slashed<'a>> for RootOwnerNameRepoName {
    type Error = String;

    fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);

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

impl RootOwnerNameRepoName {
    fn get_only(&self) -> Part1OwnerNamePart2RepoNamePart3Only {
        match self {
            Self::Empty => Part1OwnerNamePart2RepoNamePart3Only::Empty,
            Self::Pulls => Part1OwnerNamePart2RepoNamePart3Only::Pulls,
            Self::Issues(..) => Part1OwnerNamePart2RepoNamePart3Only::Issues,
        }
    }
}

impl RouteToView for ::leptos::prelude::Memo<RootOwnerNameRepoName> {
    type PrevParams = ParamsOwnerNameRepoName;

    type ArgFromParent = ::leptos::prelude::Signal<RepositoryId>;

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let root_owner_name_repo_name_issues_child_memo =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                RootOwnerNameRepoName::Issues(child) => Some(child),
                _ => None,
            });

        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());

        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only); // Very weirdly, if I don't .track(), this sometimes,
                                                                  // doesn't work.
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                RootOwnerNameRepoName::Empty => {
                    let params = prev_params;
                    let outlet: std::sync::Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        std::sync::Arc::new(::zwang_router::empty_component);

                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };
                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(
                            IssuesList, info,
                        ),
                    )
                }
                RootOwnerNameRepoName::Pulls => {
                    let params = prev_params;
                    let outlet: std::sync::Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        std::sync::Arc::new(::zwang_router::empty_component);

                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };
                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(
                            PullRequestsTab,
                            info,
                        ),
                    )
                }
                RootOwnerNameRepoName::Issues(_) => {
                    let child_memo = ::zwang_router::MemoExt::unwrap(
                        root_owner_name_repo_name_issues_child_memo,
                    );

                    let outlet: std::sync::Arc<
                        dyn Fn(::leptos::prelude::Signal<RepositoryId>) -> _
                            + Send
                            + Sync
                            + 'static,
                    > = std::sync::Arc::new(move |arg_from_parent| {
                        child_memo.render(arg_from_parent, prev_params)
                    });
                    let params = prev_params;

                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(
                            IssuesTab, info,
                        ),
                    )
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RootOwnerNameRepoNameIssues {
    Empty,
    IssueNumber { issue_number: String },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParamsOwnerNameRepoNameIssueNumber {
    pub owner_name: ::leptos::prelude::Memo<String>,
    pub repo_name: ::leptos::prelude::Memo<String>,
    pub issue_number: ::leptos::prelude::Memo<String>,
}

impl std::fmt::Display for RootOwnerNameRepoNameIssues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RootOwnerNameRepoNameIssues::Empty => write!(f, ""),
            RootOwnerNameRepoNameIssues::IssueNumber { issue_number, .. } => {
                write!(f, "/{}", issue_number)
            }
        }
    }
}

impl<'a> TryFrom<::zwang_router::Slashed<'a>> for RootOwnerNameRepoNameIssues {
    type Error = String;

    fn try_from(value: ::zwang_router::Slashed) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);
        Ok(match head.non_slash() {
            "" => Self::Empty,
            issue_number @ _ => {
                ::zwang_router::NoPart::try_from(tail)?;
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

impl RootOwnerNameRepoNameIssues {
    fn get_only(&self) -> Part1OwnerNamePart2RepoNamePart3IssuesOnly {
        match self {
            RootOwnerNameRepoNameIssues::Empty => Part1OwnerNamePart2RepoNamePart3IssuesOnly::Empty,
            RootOwnerNameRepoNameIssues::IssueNumber { .. } => {
                Part1OwnerNamePart2RepoNamePart3IssuesOnly::IssueNumber
            }
        }
    }
}

impl RouteToView for ::leptos::prelude::Memo<RootOwnerNameRepoNameIssues> {
    type PrevParams = ParamsOwnerNameRepoName;

    type ArgFromParent = ::leptos::prelude::Signal<RepositoryId>;

    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let params_issue_number =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                RootOwnerNameRepoNameIssues::IssueNumber { issue_number, .. } => Some(issue_number),
                _ => None,
            });

        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());

        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only); // Very weirdly, if I don't .track(), this sometimes,
                                                                  // doesn't work.
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                RootOwnerNameRepoNameIssues::Empty => {
                    let params = prev_params;
                    let outlet: std::sync::Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        std::sync::Arc::new(::zwang_router::empty_component);
                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(
                            IssuesList, info,
                        ),
                    )
                }
                RootOwnerNameRepoNameIssues::IssueNumber { .. } => {
                    let params = ParamsOwnerNameRepoNameIssueNumber {
                        owner_name: prev_params.owner_name,
                        repo_name: prev_params.repo_name,
                        issue_number: ::zwang_router::MemoExt::unwrap(params_issue_number),
                    };
                    let outlet: std::sync::Arc<dyn Fn(()) -> _ + Send + Sync + 'static> =
                        std::sync::Arc::new(::zwang_router::empty_component);
                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };
                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(
                            OneIssue, info,
                        ),
                    )
                }
            }
        }
    }
}

#[::leptos::prelude::component]
pub fn Routed() -> impl ::leptos::prelude::IntoView {
    let pathname = ::zwang_router::use_pathname();
    let root = ::leptos::prelude::Memo::new(move |_| {
        let pathname = ::leptos::prelude::Get::get(&pathname);
        let slashed = ::zwang_router::Slashed::new(&pathname)
            .expect("pathname doesn't start with a slash is weird");
        Root::try_from(slashed)
    });
    ::leptos::prelude::provide_context(::zwang_router::ParsedPath(root));
    root.render((), ())
}

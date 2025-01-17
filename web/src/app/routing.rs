use crate::app::{
    repository::{
        issues_tab::{list::IssuesList, one_issue::OneIssue, IssuesTab},
        RepositoryPage,
    },
    sidebar::Sidebar,
};

use super::{not_found::NotFound, repository::pull_requests_tab::PullRequestsTab};
use crate::app::auth::Auth;

use leptos::prelude::Signal;
use shared::types::repository::RepositoryId;

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
            let _ = ::leptos::prelude::Get::get(&this_part_only);
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                Ok(_) => ::leptos::prelude::IntoAny::into_any(
                    ::leptos::prelude::Memo::new(move |_| {
                        ::leptos::prelude::Get::get(&ok_memo).unwrap()
                    })
                    .render(arg_from_parent, prev_params),
                ),
                Err(_) => ::leptos::prelude::IntoAny::into_any(::leptos::prelude::view! {
                    < NotFound / >
                }),
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Root {
    Auth(RootAuth),
    OwnerName {
        owner_name: String,
        child: RootOwnerName,
    },
    Empty,
}
impl<'a> TryFrom<::zwang_router::Slashed<'a>> for Root {
    type Error = String;
    fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);
        match head.non_slash() {
            "auth" => Ok(Self::Auth(tail.try_into()?)),
            "" => {
                ::zwang_router::NoPart::try_from(tail)?;
                Ok(Self::Empty)
            }
            owner_name @ _ => Ok(Self::OwnerName {
                owner_name: owner_name.to_owned(),
                child: tail.try_into()?,
            }),
        }
    }
}
impl std::fmt::Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auth(child) => write!(f, "/auth{}", child),
            Self::Empty => write!(f, "/"),
            Self::OwnerName { owner_name, child } => {
                write!(f, "/{}{}", owner_name, child)
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RootOnly {
    Auth,
    OwnerName,
    Empty,
}
impl Root {
    fn get_only(&self) -> RootOnly {
        match self {
            Self::Auth(..) => RootOnly::Auth,
            Self::OwnerName { .. } => RootOnly::OwnerName,
            Self::Empty => RootOnly::Empty,
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
        let param_memo_owner_name =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                Root::OwnerName { owner_name, .. } => Some(owner_name),
                _ => None,
            });
        let child_memo_auth =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                Root::Auth(child) => Some(child),
                _ => None,
            });
        let child_memo_owner_name =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                Root::OwnerName { child, .. } => Some(child),
                _ => None,
            });
        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());
        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only);
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                Root::Auth(_) => {
                    let child_memo = ::zwang_router::MemoExt::unwrap(child_memo_auth);
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
                    > = ::std::sync::Arc::new(move |arg_from_parent| {
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
                            ::zwang_router::passthrough_component,
                            info,
                        ),
                    )
                }
                Root::Empty => {
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
                    > = std::sync::Arc::new(::zwang_router::empty_component::<Self::ArgFromParent>);
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
                        owner_name: ::zwang_router::MemoExt::unwrap(param_memo_owner_name),
                    };
                    let child_memo = ::zwang_router::MemoExt::unwrap(child_memo_owner_name);
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + core::marker::Send + core::marker::Sync,
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
                            ::zwang_router::passthrough_component,
                            info,
                        ),
                    )
                }
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RootAuth {
    Empty,
}
impl<'a> TryFrom<::zwang_router::Slashed<'a>> for RootAuth {
    type Error = String;
    fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);
        match head.non_slash() {
            "" => {
                ::zwang_router::NoPart::try_from(tail)?;
                Ok(Self::Empty)
            }
            other => Err(format!("Unrecognized path segment: '{}'", other)),
        }
    }
}
impl std::fmt::Display for RootAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "/"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RootAuthOnly {
    Empty,
}
impl RootAuth {
    fn get_only(&self) -> RootAuthOnly {
        match self {
            Self::Empty => RootAuthOnly::Empty,
        }
    }
}
impl RouteToView for ::leptos::prelude::Memo<RootAuth> {
    type PrevParams = ();
    type ArgFromParent = ();
    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());
        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only);
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                RootAuth::Empty => {
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
                    > = std::sync::Arc::new(::zwang_router::empty_component::<Self::ArgFromParent>);
                    let params = prev_params;
                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };
                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(Auth, info),
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
impl<'a> TryFrom<::zwang_router::Slashed<'a>> for RootOwnerName {
    type Error = String;
    fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);
        match head.non_slash() {
            repo_name @ _ => Ok(Self::RepoName {
                repo_name: repo_name.to_owned(),
                child: tail.try_into()?,
            }),
        }
    }
}
impl std::fmt::Display for RootOwnerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepoName { repo_name, child } => write!(f, "/{}{}", repo_name, child),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RootOwnerNameOnly {
    RepoName,
}
impl RootOwnerName {
    fn get_only(&self) -> RootOwnerNameOnly {
        match self {
            Self::RepoName { .. } => RootOwnerNameOnly::RepoName,
        }
    }
}
impl RouteToView for ::leptos::prelude::Memo<RootOwnerName> {
    type PrevParams = ParamsOwnerName;
    type ArgFromParent = ();
    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let param_memo_repo_name =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                RootOwnerName::RepoName { repo_name, .. } => Some(repo_name),
            });
        let child_memo_repo_name =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                RootOwnerName::RepoName { child, .. } => Some(child),
            });
        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());
        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only);
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                RootOwnerName::RepoName { .. } => {
                    let params = ParamsOwnerNameRepoName {
                        owner_name: prev_params.owner_name,
                        repo_name: ::zwang_router::MemoExt::unwrap(param_memo_repo_name),
                    };
                    let child_memo = ::zwang_router::MemoExt::unwrap(child_memo_repo_name);
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + core::marker::Send + core::marker::Sync,
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
    Pulls(RootOwnerNameRepoNamePulls),
    Issues(RootOwnerNameRepoNameIssues),
    Empty,
}
impl<'a> TryFrom<::zwang_router::Slashed<'a>> for RootOwnerNameRepoName {
    type Error = String;
    fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);
        match head.non_slash() {
            "pulls" => Ok(Self::Pulls(tail.try_into()?)),
            "issues" => Ok(Self::Issues(tail.try_into()?)),
            "" => {
                ::zwang_router::NoPart::try_from(tail)?;
                Ok(Self::Empty)
            }
            other => Err(format!("Unrecognized path segment: '{}'", other)),
        }
    }
}
impl std::fmt::Display for RootOwnerNameRepoName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pulls(child) => write!(f, "/pulls{}", child),
            Self::Issues(child) => write!(f, "/issues{}", child),
            Self::Empty => write!(f, "/"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RootOwnerNameRepoNameOnly {
    Pulls,
    Issues,
    Empty,
}
impl RootOwnerNameRepoName {
    fn get_only(&self) -> RootOwnerNameRepoNameOnly {
        match self {
            Self::Pulls(..) => RootOwnerNameRepoNameOnly::Pulls,
            Self::Issues(..) => RootOwnerNameRepoNameOnly::Issues,
            Self::Empty => RootOwnerNameRepoNameOnly::Empty,
        }
    }
}
impl RouteToView for ::leptos::prelude::Memo<RootOwnerNameRepoName> {
    type PrevParams = ParamsOwnerNameRepoName;
    type ArgFromParent = Signal<RepositoryId>;
    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let child_memo_pulls =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                RootOwnerNameRepoName::Pulls(child) => Some(child),
                _ => None,
            });
        let child_memo_issues =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                RootOwnerNameRepoName::Issues(child) => Some(child),
                _ => None,
            });
        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());
        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only);
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                RootOwnerNameRepoName::Pulls(_) => {
                    let child_memo = ::zwang_router::MemoExt::unwrap(child_memo_pulls);
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
                    > = ::std::sync::Arc::new(move |arg_from_parent| {
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
                            ::zwang_router::passthrough_component,
                            info,
                        ),
                    )
                }
                RootOwnerNameRepoName::Issues(_) => {
                    let child_memo = ::zwang_router::MemoExt::unwrap(child_memo_issues);
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
                    > = ::std::sync::Arc::new(move |arg_from_parent| {
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
                            ::zwang_router::passthrough_component,
                            info,
                        ),
                    )
                }
                RootOwnerNameRepoName::Empty => {
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
                    > = std::sync::Arc::new(::zwang_router::empty_component::<Self::ArgFromParent>);
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
pub enum RootOwnerNameRepoNamePulls {
    Empty,
}
impl<'a> TryFrom<::zwang_router::Slashed<'a>> for RootOwnerNameRepoNamePulls {
    type Error = String;
    fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);
        match head.non_slash() {
            "" => {
                ::zwang_router::NoPart::try_from(tail)?;
                Ok(Self::Empty)
            }
            other => Err(format!("Unrecognized path segment: '{}'", other)),
        }
    }
}
impl std::fmt::Display for RootOwnerNameRepoNamePulls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "/"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RootOwnerNameRepoNamePullsOnly {
    Empty,
}
impl RootOwnerNameRepoNamePulls {
    fn get_only(&self) -> RootOwnerNameRepoNamePullsOnly {
        match self {
            Self::Empty => RootOwnerNameRepoNamePullsOnly::Empty,
        }
    }
}
impl RouteToView for ::leptos::prelude::Memo<RootOwnerNameRepoNamePulls> {
    type PrevParams = ParamsOwnerNameRepoName;
    type ArgFromParent = Signal<RepositoryId>;
    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());
        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only);
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                RootOwnerNameRepoNamePulls::Empty => {
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
                    > = std::sync::Arc::new(::zwang_router::empty_component::<Self::ArgFromParent>);
                    let params = prev_params;
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
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RootOwnerNameRepoNameIssues {
    IssueNumber {
        issue_number: String,
        child: RootOwnerNameRepoNameIssuesIssueNumber,
    },
    Empty,
}
impl<'a> TryFrom<::zwang_router::Slashed<'a>> for RootOwnerNameRepoNameIssues {
    type Error = String;
    fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);
        match head.non_slash() {
            "" => {
                ::zwang_router::NoPart::try_from(tail)?;
                Ok(Self::Empty)
            }
            issue_number @ _ => Ok(Self::IssueNumber {
                issue_number: issue_number.to_owned(),
                child: tail.try_into()?,
            }),
        }
    }
}
impl std::fmt::Display for RootOwnerNameRepoNameIssues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "/"),
            Self::IssueNumber {
                issue_number,
                child,
            } => {
                write!(f, "/{}{}", issue_number, child)
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RootOwnerNameRepoNameIssuesOnly {
    IssueNumber,
    Empty,
}
impl RootOwnerNameRepoNameIssues {
    fn get_only(&self) -> RootOwnerNameRepoNameIssuesOnly {
        match self {
            Self::IssueNumber { .. } => RootOwnerNameRepoNameIssuesOnly::IssueNumber,
            Self::Empty => RootOwnerNameRepoNameIssuesOnly::Empty,
        }
    }
}
impl RouteToView for ::leptos::prelude::Memo<RootOwnerNameRepoNameIssues> {
    type PrevParams = ParamsOwnerNameRepoName;
    type ArgFromParent = Signal<RepositoryId>;
    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let param_memo_issue_number =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                RootOwnerNameRepoNameIssues::IssueNumber { issue_number, .. } => Some(issue_number),
                _ => None,
            });
        let child_memo_issue_number =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                RootOwnerNameRepoNameIssues::IssueNumber { child, .. } => Some(child),
                _ => None,
            });
        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());
        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only);
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                RootOwnerNameRepoNameIssues::Empty => {
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
                    > = std::sync::Arc::new(::zwang_router::empty_component::<Self::ArgFromParent>);
                    let params = prev_params;
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
                    let params = ParamsIssueNumberOwnerNameRepoName {
                        owner_name: prev_params.owner_name,
                        repo_name: prev_params.repo_name,
                        issue_number: ::zwang_router::MemoExt::unwrap(param_memo_issue_number),
                    };
                    let child_memo = ::zwang_router::MemoExt::unwrap(child_memo_issue_number);
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + core::marker::Send + core::marker::Sync,
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
                            ::zwang_router::passthrough_component,
                            info,
                        ),
                    )
                }
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RootOwnerNameRepoNameIssuesIssueNumber {
    Empty,
}
impl<'a> TryFrom<::zwang_router::Slashed<'a>> for RootOwnerNameRepoNameIssuesIssueNumber {
    type Error = String;
    fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        let (head, tail) = ::zwang_router::split_slashed(value);
        match head.non_slash() {
            "" => {
                ::zwang_router::NoPart::try_from(tail)?;
                Ok(Self::Empty)
            }
            other => Err(format!("Unrecognized path segment: '{}'", other)),
        }
    }
}
impl std::fmt::Display for RootOwnerNameRepoNameIssuesIssueNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "/"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RootOwnerNameRepoNameIssuesIssueNumberOnly {
    Empty,
}
impl RootOwnerNameRepoNameIssuesIssueNumber {
    fn get_only(&self) -> RootOwnerNameRepoNameIssuesIssueNumberOnly {
        match self {
            Self::Empty => RootOwnerNameRepoNameIssuesIssueNumberOnly::Empty,
        }
    }
}
impl RouteToView for ::leptos::prelude::Memo<RootOwnerNameRepoNameIssuesIssueNumber> {
    type PrevParams = ParamsIssueNumberOwnerNameRepoName;
    type ArgFromParent = Signal<RepositoryId>;
    fn render(
        self,
        arg_from_parent: Self::ArgFromParent,
        prev_params: Self::PrevParams,
    ) -> impl ::leptos::prelude::IntoView {
        let this_part_only =
            ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());
        move || {
            let _ = ::leptos::prelude::Get::get(&this_part_only);
            match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                RootOwnerNameRepoNameIssuesIssueNumber::Empty => {
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
                    > = std::sync::Arc::new(::zwang_router::empty_component::<Self::ArgFromParent>);
                    let params = prev_params;
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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParamsOwnerName {
    pub owner_name: ::leptos::prelude::Memo<String>,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParamsOwnerNameRepoName {
    pub owner_name: ::leptos::prelude::Memo<String>,
    pub repo_name: ::leptos::prelude::Memo<String>,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParamsIssueNumberOwnerNameRepoName {
    pub issue_number: ::leptos::prelude::Memo<String>,
    pub owner_name: ::leptos::prelude::Memo<String>,
    pub repo_name: ::leptos::prelude::Memo<String>,
}

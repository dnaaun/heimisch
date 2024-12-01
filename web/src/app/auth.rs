use leptos_router::hooks::use_query_map;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use shared::{
    endpoints::{
        defns::api::{app_installs::create::{
            CreateAppInstallEndpoint, CreateAppInstallPayload, CreateAppInstallResponse,
        }, auth::finish::GithubAccessToken},
        endpoint_request::{EndpointRequest, OwnApiError},
    },
    types::installation::InstallationId,
};
use std::ops::Deref;

use leptos::{prelude::*, task::spawn_local_scoped};
use shared::endpoints::defns::api::auth::finish::{
    AuthFinishEndpoint, AuthFinishPayload, AuthFinishResponse,
};
use thaw::*;
use wasm_bindgen_futures::JsFuture;

use crate::{
    consts::HEIMISCH_DOMAIN_URL,
    local_storage::{add_installation_id_to_local_storage, local_storage},
};

pub const USER_ACCESS_TOKEN_KEY: &str = "access_token";

#[derive(PartialEq, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum AuthQParams {
    UserAuthQParams(UserAuthQParams),
    AppInstallationQParams(AppInstallationQParams),
}

#[derive(PartialEq, Deserialize, Serialize, Clone)]
pub struct UserAuthQParams {
    code: String,
    state: String,
    show_copy_to_cli: Option<String>,
}

#[derive(PartialEq, Deserialize, Serialize, Clone)]
pub struct AppInstallationQParams {
    /// These are only here because we expect this value when Github does a redirect for
    /// an app installation, and we like to fail fast.
    #[allow(unused)]
    code: String,
    #[allow(unused)]
    setup_action: String,
    installation_id: String,
}

/// use_query() from leptos doesn't support he full power of serde. Specifically, what I want
/// is the ability to sepcify an enum (with an untagged serde (de)serialization flag).
fn use_serde_query_string<T: DeserializeOwned>() -> Result<T, serde::de::value::Error> {
    let query_string = use_query_map().get().to_query_string();
    let query_string = query_string.strip_prefix("?").unwrap_or("");
    serde_urlencoded::from_str(&query_string)
}

#[component]
pub fn Auth() -> impl IntoView {
    let body = match use_serde_query_string() {
        Ok(AuthQParams::UserAuthQParams(params)) => view! { <UserAuth params /> }.into_any(),
        Ok(AuthQParams::AppInstallationQParams(params)) => {
            view! { <AppInstallationAuth params /> }.into_any()
        }
        Err(_) => view! { <div>Not sure how you got here, to be honest.</div> }.into_any(),
    };

    view! {
        <Flex
            vertical=true
            justify=FlexJustify::Center
            align=FlexAlign::Center
            gap=FlexGap::Large
            style="height: 100vh"
        >
            <ToasterProvider>{body}</ToasterProvider>
        </Flex>
    }
}

#[component]
fn AppInstallationAuth(params: AppInstallationQParams) -> impl IntoView {
    let AppInstallationQParams {
        installation_id, ..
    } = params;

    let user_access_token =
        LocalResource::new(|| async { local_storage().get_item(USER_ACCESS_TOKEN_KEY).expect("") });

    let on_click = |_| {
        window()
            .location()
            .set_href("/api/auth/initiate")
            .expect("");
    };

    let body = move || {
        let installation_id = installation_id.clone();
        let user_access_token = user_access_token
            .read()
            .as_ref()
            .map(|u| u.deref().clone())
            .flatten();
        match user_access_token {
            Some(user_access_token) => {
                view! { <AppInstallationAttempt user_access_token installation_id={
                    InstallationId::from(installation_id.parse::<i64>().unwrap())
                } /> }
                .into_any()
            }
            _ => view! {
                <>
                    <div style="font-size: 1.3em">
                        You must first login with your Github account.
                    </div>
                    <Button on_click appearance=ButtonAppearance::Primary>
                        Sign in with Github
                    </Button>
                </>
            }
            .into_any(),
        }
    };

    let fallback = || view! { <Spinner size=SpinnerSize::Huge /> };
    view! {
        <Suspense fallback>
            {body}
        </Suspense>
    }
}

#[component]
pub fn AppInstallationAttempt(
    installation_id: InstallationId,
    user_access_token: String,
) -> impl IntoView {
    let installation_rsrc: LocalResource<Result<CreateAppInstallResponse, OwnApiError>> =
        LocalResource::new(move || {
            let installation_id = installation_id.clone();
            let user_access_token = user_access_token.clone();
            async move {
                let client = Client::new();
                CreateAppInstallEndpoint::make_request(
                    &HEIMISCH_DOMAIN_URL,
                    &client,
                    CreateAppInstallPayload {
                        installation_id,
                        user_access_token,
                    },
                    (),
                )
                .await
            }
        });

    let body = move || {
        installation_rsrc.read().as_ref().map(|installation| {
            match installation.deref() {
                Ok(CreateAppInstallResponse::Success { installation_id }) => {
                    add_installation_id_to_local_storage(*installation_id);
                    view! { <div>Installed app!</div> }.into_any()
                },
                other @ _ => {
                    tracing::error!("{:?}", other);
                    view! { <div>Failed to install Heimisch to Github repo. Please try again.</div> }.into_any()
                },
            }
        })
    };
    let fallback =
        || view! { <Spinner size=SpinnerSize::Huge label="Installing Heimisch on Github Repo"/> };
    view! {
        <Suspense fallback>
            {body}
        </Suspense>
    }
}

#[component]
pub fn UserAuth(params: UserAuthQParams) -> impl IntoView {
    let UserAuthQParams {
        code,
        state,
        show_copy_to_cli,
    } = params;
    let body = {
        let user_access_token_rsrc = LocalResource::new(
            // || (),
            move || {
                let code = code.clone();
                let state = state.clone();
                async move {
                    AuthFinishEndpoint::make_request(
                        &HEIMISCH_DOMAIN_URL,
                        &reqwest::Client::new(),
                        AuthFinishPayload { state, code },
                        (),
                    )
                    .await
                }
            },
        );
        let show_copy_to_cli = show_copy_to_cli.is_some();
        (move || view! {
            <Suspense fallback=LoggingIn>
                {move || {
                    user_access_token_rsrc
                        .read()
                        .as_ref()
                        .map(move |result| {
                            match result.deref() {
                                Ok(AuthFinishResponse::Success { user_access_token }) => {
                                    local_storage()
                                        .set(USER_ACCESS_TOKEN_KEY, user_access_token.as_ref())
                                        .expect("");

                                    view! {
                                        // I'm fairly certain show_copy_to_cli will
                                        // have to depend on the `state` query param
                                        // and not another query param because Github
                                        // won't let me do it otherwise, but who cares
                                        // for now, since I've abandoned the CLI?
                                        <Success
                                            user_access_token=user_access_token.clone()
                                            show_copy_to_cli
                                        />
                                    }
                                        .into_any()
                                }
                                _ => {
                                    view! {
                                        <Text style="font-size: 2em; padding: 0.4em; border-radius: var(--borderRadiusMedium)">
                                            "Authenticating Heimisch failed. Please try again."
                                        </Text>
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>
        })
                        .into_any()
    };

    view! {
        <Flex
            vertical=true
            justify=FlexJustify::Center
            align=FlexAlign::Center
            gap=FlexGap::Large
            style="height: 100vh"
        >
            <ToasterProvider>{body}</ToasterProvider>
        </Flex>
    }
}

#[component]
fn LoggingIn() -> impl IntoView {
    view! { <Spinner size=SpinnerSize::Huge label="Logging you into Heimisch CLI" /> }
}

#[component]
fn Success(user_access_token: GithubAccessToken, show_copy_to_cli: bool) -> impl IntoView {
    let user_access_token2 = user_access_token.clone();
    let toaster = ToasterInjection::expect_context();
    let on_click = move |_| {
        let user_access_token3 = user_access_token2.clone();
        spawn_local_scoped(async move {
            JsFuture::from(
                window()
                    .navigator()
                    .clipboard()
                    .write_text(user_access_token3.as_ref()),
            )
            .await
            .unwrap();
            toast_copied_to_clipboard(toaster);
        })
    };
    let user_access_token: String = user_access_token.into();
    if show_copy_to_cli {
        view! {
            <>
                <div>
                    Copy the below back into your CLI prompt to finish authenticating Heimisch CLI.
                </div>
                <Text
                    tag=TextTag::Code
                    style="font-size: 2em; padding: 0.4em; border-radius: var(--borderRadiusMedium)"
                >
                    {user_access_token}
                </Text>
                <Button
                    icon=icondata::BiClipboardRegular
                    appearance=ButtonAppearance::Primary
                    on_click
                >
                    Copy to clipboard
                </Button>
            </>
        }
        .into_any()
    } else {
        view! { <div style="font-size: 2em">"You're logged in!"</div> }.into_any()
    }
}

pub fn toast_copied_to_clipboard(toaster: ToasterInjection) {
    toaster.dispatch_toast(
        || {
            view! {
                <Toast>
                    <ToastTitle>"Copied."</ToastTitle>
                </Toast>
            }
        },
        ToastOptions::default().with_position(ToastPosition::Top),
    );
}

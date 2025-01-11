use serde::{Deserialize, Serialize};
use shared::{
    endpoints::{
        defns::api::{
            app_installs::create::{
                CreateAppInstallEndpoint, CreateAppInstallPayload, CreateAppInstallResponse,
            },
            auth::finish::GithubAccessToken,
        },
        endpoint_client::OwnApiError,
    },
    types::installation::InstallationId,
};
use std::{collections::HashSet, ops::Deref};

use leptos::{
    prelude::*,
    task::{spawn_local, spawn_local_scoped},
};
use shared::endpoints::defns::api::auth::finish::{
    AuthFinishEndpoint, AuthFinishPayload, AuthFinishResponse,
};
use wasm_bindgen_futures::JsFuture;

use crate::{
    app::{flowbite::Spinner, routing::use_serde_search, sync_engine_provider::use_sync_engine},
    consts::ENDPOINT_CLIENT,
    local_storage::add_installation_ids_to_local_storage,
};

use super::routing::Part1AuthCaptures;

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

#[allow(non_snake_case)]
pub fn Auth(
    #[allow(unused_variables)] child_component: impl Fn(()) -> AnyView + Send + Sync,
    #[allow(unused_variables)] captures: Memo<Part1AuthCaptures>,
    #[allow(unused_variables)] arg_from_parent: (),
) -> impl IntoView {
    let body = match use_serde_search().get() {
        Ok(AuthQParams::UserAuthQParams(params)) => view! { <UserAuth params /> }.into_any(),
        Ok(AuthQParams::AppInstallationQParams(params)) => {
            view! { <AppInstallationAuth params /> }.into_any()
        }
        Err(_) => view! { <div>Not sure how you got here, to be honest.</div> }.into_any(),
    };

    view! { <div class="flex justify-center items-center h-screen">{body}</div> }
}

#[component]
fn AppInstallationAuth(params: AppInstallationQParams) -> impl IntoView {
    let AppInstallationQParams {
        installation_id, ..
    } = params;

    let body = move || {
        let installation_id = installation_id.clone();

        view! {
            <AppInstallationAttempt installation_id=InstallationId::from(
                installation_id.parse::<i64>().unwrap(),
            ) />
        }
    };

    let fallback = || view! { <Spinner /> };
    view! { <Transition fallback>{body}</Transition> }
}

#[component]
pub fn AppInstallationAttempt(installation_id: InstallationId) -> impl IntoView {
    let installation_rsrc: LocalResource<Result<CreateAppInstallResponse, OwnApiError>> =
        LocalResource::new(move || async move {
            ENDPOINT_CLIENT
                .with(|e| e.clone())
                .make_post_request(
                    CreateAppInstallEndpoint,
                    CreateAppInstallPayload { installation_id },
                    (),
                )
                .await
        });

    let body = move || {
        let sync_engine = use_sync_engine().clone();
        let installation_rsrc = installation_rsrc.read().clone();
        installation_rsrc.map(move |installation| match installation.deref().clone() {
            Ok(CreateAppInstallResponse::Success { installation_id }) => {
                add_installation_ids_to_local_storage(&HashSet::from_iter([installation_id]));
                spawn_local(async move {
                    let _ = sync_engine
                        .fetch_repositorys_for_installation_id(&installation_id)
                        .await;
                });
                view! { <div>Installed app!</div> }.into_any()
            }
            other => {
                tracing::error!("{:?}", other);
                view! { <div>Failed to install Heimisch to Github repo. Please try again.</div> }
                    .into_any()
            }
        })
    };
    let fallback = || {
        view! {
            <div class="flex gap-x-3 items-center justify-center">
                <Spinner />
                <div>Installing Heimisch on Github Repo</div>
            </div>
        }
    };
    view! { <Transition fallback>{body}</Transition> }
}

#[component]
pub fn UserAuth(params: UserAuthQParams) -> impl IntoView {
    let UserAuthQParams {
        code,
        state,
        show_copy_to_cli,
    } = params;
    let body = {
        let user_access_token_rsrc = LocalResource::new(move || {
            let code = code.clone();
            let state = state.clone();
            async move {
                ENDPOINT_CLIENT
                    .with(|e| e.clone())
                    .make_post_request(AuthFinishEndpoint, AuthFinishPayload { state, code }, ())
                    .await
            }
        });
        let show_copy_to_cli = show_copy_to_cli.is_some();
        (move || {
            view! {
                <Transition fallback=LoggingIn>
                    {move || {
                        user_access_token_rsrc
                            .read()
                            .as_ref()
                            .map(move |result| {
                                match result.deref() {
                                    Ok(AuthFinishResponse::Success(access_token)) => {
                                        view! {
                                            // I'm fairly certain show_copy_to_cli will
                                            // have to depend on the `state` query param
                                            // and not another query param because Github
                                            // won't let me do it otherwise, but who cares
                                            // for now, since I've abandoned the CLI?
                                            <Success settings=if show_copy_to_cli {
                                                SuccessSettings::ShowCopyToCli(access_token.clone())
                                            } else {
                                                SuccessSettings::NoCopyToCli
                                            } />
                                        }
                                            .into_any()
                                    }
                                    err => {
                                        tracing::error!("{err:?}");
                                        view! {
                                            <div class="text-lg">
                                                "Authenticating Heimisch failed. Please try again."
                                            </div>
                                        }
                                            .into_any()
                                    }
                                }
                            })
                    }}
                </Transition>
            }
        })
        .into_any()
    };

    view! { <div class="flex justify-center items-center h-screen">{body}</div> }
}

#[component]
fn LoggingIn() -> impl IntoView {
    view! {
        <div class="flex gap-3 items-center justify-center">
            <Spinner />
            <div>Logging you into Heimisch CLI</div>
        </div>
    }
}

#[derive(Clone, Debug)]
enum SuccessSettings {
    ShowCopyToCli(GithubAccessToken),
    NoCopyToCli,
}

#[component]
fn Success(settings: SuccessSettings) -> impl IntoView {
    let make_on_click = |access_token: GithubAccessToken| {
        move |_| {
            let access_token = access_token.clone();
            spawn_local_scoped(async move {
                JsFuture::from(window().navigator().clipboard().write_text(&access_token))
                    .await
                    .unwrap();
            })
        }
    };
    match settings {
        SuccessSettings::ShowCopyToCli(github_access_token) => {
            let on_click = make_on_click(github_access_token.clone());
            view! {
                <>
                    <div>
                        Copy the below back into your CLI prompt to finish authenticating Heimisch CLI.
                    </div>
                    <div class="bg-gray-100 p-2 rounded font-mono">
                        {github_access_token.deref().clone()}
                    </div>
                    <button on:click=on_click>Copy to clipboard</button>
                </>
            }
        }
        .into_any(),
        SuccessSettings::NoCopyToCli => {
            view! { <div style="font-size: 2em">"You're logged in!"</div> }.into_any()
        }
    }
}

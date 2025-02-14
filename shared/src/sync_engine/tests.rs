use github_api::models::IssuesCreateRequest;
use std::{cell::RefCell, rc::Rc};
use url::Url;

use crate::{
    endpoints::endpoint_client::EndpointClient,
    github_api_trait::tests::MockGithubApi,
    types::{repository::Repository, user::User},
};

use super::{websocket_updates::transport::tests::MockTransport, SyncEngine};

#[wasm_bindgen_test::wasm_bindgen_test]
async fn testing_optimistic_create() {
    let (mock_transport, mock_transport_handler) = MockTransport::new();
    let mock_transport = Rc::new(RefCell::new(Some(mock_transport)));
    let mock_github_api = MockGithubApi::builder().build();
    let sync_engine = SyncEngine::new(
        EndpointClient::new(|_| (), Url::parse("https://www.example.com/").unwrap()),
        move |_| {
            let mock_transport = mock_transport.clone();
            async move { Ok(mock_transport.borrow_mut().take().unwrap()) }
        },
        mock_github_api,
    )
    .await
    .unwrap();

    let user = User::default();
    let repository = Repository::default();
    let installation_id = repository.installation_id;

    let optimistic_issue_id = sync_engine
        .create_issue(
            &installation_id,
            &user,
            &repository,
            IssuesCreateRequest::default(),
        )
        .unwrap();
}

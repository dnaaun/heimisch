use axum_test::TestServer;
use http::Method;
use shared::endpoints::endpoint::{Endpoint, QueryParams};

pub trait EndpointTestClient: Endpoint {
    async fn make_test_request(
        test_server: &TestServer,
        payload: &Self::JsonPayload,
        query_params: Self::QueryParams,
    ) -> <Self as Endpoint>::JsonResponse {
        let req = test_server
            .method(
                match Self::METHOD {
                    shared::endpoints::endpoint::Method::Post => Method::POST,
                    shared::endpoints::endpoint::Method::Get => Method::GET,
                },
                Self::PATH,
            )
            .save_cookies()
            .json(payload)
            .add_query_params(query_params.get_pairs().collect::<Vec<_>>());

        req.await.json::<Self::JsonResponse>()
    }
}

impl<E: Endpoint> EndpointTestClient for E {}

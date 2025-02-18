// use axum_test::TestServer;
// use shared::endpoints::endpoint::{GetEndpoint, PostEndpoint};

// pub trait PostEndpointTestClient: PostEndpoint {
//     async fn make_test_request(
//         test_server: &TestServer,
//         payload: &Self::JsonPayload,
//         query_params: Self::QueryParams,
//     ) -> <Self as PostEndpoint>::JsonResponse {
//         let req = test_server
//             .post(Self::PATH)
//             .save_cookies()
//             .json(payload)
//             .add_query_params(query_params);

//         req.await.json::<Self::JsonResponse>()
//     }
// }

// impl<E: PostEndpoint> PostEndpointTestClient for E {}

// #[allow(dead_code)]
// pub trait GetEndpointTestClient: GetEndpoint {
//     async fn make_test_request(
//         test_server: &TestServer,
//         query_params: Self::QueryParams,
//     ) -> <Self as GetEndpoint>::JsonResponse {
//         let req = test_server
//             .post(Self::PATH)
//             .save_cookies()
//             .add_query_params(query_params);

//         req.await.json::<Self::JsonResponse>()
//     }
// }

// impl<E: GetEndpoint> GetEndpointTestClient for E {}

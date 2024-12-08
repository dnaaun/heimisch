use super::super::super::super::endpoint::Method;

pub struct AuthInitiateEndpoint;

impl AuthInitiateEndpoint {
    pub const METHOD: Method = Method::Get;
    pub const PATH: &'static str = "/api/auth/initiate";
}

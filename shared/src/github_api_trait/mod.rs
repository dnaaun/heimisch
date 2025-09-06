mod actual_impl;
pub mod trait_defn;

pub use actual_impl::GithubApi;
pub use trait_defn::GithubApiTrait;

#[cfg(test)]
pub use trait_defn::MockGithubApiTrait;

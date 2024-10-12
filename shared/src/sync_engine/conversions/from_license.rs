use crate::avail::Avail;

pub fn from_license(api_license: github_api::models::License) -> crate::types::license::License {
    let github_api::models::License {
        key,
        name,
        node_id,
        spdx_id,
        url,
    } = api_license;
    crate::types::license::License {
        body: Avail::No,
        conditions: Avail::No,
        description: Avail::No,
        featured: Avail::No,
        html_url: Avail::No,
        implementation: Avail::No,
        key: key.into(),
        limitations: Avail::No,
        name: name.into(),
        node_id: node_id.into(),
        permissions: Avail::No,
        spdx_id: spdx_id.into(),
        url: url.into(),
    }
}

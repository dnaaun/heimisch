use std::convert::Infallible;

use crate::{avail::Avail, sync_engine::conversions::ToDb};

impl ToDb for github_api::models::License {
    type DbType = crate::types::license::License;

    type Error = Infallible;

    type OtherChanges = ();

    type Args = ();

    async fn try_to_db_type_and_other_changes(
        self,
        _args: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let github_api::models::License {
            key,
            name,
            node_id,
            spdx_id,
            url,
        } = self;
        Ok((
            crate::types::license::License {
                body: Avail::No,
                conditions: Avail::No,
                description: Avail::No,
                featured: Avail::No,
                html_url: Avail::No,
                implementation: Avail::No,
                key: key.into(),
                limitations: Avail::No,
                name,
                node_id,
                permissions: Avail::No,
                spdx_id: spdx_id.into(),
                url: url.into(),
            },
            (),
        ))
    }
}

use crate::sync_engine::conversions::ToDb;

impl ToDb for github_api::models::Label {
    type Args = ();
    type OtherChanges = ();
    type DbType = crate::types::label::Label;

    type Error = std::convert::Infallible;

    async fn try_to_db_type_and_other_changes(
        self,
        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let github_api::models::Label {
            color,
            default,
            description,
            id,
            name,
            node_id,
            url,
        } = self;
        let id = crate::types::label::LabelId::from(id);
        let label = crate::types::label::Label {
            color,
            default,
            description,
            id,
            name,
            node_id,
            url,
        };

        Ok((label, Default::default()))
    }
}

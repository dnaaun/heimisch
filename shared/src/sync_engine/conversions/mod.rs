use std::convert::Infallible;

/// TODO: Refactor all of the below to use ToDbType
pub mod from_app10;
pub mod from_integration;
pub mod from_issue;
pub mod from_issue_comment;
pub mod from_license;
pub mod from_milestone1;
pub mod from_nullable_simple_user;
pub mod from_repository;
pub mod from_user;
pub mod from_user1;
pub mod from_user2;
pub mod conversion_error;

pub trait ToDb: Sized {
    type DbType;
    type Error;
    type OtherChanges;

    /// The `OtherChanges` shouldn't include the changes from the main type
    /// returned as first item of the tuple.
    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error>;
}

pub trait ToDbNoOtherChanges: ToDb<OtherChanges = ()> {
    /// We define this only for ones where more changes is (), so we don't accidentally forget to
    /// integrate those other changes.
    fn try_to_db_type(self) -> Result<Self::DbType, Self::Error> {
        self.try_to_db_type_and_other_changes()
            .map(|(types, _)| types)
    }
}

impl<T> ToDbNoOtherChanges for T where T: ToDb<OtherChanges = ()> {}

#[allow(unused)]
pub trait InfallibleToDb: ToDb<Error = Infallible> {
    /// The `OtherChanges` shouldn't include the changes from the main type
    /// returned as first item of the tuple.
    fn to_db_type_and_other_changes(self) -> (Self::DbType, Self::OtherChanges) {
        self.try_to_db_type_and_other_changes().unwrap()
    }
}

pub trait InfallibleToDbNoOtherChanges: InfallibleToDb + ToDbNoOtherChanges {
    fn to_db_type(self) -> Self::DbType {
        self.try_to_db_type().unwrap()
    }
}

impl<T: InfallibleToDb + ToDbNoOtherChanges> InfallibleToDbNoOtherChanges for T where
    T: InfallibleToDb + ToDbNoOtherChanges
{
}

impl<T> InfallibleToDb for T where T: ToDb<Error = Infallible> {}

pub mod github_api;
pub mod webhooks;

use std::convert::Infallible;

pub trait ToDb: Sized {
    type DbType;
    type Error;
    type OtherChanges;
    type Args;

    /// The `OtherChanges` shouldn't include the changes from the main type
    /// returned as first item of the tuple.
    fn try_to_db_type_and_other_changes(
        self,
        args: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error>;
}

pub trait ToDbNoOtherChanges: ToDb<OtherChanges = ()> {
    /// We define this only for ones where more changes is (), so we don't accidentally forget to
    /// integrate those other changes.
    fn try_to_db_type(self, args: Self::Args) -> Result<Self::DbType, Self::Error> {
        self.try_to_db_type_and_other_changes(args)
            .map(|(types, _)| types)
    }
}

impl<T> ToDbNoOtherChanges for T where T: ToDb<OtherChanges = ()> {}

#[allow(unused)]
pub trait InfallibleToDb: ToDb<Error = Infallible> {
    /// The `OtherChanges` shouldn't include the changes from the main type
    /// returned as first item of the tuple.
    fn to_db_type_and_other_changes(self, args: Self::Args) -> (Self::DbType, Self::OtherChanges) {
        self.try_to_db_type_and_other_changes(args).unwrap()
    }
}

pub trait InfallibleToDbNoOtherChanges: InfallibleToDb + ToDbNoOtherChanges {
    fn to_db_type(self, args: Self::Args) -> Self::DbType {
        self.try_to_db_type(args).unwrap()
    }
}

impl<T: InfallibleToDb + ToDbNoOtherChanges> InfallibleToDbNoOtherChanges for T where
    T: InfallibleToDb + ToDbNoOtherChanges
{
}

impl<T> InfallibleToDb for T where T: ToDb<Error = Infallible> {}

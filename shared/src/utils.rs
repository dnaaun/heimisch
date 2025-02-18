pub trait LogErr {
    fn log_err(self) -> Self;
}

impl<T, E: std::fmt::Debug> LogErr for Result<T, E> {
    fn log_err(self) -> Self {
        if let Err(err) = &self {
            tracing::error!("{err:?}");
            panic!("{err:?}");
        }
        self
    }
}

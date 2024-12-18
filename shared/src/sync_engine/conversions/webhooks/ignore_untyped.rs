use github_webhook_body::Untyped;

/// Used to statically guarnatee that we're ignoring only when something is `Untyped` or `()` in the
/// generated types for the webhook.
pub fn ignore_untyped<T: UntypedTrait>(_: T) {}

pub trait UntypedTrait {}

impl UntypedTrait for Untyped {}
impl UntypedTrait for () {}
impl<T: UntypedTrait> UntypedTrait for Option<T> {}
impl<T: UntypedTrait> UntypedTrait for Vec<T> {}
impl<T: UntypedTrait> UntypedTrait for &T {}
impl<T: UntypedTrait> UntypedTrait for (T,) {}
impl<T1, T2> UntypedTrait for (T1, T2)
where
    T1: UntypedTrait,
    T2: UntypedTrait,
{
}
impl<T1, T2, T3> UntypedTrait for (T1, T2, T3)
where
    T1: UntypedTrait,
    T2: UntypedTrait,
    T3: UntypedTrait,
{
}

impl<T1, T2, T3, T4> UntypedTrait for (T1, T2, T3, T4)
where
    T1: UntypedTrait,
    T2: UntypedTrait,
    T3: UntypedTrait,
    T4: UntypedTrait,
{
}

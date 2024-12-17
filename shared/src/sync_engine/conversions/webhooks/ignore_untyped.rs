use github_webhook_body::Untyped;

/// Used to statically guarnatee that we're ignoring only when something is `Untyped` in the
/// generated types for the webhook.
pub fn ignore_untyped<T: UntypedTrait>(_: T) {}

pub trait UntypedTrait {}

impl UntypedTrait for Untyped {}
impl<T: UntypedTrait> UntypedTrait for Option<T> {}
impl<T: UntypedTrait> UntypedTrait for Vec<T> {}
impl<T: UntypedTrait> UntypedTrait for &T {}
impl<T: UntypedTrait> UntypedTrait for (T,) {}
impl<T1: UntypedTrait, T2: UntypedTrait> UntypedTrait for (T1, T2) {}


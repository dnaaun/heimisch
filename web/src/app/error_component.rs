use leptos::prelude::*;

use crate::frontend_error::FrontendError;

#[component]
pub fn ErrorComponent<V>(
    #[prop(into)] children: Box<dyn Fn() -> Result<V, FrontendError> + Send + Sync>,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    move || {
        let child = children();
        match child {
            Ok(v) => v.into_any(),
            Err(err) => {
                tracing::error!("{err:?}");
                view! {
                    <div class="w-max h-max flex items-center justify-center">
                        <div>Some error happened!</div>
                    </div>
                }
            }
            .into_any(),
        };
    }
}

// From https://github.com/thaw-ui/thaw/blob/cc21e2506fdb2c3343b20fbc13b1fc87dcc0638f/thaw/src/icon/mod.rs

use std::sync::Arc;

use leptos::{ev, prelude::*};

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: Signal<icondata_core::Icon>,
    /// The width of the icon (horizontal side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into, default = "1em".into())]
    width: MaybeProp<String>,
    /// The height of the icon (vertical side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into, default = "1em".into())]
    height: MaybeProp<String>,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: Option<Signal<String>>,
    /// Callback when clicking on the icon.
    #[prop(optional, into)]
    on_click: Option<Arc<dyn Fn(ev::MouseEvent) + Send + Sync>>,
) -> impl IntoView {
    move || {
        let icon = icon.get();

        let style = match (style, icon.style) {
            (Some(a), Some(b)) => Some(ArcMemo::new(move |_| format!("{b} {}", a.get())).into()),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b.into()),
            (None, None) => None,
        };
        let on_click = on_click.clone();
        let on_click = move |ev| {
            if let Some(click) = on_click.as_ref() {
                click(ev);
            }
        };

        view! {
            <svg
                class=class
                style=move || if let Some(s) = style.as_ref() { s.get() } else { String::new() }
                x=icon.x
                y=icon.y
                width=move || width.get()
                height=move || height.get()
                viewBox=icon.view_box
                stroke-linecap=icon.stroke_linecap
                stroke-linejoin=icon.stroke_linejoin
                stroke-width=icon.stroke_width
                stroke=icon.stroke
                fill=icon.fill.unwrap_or("currentColor")
                inner_html=icon.data
                on:click=on_click
            ></svg>
        }
    }
}

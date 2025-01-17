use leptos::prelude::*;

/// https://flowbite.com/docs/components/badge/#pills-badge
#[component]
pub fn PillBadge(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] color: PillBadgeColor,
) -> impl IntoView {
    let class = color.get_bg_and_text_classes().join(" ")
        + " text-xs font-medium me-2 px-2.5 py-0.5 rounded-full";
    view! { <span class=class>{children.map(|c| c())}</span> }
}

#[allow(dead_code)]
#[derive(Default)]
pub enum PillBadgeColor {
    #[default]
    Default,
    Dark,
    Red,
    Green,
    Yellow,
    Indigo,
    Purple,
    Pink,
}

impl PillBadgeColor {
    fn get_bg_and_text_classes(&self) -> Vec<&'static str> {
        match self {
            PillBadgeColor::Default => vec!["bg-blue-100", "text-blue-800"],
            PillBadgeColor::Dark => vec!["bg-gray-100", "text-gray-800"],
            PillBadgeColor::Red => vec!["bg-red-100", "text-red-800"],
            PillBadgeColor::Green => vec!["bg-green-100", "text-green-800"],
            PillBadgeColor::Yellow => vec!["bg-yellow-100", "text-yellow-800"],
            PillBadgeColor::Indigo => vec!["bg-indigo-100", "text-indigo-800"],
            PillBadgeColor::Purple => vec!["bg-purple-100", "text-purple-800"],
            PillBadgeColor::Pink => vec!["bg-pink-100", "text-pink-800"],
        }
    }
}

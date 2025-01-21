use leptos::prelude::*;

#[allow(dead_code)]
#[derive(Default)]
pub enum ButtonColor {
    #[default]
    Default,
    Alternative,
    Dark,
    Light,
    Green,
    Red,
    Yellow,
    Purple,
}

impl ButtonColor {
    fn get_classes(&self) -> &'static str {
        match self {
            Self::Default => "text-white bg-blue-700 hover:bg-blue-800 focus:ring-blue-300  dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
            Self::Alternative => "dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
            Self::Dark => "text-white bg-gray-800 hover:bg-gray-900 focus:ring-gray-300  dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700",
            Self::Light => "text-gray-900 bg-white border border-gray-300 focus:ring-4  dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700",
            Self::Green => "text-white bg-green-700 hover:bg-green-800 focus:ring-green-300  dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800",
            Self::Red => "text-white bg-red-700 hover:bg-red-800 focus:ring-red-300  dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900",
            Self::Yellow => "text-white bg-yellow-400 hover:bg-yellow-500 focus:ring-yellow-300  dark:focus:ring-yellow-900",
            Self::Purple => "text-white bg-purple-700 hover:bg-purple-800 focus:ring-purple-300  dark:bg-purple-600 dark:hover:bg-purple-700 dark:focus:ring-purple-900",
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub enum ButtonSize {
    #[default]
    ExtraSmall,
    Small,
    Base,
    Large,
    ExtraLarge,
}

impl ButtonSize {
    fn get_classes(&self) -> &'static str {
        match self {
            Self::ExtraSmall => "px-3 py-2 text-xs",
            Self::Small => "px-3 py-2 text-sm",
            Self::Base => "px-5 py-2.5 text-sm",
            Self::Large => "px-5 py-3 text-base",
            Self::ExtraLarge => "px-6 py-3.5 text-base",
        }
    }
}

/// https://flowbite.com/docs/components/buttons/#default-button
#[component]
pub fn Button(
    #[prop(optional, into)] color: Signal<ButtonColor>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] size: Signal<ButtonSize>,
) -> impl IntoView {
    let class = move || {
        format!(
            "font-medium rounded-lg me-2 mb-2 focus:outline-none {} {}",
            size.read().get_classes(),
            color.read().get_classes()
        )
    };
    view! {
        <button type="button" class=class>
            {children.map(|c| c())}
        </button>
    }
}

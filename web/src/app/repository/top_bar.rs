use leptos::prelude::*;

#[component]
pub fn AppHeaderContextItem(
    text: Signal<String>,
    on_click: Box<dyn Fn()>,
    #[prop(optional)] bold: bool,
) -> impl IntoView {
    view! {
        <a
            on:click=move |_| on_click()
            class="flex items-center min-w-xs leading-6 text-inherit no-underline rounded-md px-1.5 py-1 cursor-pointer hover:bg-gray-200"
            class=("font-semibold", bold)
        >
            <span>{text}</span>
        </a>
    }
}

#[component]
pub fn TopBar(
    #[prop(into)] owner_name: Signal<String>,
    #[prop(into)] repo_name: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="pl-4 pr-4 pt-4 pb-2 bg-gray-50 flex items-center flex-nowrap">
            <div class="w-16"></div>
            <nav>
                <ul class="list-none m-0 p-0 flex items-center">
                    <li class="flex items-center">
                        <AppHeaderContextItem text=owner_name on_click=Box::new(|| ()) />
                        <span>/</span>
                    </li>
                    <li>
                        <AppHeaderContextItem bold=true text=repo_name on_click=Box::new(|| ()) />
                    </li>
                </ul>
            </nav>
        </div>
    }
}

use leptos::SignalUpdate;
use leptos::{component, create_signal, view, IntoView};
use leptos_meta::Title;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn Frontend() -> impl IntoView {
    view! {

        // sets the document title
        <Title text="Heimisch"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
 
/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

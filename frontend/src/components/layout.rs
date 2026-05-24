use leptos::*;
use super::{sidebar::Sidebar, navbar::Navbar};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="layout">
            <Navbar/>
            <div class="layout-content">
                <Sidebar/>
                <main class="main-content">
                    {children()}
                </main>
            </div>
        </div>
    }
}

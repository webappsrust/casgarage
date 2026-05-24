use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-brand">
                <h1 class="navbar-title">"CasGarage"</h1>
            </div>
            <div class="navbar-menu">
                <div class="navbar-end">
                    <div class="navbar-item">
                        <span class="navbar-user">"Admin"</span>
                    </div>
                </div>
            </div>
        </nav>
    }
}

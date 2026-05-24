use leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="page">
            <div class="page-content">
                <h1>"404 - Page Not Found"</h1>
                <p>"The page you are looking for does not exist."</p>
            </div>
        </div>
    }
}

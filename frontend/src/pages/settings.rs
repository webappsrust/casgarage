use leptos::*;

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <div class="page">
            <div class="page-header">
                <h1 class="page-title">"Settings"</h1>
            </div>
            <div class="page-content">
                <p>"Application settings will be displayed here."</p>
            </div>
        </div>
    }
}

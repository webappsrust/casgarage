use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <nav class="sidebar-nav">
                <A href="/" class="sidebar-item" active_class="active">
                    <span class="sidebar-icon">"📊"</span>
                    <span class="sidebar-label">"Dashboard"</span>
                </A>
                <A href="/buckets" class="sidebar-item" active_class="active">
                    <span class="sidebar-icon">"🗂️"</span>
                    <span class="sidebar-label">"Buckets"</span>
                </A>
                <A href="/keys" class="sidebar-item" active_class="active">
                    <span class="sidebar-icon">"🔑"</span>
                    <span class="sidebar-label">"Access Keys"</span>
                </A>
                <A href="/cluster" class="sidebar-item" active_class="active">
                    <span class="sidebar-icon">"🖥️"</span>
                    <span class="sidebar-label">"Cluster"</span>
                </A>
                <A href="/settings" class="sidebar-item" active_class="active">
                    <span class="sidebar-icon">"⚙️"</span>
                    <span class="sidebar-label">"Settings"</span>
                </A>
            </nav>
        </aside>
    }
}

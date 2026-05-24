use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="page">
            <div class="page-header">
                <h1 class="page-title">"Dashboard"</h1>
            </div>
            <div class="page-content">
                <div class="dashboard-grid">
                    <div class="dashboard-card">
                        <h3 class="card-title">"Storage Used"</h3>
                        <p class="card-value">"0 GB"</p>
                    </div>
                    <div class="dashboard-card">
                        <h3 class="card-title">"Total Buckets"</h3>
                        <p class="card-value">"0"</p>
                    </div>
                    <div class="dashboard-card">
                        <h3 class="card-title">"Total Objects"</h3>
                        <p class="card-value">"0"</p>
                    </div>
                    <div class="dashboard-card">
                        <h3 class="card-title">"Cluster Status"</h3>
                        <p class="card-value">"Healthy"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub mod api;
pub mod components;
pub mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use pages::{
    dashboard::Dashboard,
    buckets::Buckets,
    keys::Keys,
    cluster::Cluster,
    settings::Settings,
    not_found::NotFound,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/casgarage-frontend.css"/>
        <Title text="CasGarage"/>
        <Meta name="description" content="S3-compatible object storage"/>

        <Router>
            <main class="app">
                <components::layout::Layout>
                    <Routes>
                        <Route path="/" view=Dashboard/>
                        <Route path="/buckets" view=Buckets/>
                        <Route path="/keys" view=Keys/>
                        <Route path="/cluster" view=Cluster/>
                        <Route path="/settings" view=Settings/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </components::layout::Layout>
            </main>
        </Router>
    }
}

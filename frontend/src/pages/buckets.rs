use leptos::*;
use crate::components::{button::*, modal::Modal};

#[component]
pub fn Buckets() -> impl IntoView {
    let (show_create_modal, set_show_create_modal) = create_signal(false);

    view! {
        <div class="page">
            <div class="page-header">
                <h1 class="page-title">"Buckets"</h1>
                <Button
                    variant=ButtonVariant::Primary
                    on_click=move |_| set_show_create_modal.set(true)
                >
                    "Create Bucket"
                </Button>
            </div>
            <div class="page-content">
                <p>"No buckets found. Create your first bucket to get started."</p>
            </div>

            <Modal
                show=show_create_modal.into()
                on_close=move |_| set_show_create_modal.set(false)
                title=Some("Create New Bucket".to_string())
                auto_close_ms=0
            >
                <p>"Create bucket form will go here"</p>
            </Modal>
        </div>
    }
}

use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn Modal(
    #[prop(into)] show: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional, default = 3000)] auto_close_ms: u32,
    children: Children,
) -> impl IntoView {
    // Auto-close timer
    create_effect(move |_| {
        if show.get() && auto_close_ms > 0 {
            set_timeout(
                move || {
                    on_close.call(());
                },
                std::time::Duration::from_millis(auto_close_ms as u64),
            );
        }
    });

    let handle_backdrop_click = move |e: MouseEvent| {
        if let Some(target) = e.target() {
            if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                if element.class_list().contains("modal-backdrop") {
                    on_close.call(());
                }
            }
        }
    };

    let handle_close = move |_| {
        on_close.call(());
    };

    view! {
        <Show when=move || show.get() fallback=|| view! {}>
            <div class="modal-backdrop" on:click=handle_backdrop_click>
                <div class="modal">
                    <div class="modal-header">
                        {title.as_ref().map(|t| view! {
                            <h3 class="modal-title">{t.clone()}</h3>
                        })}
                        <button class="modal-close" on:click=handle_close>
                            "×"
                        </button>
                    </div>
                    <div class="modal-body">
                        {children()}
                    </div>
                </div>
            </div>
        </Show>
    }
}

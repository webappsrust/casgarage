use leptos::*;
use web_sys::Event;

#[component]
pub fn Checkbox(
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let handle_change = move |ev: Event| {
        let target = ev.target().unwrap();
        let input = target.unchecked_into::<web_sys::HtmlInputElement>();
        on_change.call(input.checked());
    };

    view! {
        <label class="checkbox-container" class:disabled=disabled>
            <input
                type="checkbox"
                class="checkbox-input"
                checked=move || checked.get()
                on:change=handle_change
                disabled=disabled
            />
            <span class="checkbox-checkmark"></span>
            {label.as_ref().map(|l| view! {
                <span class="checkbox-label">{l.clone()}</span>
            })}
        </label>
    }
}

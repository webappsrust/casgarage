use leptos::*;
use web_sys::Event;

#[component]
pub fn Input(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] input_type: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] label: Option<String>,
) -> impl IntoView {
    let handle_input = move |ev: Event| {
        let target = ev.target().unwrap();
        let input = target.unchecked_into::<web_sys::HtmlInputElement>();
        on_input.call(input.value());
    };

    view! {
        <div class="input-group">
            {label.as_ref().map(|l| view! {
                <label class="input-label">{l.clone()}</label>
            })}
            <input
                class="input-field"
                type=input_type.unwrap_or_else(|| "text".to_string())
                placeholder=placeholder.unwrap_or_default()
                value=move || value.get()
                on:input=handle_input
                disabled=disabled
            />
        </div>
    }
}

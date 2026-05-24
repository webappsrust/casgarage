use leptos::*;

#[component]
pub fn Dropdown(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] options: Vec<(String, String)>, // (value, label)
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    let selected_label = move || {
        options
            .iter()
            .find(|(v, _)| v == &value.get())
            .map(|(_, l)| l.clone())
            .or_else(|| placeholder.clone())
            .unwrap_or_default()
    };

    let toggle = move |_| {
        if !disabled {
            set_is_open.update(|v| *v = !*v);
        }
    };

    let select_option = move |opt_value: String| {
        on_change.call(opt_value);
        set_is_open.set(false);
    };

    view! {
        <div class="dropdown" class:disabled=disabled>
            <button class="dropdown-toggle" on:click=toggle type="button">
                <span class="dropdown-label">{selected_label}</span>
                <span class="dropdown-arrow">"▼"</span>
            </button>
            <Show when=move || is_open.get()>
                <ul class="dropdown-menu">
                    <For
                        each=move || options.clone()
                        key=|(v, _)| v.clone()
                        children=move |(opt_value, opt_label)| {
                            let value_clone = opt_value.clone();
                            view! {
                                <li
                                    class="dropdown-item"
                                    class:selected=move || value.get() == opt_value
                                    on:click=move |_| select_option(value_clone.clone())
                                >
                                    {opt_label}
                                </li>
                            }
                        }
                    />
                </ul>
            </Show>
        </div>
    }
}

use leptos::*;
use web_sys::MouseEvent;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Success,
}

#[component]
pub fn Button(
    #[prop(into)] on_click: Callback<MouseEvent>,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] loading: bool,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Danger => "btn-danger",
        ButtonVariant::Success => "btn-success",
    };

    view! {
        <button
            class=format!("btn {}", variant_class)
            on:click=move |e| {
                if !disabled && !loading {
                    on_click.call(e);
                }
            }
            disabled=disabled || loading
        >
            <Show when=move || loading fallback=move || children()>
                <span class="btn-spinner">"⟳"</span>
            </Show>
        </button>
    }
}

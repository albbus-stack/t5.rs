use dioxus::prelude::*;

pub enum Variant {
    Primary,
    Secondary,
    Neutral,
}

pub fn text_button(
    text: &str,
    onclick: impl FnMut(Event<MouseData>) + 'static,
    class: &str,
    variant: Variant,
) -> Element {
    button(
        rsx!({ text }),
        onclick,
        &["py-3 px-6", class].join(" "),
        variant,
    )
}

pub fn button(
    text: Element,
    onclick: impl FnMut(Event<MouseData>) + 'static,
    class: &str,
    variant: Variant,
) -> Element {
    rsx! {
        button {
            "data-ripple-dark": "true",
            class: [
                "select-none rounded-lg text-center p-1 px-2 align-middle font-sans text-xs font-bold uppercase shadow-md transition-all hover:shadow-lg disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none",
                match variant {
                    Variant::Primary => {
                        "bg-blue-500 shadow-blue-500/20 hover:shadow-blue-500/30 text-white"
                    }
                    Variant::Secondary => {
                        "bg-red-500 shadow-red-500/20 hover:shadow-red-500/30 text-white"
                    }
                    Variant::Neutral => {
                        "bg-gray-100 shadow-gray-200/10 hover:shadow-gray-200/20 text-gray-900"
                    }
                },
                class,
            ]
                .join(" "),
            onclick: onclick,
            {text}
        }
    }
}

pub fn icon(name: &str, class: &str) -> Element {
    rsx! {
        span { class: ["material-icons", class].join(" "), {name} }
    }
}

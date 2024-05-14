use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Variant {
    Primary,
    Secondary,
    Neutral,
}

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    pub text: String,
    pub onclick: EventHandler<MouseEvent>,
    pub class: String,
    pub variant: Variant,
}

#[component]
pub fn TextButton(props: ButtonProps) -> Element {
    rsx! {
        Button {
            text: props.text,
            onclick: props.onclick,
            class: &["py-3 px-6", &props.class].join(" "),
            variant: props.variant
        }
    }
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let onclick = props.onclick;
    rsx! {
        button {
            "data-ripple-dark": "true",
            class: [
                "select-none rounded-lg text-center p-1 px-2 align-middle font-sans text-xs font-bold uppercase shadow-md transition-all hover:shadow-lg disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none",
                match props.variant {
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
                &props.class,
            ]
                .join(" "),
            onclick: move |evt| onclick.clone().call(evt),
            {props.text}
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    pub name: String,
    pub class: String,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    rsx! {
        span { class: ["material-icons", &props.class].join(" "), {props.name.clone()} }
    }
}

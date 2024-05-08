use crate::{get_head, ui, Page};
use dioxus::prelude::*;

pub fn about(mut page_provider: Signal<Page>) -> Element {
    rsx! {
        {get_head()},
        body {
            div { class: "flex flex-col items-center h-screen bg-gray-900 text-white py-20",
                h1 { class: "text-4xl mb-8 font-bold", "About T5 ⚙️" }
                p { class: "text-center px-10",
                    "T5 is a full-stack web framework built with Rust and WebAssembly. It is designed to be simple, fast, and easy to use."
                }
                {ui::text_button("Home Page", move |_| {
                    page_provider.set(Page::Home);
                }, "mt-5", ui::Variant::Neutral)}
            }
        }
    }
}

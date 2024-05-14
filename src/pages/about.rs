use crate::ui::*;
use crate::{get_head, Context, Page};
use dioxus::prelude::*;

#[component]
pub fn AboutPage(mut context: Context) -> Element {
    rsx! {
        {get_head()},
        body {
            div { class: "flex flex-col items-center h-screen bg-gray-900 text-white py-20",
                h1 { class: "text-4xl mb-8 font-bold", "About T5 ⚙️" }
                p { class: "text-center px-10",
                    "An opinionated cross-platform full-stack application template developed with Rust, Cargo Mobile 2, Dioxus, Warp, Diesel, PostgreSQL, Supabase Auth, Bun and TailwindCSS."
                }
                TextButton {
                    text: "Home Page",
                    onclick: move |_| {
                        context.page_provider.set(Page::Home);
                    },
                    class: "mt-5",
                    variant: Variant::Neutral
                }
            }
        }
    }
}

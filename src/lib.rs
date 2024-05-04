use anyhow::Result;
use dioxus::prelude::*;
mod api;
mod ui;

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("rs-mobile"),
    );
}

#[cfg(target_arch = "wasm32")]
fn init_logging() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
}

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
fn init_logging() {
    env_logger::init();
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn _start_app() {
    stop_unwind(|| main().unwrap());
}

#[no_mangle]
#[inline(never)]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub extern "C" fn start_app() {
    #[cfg(target_os = "android")]
    {
        tao::android_binding!(
            com_example,
            rs_mobile,
            WryActivity,
            wry::android_setup,
            _start_app
        );
        wry::android_binding!(com_example, rs_mobile);
    }

    #[cfg(target_os = "ios")]
    _start_app()
}

pub fn main() -> Result<()> {
    init_logging();
    launch(app);
    Ok(())
}

fn get_head() -> Element {
    rsx! {
        style { {include_str!("../assets/out/tailwind.css")} }
        script { "type": "module",
            {include_str!("../node_modules/@material-tailwind/html/scripts/ripple.js")}
        }
    }
}

fn app() -> Element {
    let mut items = use_signal(|| vec![1, 2, 3]);

    rsx! {
        {get_head()},
        body {
            div { class: "flex flex-col items-center h-screen bg-gray-900 text-white py-10 pt-20 relative",
                h1 { class: "text-4xl mb-8 font-bold", "Hello, T5 🚀" }
                div { class: "flex flex-col justify-center items-center gap-4",
                    {ui::text_button("Add item", move|_| {
                            let mut items_mut = items.write();
                            let new_item = items_mut.len() + 1;
                            items_mut.push(new_item);
                    }, "", ui::Variant::Primary)},
                    {ui::text_button("Remove item", move|_| {
                        let mut items_mut = items.write();
                        items_mut.pop();
                    }, "mb-5", ui::Variant::Secondary)},
                    {ui::button(ui::icon("account_circle", "pt-1"), move|_| {}, "mb-5", ui::Variant::Neutral)}
                }
                {posts()},
                div { class: "mt-2 max-h-[300px] overflow-y-auto",
                    for item in items.read().iter() {
                        div { "- {item}" }
                    }
                }
            }
        }
    }
}

fn posts() -> Element {
    let post = use_resource(move || api::get_post(0123456789));

    match &*post.read_unchecked() {
        Some(Ok(post)) => {
            rsx! {
                {format!("id: {} | title: {} | body: {}", post.id, post.title, post.body)}
            }
        }
        Some(Err(err)) => {
            rsx! { "An error occurred while fetching stories {err}" }
        }
        None => {
            rsx! { "Loading items" }
        }
    }
}

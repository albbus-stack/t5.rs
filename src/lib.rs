use anyhow::Result;
use dioxus::dioxus_core;
use dioxus::prelude::*;

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("rs-mobile"),
    );
}

#[cfg(not(target_os = "android"))]
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
        style {
            {include_str!("../assets/out/tailwind.css")}
        }
        script {
            "type": "module",
            {include_str!("../node_modules/@material-tailwind/html/scripts/ripple.js")}
        }
    }
}

fn button(text: &str, onclick: impl FnMut(Event<MouseData>) + 'static) -> Element {
    rsx! {
        button {
            "data-ripple-dark": "true",
            class: "select-none rounded-lg bg-blue-500 py-3 px-6 text-center align-middle font-sans text-xs font-bold uppercase text-white shadow-md shadow-blue-500/20 transition-all hover:shadow-lg hover:shadow-blue-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none",
            onclick: onclick,
            {text}
        }
    }
}

fn app() -> Element {
    let mut items = use_signal(|| vec![1, 2, 3]);

    rsx! {
        {get_head()}
        body {
            div {
                class: "flex flex-col justify-center items-center min-h-screen bg-gray-900 text-white py-10",
                h1 { class: "text-4xl mb-8 font-bold", "Hello, T5 🚀" }
                div {
                    class: "flex flex-col justify-center items-center gap-4",
                    {button("Add item", move|_| {
                            let mut items_mut = items.write();
                            let new_item = items_mut.len() + 1;
                            items_mut.push(new_item);
                    })}
                    {button("Remove item", move|_| {
                        let mut items_mut = items.write();
                        items_mut.pop();
                    })}
                    button {
                        "data-ripple-dark": "true",
                        "type": "button",
                        class: "relative align-middle select-none font-sans font-medium text-center uppercase transition-all disabled:opacity-50 disabled:shadow-none disabled:pointer-events-none w-10 max-w-[40px] h-10 max-h-[40px] rounded-lg text-xs bg-gray-100 text-gray-900 shadow-md shadow-gray-900/10 hover:shadow-lg hover:shadow-gray-900/20 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none",
                        span {
                            class: "material-icons pt-1", "account_circle"
                        }
                    }
                    div {
                        class: "mt-2",
                        for item in items.read().iter() {
                            div { "- {item}" }
                        }
                    }
                }
            }
        }
    }
}

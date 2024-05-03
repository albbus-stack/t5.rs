use anyhow::Result;
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
        head {
            style {
                dangerous_inner_html: include_str!("../assets/out/tailwind.css")
            }
        }
    }
}

fn app() -> Element {
    let mut items = use_signal(|| vec![1, 2, 3]);

    log::debug!("Hello from the app");

    rsx! {
        {get_head()}
        body {
            div {
                class: "flex flex-col justify-center items-center min-h-screen bg-gray-900 text-white py-10",
                h1 { class: "text-4xl mb-8 font-bold", "Hello, T5 🚀" }
                div {
                    class: "flex flex-col justify-center items-center",
                    button {
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg mb-3",
                        onclick: move|_| {
                            println!("Clicked!");
                            let mut items_mut = items.write();
                            let new_item = items_mut.len() + 1;
                            items_mut.push(new_item);
                            println!("Requested update");
                        },
                        "Add item"
                    }
                    for item in items.read().iter() {
                        div { "- {item}" }
                    }
                }
            }
        }
    }
}

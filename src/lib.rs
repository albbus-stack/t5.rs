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

fn app() -> Element {
    let mut items = use_signal(|| vec![1, 2, 3]);

    log::debug!("Hello from the app");

    rsx! {
        style {

        }
        div {
            h1 { "Hello, Mobile" }
            div { margin_left: "auto", margin_right: "auto", width: "200px", padding: "10px", border: "1px solid black",
                button {
                    margin_bottom: "10px",
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

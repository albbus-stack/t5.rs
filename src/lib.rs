#[macro_use]
extern crate dotenvy_macro;

use crate::store::StoreImpl;
use auth::User;
use store::Store;

use anyhow::Result;
use auth::Supabase;
use dioxus::prelude::*;

use pages::about::*;
use pages::home::*;

mod api;
mod auth;
mod store;

mod pages;
mod ui;

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("t5-rs"),
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
            t5rs,
            WryActivity,
            wry::android_setup,
            _start_app
        );
        wry::android_binding!(com_example, t5rs);
    }

    #[cfg(target_os = "ios")]
    _start_app()
}

pub fn main() -> Result<()> {
    init_logging();
    launch(app);
    Ok(())
}

pub fn get_head() -> Element {
    rsx! {
        style { {include_str!("../assets/out/tailwind.css")} }
        script { "type": "module",
            {include_str!("../node_modules/@material-tailwind/html/scripts/ripple.js")}
        }
    }
}

#[derive(Clone)]
pub enum Page {
    Home,
    About,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Context {
    supabase_client: Signal<Supabase>,
    page_provider: Signal<Page>,
    store: Signal<Store>,
}

fn app() -> Element {
    let store: Signal<Store> = use_signal(|| store::new_store("com.example.t5rs"));

    let supabase_client = use_signal(|| {
        if let Ok(user) = store.read().get::<User>("user") {
            let mut client = Supabase::new();
            client.user = Some(user);
            client
        } else {
            Supabase::new()
        }
    });

    let page_provider: Signal<Page> = use_signal(|| Page::Home);
    let page_clone = page_provider.read().clone();

    let context = Context {
        supabase_client,
        page_provider,
        store,
    };

    {
        match page_clone {
            Page::Home => rsx! {
                HomePage { context }
            },
            Page::About => rsx! {
                AboutPage { context }
            },
        }
    }
}

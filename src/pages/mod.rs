use crate::{api, auth::Supabase, get_head, ui, Page};
use dioxus::prelude::*;

pub mod about;

pub fn home(mut supabase_client: Signal<Supabase>, mut page_provider: Signal<Page>) -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let mut auth_output = use_signal(|| "".to_string());

    let sign_in = move |_| {
        spawn(async move {
            let response = supabase_client
                .read()
                .clone()
                .sign_in_password(&email.read().clone(), &password.read().clone())
                .await
                .unwrap();

            let json_response: serde_json::Value = response.json().await.unwrap();
            let access_token: &str = match json_response["access_token"].as_str() {
                Some(token) => token,
                None => {
                    auth_output.set("Invalid credentials".to_string());
                    return;
                }
            };

            let mut client_clone = supabase_client.read().clone();
            client_clone.set_bearer_token(access_token.to_string());
            supabase_client.set(client_clone);

            auth_output.set(format!("Logged in with {}", email.read()));
        });
    };

    let sign_up = move |_| {
        spawn(async move {
            let result = supabase_client
                .read()
                .clone()
                .signup_email_password(&email.read().clone(), &password.read().clone())
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            auth_output.set(result);
        });
    };

    let sign_out = move |_| {
        spawn(async move {
            let result = supabase_client.read().clone().logout().await;

            match result {
                Ok(_) => auth_output.set("".to_string()),
                Err(err) => {
                    auth_output.set(format!("An error occurred while logging out: {}", err))
                }
            }
        });
    };

    rsx! {
        {get_head()},
        body {
            div { class: "flex flex-col items-center h-screen bg-gray-900 text-white py-20 relative overflow-y-auto w-full",
                h1 { class: "text-4xl mb-8 font-bold", "Hello, T5 🚀" }
                div { class: "text-center px-10 flex flex-col justify-center items-center gap-4 mb-5 w-full",
                    input {
                        class: "w-[300px] p-2 px-3 rounded-lg border-none bg-gray-800 text-white",
                        "type": "email",
                        placeholder: "Enter your email",
                        value: "{email}",
                        oninput: move |event| email.set(event.value())
                    }
                    input {
                        class: "w-[300px] p-2 px-3 rounded-lg border-none bg-gray-800 text-white",
                        "type": "password",
                        placeholder: "Enter your password",
                        value: "{password}",
                        oninput: move |event| password.set(event.value())
                    }
                    div { class: "flex gap-2 mt-2",
                        {ui::text_button("Sign in", sign_in, "", ui::Variant::Neutral)},
                        {ui::text_button("Sign up", sign_up, "", ui::Variant::Primary)},
                        {ui::text_button("Logout", sign_out, "", ui::Variant::Secondary)}
                    }
                    span { class: "w-full break-all mt-2", {auth_output.read().clone()} }
                }
                {ui::text_button("About Page", move |_| {
                    page_provider.set(Page::About);
                }, "mb-5", ui::Variant::Neutral)},
                {api_response()}
            }
        }
    }
}

fn api_response() -> Element {
    let post: Resource<std::prelude::v1::Result<common::Post, reqwest::Error>> =
        use_resource(move || api::get_post(1));

    rsx! {
        div { class: "text-center px-10",
            match &*post.read_unchecked() {
                Some(Ok(post)) => {
                    format!("id: {} | title: {} | body: {}", post.id, post.title, post.body)
                }
                Some(Err(err)) => {
                    format!("An error occurred while fetching posts: {}", err)
                }
                None => {"Loading items".to_string()}
            }
        }
    }
}

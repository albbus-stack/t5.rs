use crate::auth::User;
use crate::store::StoreImpl;
use crate::ui::*;
use crate::{api, get_head, Context, Page};
use dioxus::prelude::*;

#[component]
pub fn HomePage(mut context: Context) -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let mut auth_output =
        use_signal(
            || match context.supabase_client.read().clone().user.clone() {
                Some(user) => format!("Logged in with {}", user.email),
                None => "Logged out".to_string(),
            },
        );

    let sign_in = move |_| {
        spawn(async move {
            let response = context
                .supabase_client
                .read()
                .clone()
                .sign_in_password(&email.read().clone(), &password.read().clone())
                .await
                .unwrap();

            let json_response: serde_json::Value = response.json().await.unwrap();
            let access_token = match json_response["access_token"].as_str() {
                Some(token) => token,
                None => {
                    auth_output.set("Invalid credentials".to_string());
                    return;
                }
            };
            let refresh_token = json_response["refresh_token"].as_str().unwrap();

            let email = json_response["user"]["email"].as_str().unwrap();

            let mut client_clone = context.supabase_client.read().clone();
            let user = User {
                bearer_token: access_token.to_string(),
                refresh_token: refresh_token.to_string(),
                email: email.to_string(),
            };
            client_clone.user = Some(user.clone());
            context.supabase_client.set(client_clone);

            context
                .store
                .write()
                .set("user", &user)
                .expect("Failed to store user");

            auth_output.set(format!("Logged in with {}", email));
        });
    };

    let sign_up = move |_| {
        spawn(async move {
            let response = context
                .supabase_client
                .read()
                .clone()
                .signup_email_password(&email.read().clone(), &password.read().clone())
                .await
                .unwrap();

            let json_response: serde_json::Value = response.json().await.unwrap();
            let email: &str = match json_response["email"].as_str() {
                Some(email) => email,
                None => {
                    auth_output.set(format!(
                        "Error signin up: {}",
                        json_response["msg"].as_str().unwrap()
                    ));
                    return;
                }
            };

            auth_output.set(format!(
                "Signed up {}, confirm this email and then sign in!",
                email
            ));
        });
    };

    let sign_out = move |_| {
        spawn(async move {
            let result = context.supabase_client.read().clone().logout().await;

            match result {
                Ok(_) => {
                    let mut client_clone = context.supabase_client.read().clone();
                    client_clone.user = None;
                    context.supabase_client.set(client_clone);

                    context
                        .store
                        .write()
                        .clear()
                        .expect("Failed to clear storage");

                    auth_output.set("Logged out".to_string());
                }
                Err(err) => {
                    auth_output.set(format!("An error occurred while logging out: {}", err));
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
                        TextButton { text: "Sign in", onclick: sign_in, class: "", variant: Variant::Neutral }
                        TextButton { text: "Sign up", onclick: sign_up, class: "", variant: Variant::Primary }
                        TextButton { text: "Logout", onclick: sign_out, class: "", variant: Variant::Secondary }
                    }
                    i { class: "w-full break-all mt-2", {auth_output} }
                }
                {api_response(context)},
                TextButton {
                    text: "About Page",
                    onclick: move |_| {
                        context.page_provider.set(Page::About);
                    },
                    class: "mt-6",
                    variant: Variant::Neutral
                }
            }
        }
    }
}

fn api_response(context: Context) -> Element {
    let post: Resource<Result<common::Post, reqwest::Error>> = use_resource(move || {
        api::get_post(
            context
                .supabase_client
                .read()
                .clone()
                .user
                .unwrap_or(User::default())
                .clone()
                .bearer_token,
            1,
        )
    });

    rsx! {
        div { class: "text-center px-10",
            match &*post.read_unchecked() {
                Some(Ok(post)) => {
                    format!("id: {} | title: {} | body: {}", post.id, post.title, post.body)
                }
                Some(Err(err)) => {
                    format!("An error occurred while fetching posts: {}", err)
                }
                None => {"Loading posts...".to_string()}
            }
        }
    }
}

use crate::api::auth::AuthApi;
use leptos::*;

#[component]
pub fn OAuthCallback() -> impl IntoView {
    let handle_callback = create_action(|_: &()| async move {
        AuthApi::handle_oauth_callback()
            .await
            .map_err(|e| e.as_string().unwrap_or_else(|| "Unknown error".to_string()))
    });

    // Automatically trigger the callback handling
    handle_callback.dispatch(());

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-100">
            <div class="max-w-md w-full space-y-8 p-8 bg-white rounded-lg shadow-md">
                <div class="text-center">
                    <h2 class="text-2xl font-bold mb-4">"Processing Login..."</h2>
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 mx-auto"></div>
                </div>

                {move || handle_callback.value().get().map(|result| match result {
                    Ok(_) => view! {
                        <div class="text-center text-green-600">
                            "Login successful! Redirecting..."
                        </div>
                    }.into_view(),
                    Err(e) => view! {
                        <div class="text-center text-red-600">
                            {e}
                        </div>
                    }.into_view(),
                })}
            </div>
        </div>
    }
}

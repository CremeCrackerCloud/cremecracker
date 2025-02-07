use leptos::*;
use leptos_router::*;
use web_sys::MouseEvent;

use crate::api::auth::AuthApi;

#[component]
pub fn Login() -> impl IntoView {
    let params = use_query_map();
    let error = move || params.with(|p| p.get("error").cloned());

    let github_auth = create_action(|_: &()| async move {
        AuthApi::github_auth()
            .await
            .map_err(|e| e.as_string().unwrap_or_else(|| "Unknown error".to_string()))
    });

    let gitlab_auth = create_action(|_: &()| async move {
        AuthApi::gitlab_auth()
            .await
            .map_err(|e| e.as_string().unwrap_or_else(|| "Unknown error".to_string()))
    });

    let bitbucket_auth = create_action(|_: &()| async move {
        AuthApi::bitbucket_auth()
            .await
            .map_err(|e| e.as_string().unwrap_or_else(|| "Unknown error".to_string()))
    });

    let on_github_click = move |e: MouseEvent| {
        e.prevent_default();
        github_auth.dispatch(());
    };

    let on_gitlab_click = move |e: MouseEvent| {
        e.prevent_default();
        gitlab_auth.dispatch(());
    };

    let on_bitbucket_click = move |e: MouseEvent| {
        e.prevent_default();
        bitbucket_auth.dispatch(());
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-100">
            <div class="max-w-md w-full space-y-6 p-8 bg-white rounded-lg shadow-md">
                <div>
                    <h2 class="text-center text-3xl font-bold text-gray-900">
                        "Sign in to PaaS"
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600">
                        "Choose your preferred authentication method"
                    </p>
                </div>

                {move || error().map(|err| view! {
                    <div class="bg-red-50 border-l-4 border-red-400 p-4" role="alert">
                        <p class="text-sm text-red-700">{err}</p>
                    </div>
                })}

                <div class="space-y-3">
                    <button
                        on:click=on_github_click
                        class="group relative w-full flex items-center justify-center py-2.5 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-gray-800 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled=move || github_auth.pending().get()
                    >
                        <span class="absolute left-0 inset-y-0 flex items-center pl-3">
                            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
                                <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"></path>
                            </svg>
                        </span>
                        <span class="pl-8">
                            {move || if github_auth.pending().get() { "Connecting..." } else { "Continue with GitHub" }}
                        </span>
                    </button>

                    <button
                        on:click=on_gitlab_click
                        class="group relative w-full flex items-center justify-center py-2.5 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-orange-600 hover:bg-orange-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-orange-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled=move || gitlab_auth.pending().get()
                    >
                        <span class="absolute left-0 inset-y-0 flex items-center pl-3">
                            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M21.94 13.11l-1.05-3.22c0-.03-.01-.06-.02-.09l-2.11-6.48a.859.859 0 00-.8-.57c-.36 0-.68.25-.79.58l-2 6.17H8.84L6.83 3.33a.851.851 0 00-.79-.58c-.37 0-.69.25-.8.58L3.13 9.82v.01l-1.05 3.22c-.19.58.01 1.22.52 1.58l9.89 7.19c.17.12.39.12.56 0l9.89-7.19c.51-.37.71-1 .52-1.58"></path>
                            </svg>
                        </span>
                        <span class="pl-8">
                            {move || if gitlab_auth.pending().get() { "Connecting..." } else { "Continue with GitLab" }}
                        </span>
                    </button>

                    <button
                        on:click=on_bitbucket_click
                        class="group relative w-full flex items-center justify-center py-2.5 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled=move || bitbucket_auth.pending().get()
                    >
                        <span class="absolute left-0 inset-y-0 flex items-center pl-3">
                            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M.778 1.213a.768.768 0 00-.768.892l3.263 19.81c.084.5.515.868 1.022.873H19.95a.772.772 0 00.77-.646l3.27-20.03a.768.768 0 00-.768-.891zM14.52 15.53H9.522L8.17 8.466h7.561z"></path>
                            </svg>
                        </span>
                        <span class="pl-8">
                            {move || if bitbucket_auth.pending().get() { "Connecting..." } else { "Continue with Bitbucket" }}
                        </span>
                    </button>
                </div>
            </div>
        </div>
    }
}

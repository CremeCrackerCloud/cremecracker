use crate::api::UserApi;
use leptos::*;
use wasm_bindgen::JsValue;

#[component]
pub fn Dashboard() -> impl IntoView {
    let user_resource = create_resource(
        || (),
        |_| async move {
            match UserApi::get_current_user().await {
                Ok(user) => Ok(user),
                Err(err) => Err(err
                    .as_string()
                    .unwrap_or_else(|| "Unknown error".to_string())),
            }
        },
    );

    view! {
        <div class="min-h-screen bg-gray-50">
            <main class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                {move || user_resource.get().map(|result| match result {
                    Ok(user) => view! {
                        <div>
                            <div class="mb-8">
                                <h1 class="text-3xl font-bold text-gray-900">
                                    "Welcome, " {user.username} "!"
                                </h1>
                                <p class="mt-2 text-sm text-gray-600">
                                    "You are now logged in to CremeCracker PaaS. Start managing your applications and deployments."
                                </p>
                            </div>

                            <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                                <div class="bg-white overflow-hidden shadow rounded-lg divide-y divide-gray-200">
                                    <div class="px-6 py-5">
                                        <div class="flex items-center">
                                            <div class="flex-shrink-0 bg-blue-500 rounded-md p-3">
                                                <svg class="h-6 w-6 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                                                </svg>
                                            </div>
                                            <div class="ml-5">
                                                <h3 class="text-lg font-medium text-gray-900">
                                                    "Applications"
                                                </h3>
                                                <p class="mt-1 text-sm text-gray-500">
                                                    "Deploy and manage your applications with ease."
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="px-6 py-4 bg-gray-50">
                                        <button class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                                            "Create New App"
                                        </button>
                                    </div>
                                </div>

                                <div class="bg-white overflow-hidden shadow rounded-lg divide-y divide-gray-200">
                                    <div class="px-6 py-5">
                                        <div class="flex items-center">
                                            <div class="flex-shrink-0 bg-green-500 rounded-md p-3">
                                                <svg class="h-6 w-6 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                                                </svg>
                                            </div>
                                            <div class="ml-5">
                                                <h3 class="text-lg font-medium text-gray-900">
                                                    "Deployments"
                                                </h3>
                                                <p class="mt-1 text-sm text-gray-500">
                                                    "Monitor and manage your application deployments."
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="px-6 py-4 bg-gray-50">
                                        <button class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500">
                                            "View Deployments"
                                        </button>
                                    </div>
                                </div>

                                <div class="bg-white overflow-hidden shadow rounded-lg divide-y divide-gray-200">
                                    <div class="px-6 py-5">
                                        <div class="flex items-center">
                                            <div class="flex-shrink-0 bg-purple-500 rounded-md p-3">
                                                <svg class="h-6 w-6 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                                </svg>
                                            </div>
                                            <div class="ml-5">
                                                <h3 class="text-lg font-medium text-gray-900">
                                                    "Settings"
                                                </h3>
                                                <p class="mt-1 text-sm text-gray-500">
                                                    "Configure your account and application settings."
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="px-6 py-4 bg-gray-50">
                                        <button class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-purple-600 hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500">
                                            "Open Settings"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_view(),
                    Err(_) => view! {
                        <div class="text-center py-12">
                            <div class="bg-white p-8 rounded-lg shadow-lg max-w-md mx-auto">
                                <svg class="mx-auto h-12 w-12 text-red-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                                </svg>
                                <h2 class="mt-4 text-xl font-semibold text-gray-900">
                                    "Authentication Required"
                                </h2>
                                <p class="mt-2 text-gray-600">
                                    "Please log in to access the dashboard and manage your applications."
                                </p>
                                <a
                                    href="/login"
                                    class="mt-6 inline-block w-full py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                >
                                    "Go to Login"
                                </a>
                            </div>
                        </div>
                    }.into_view()
                })}
            </main>
        </div>
    }
}

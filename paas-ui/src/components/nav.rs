use crate::api::{AuthApi, UserApi};
use leptos::*;
use leptos_router::*;

#[component]
pub fn NavBar() -> impl IntoView {
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

    let user_resource_clone = user_resource.clone();

    let logout = create_action(move |_: &()| {
        let user_resource = user_resource_clone.clone();

        async move {
            match AuthApi::logout().await {
                Ok(_) => {
                    // Força o refresh do recurso do usuário
                    user_resource.refetch();
                    // Navega para a página de login
                    window().location().set_href("/login").unwrap();
                    Ok(())
                }
                Err(err) => Err(err
                    .as_string()
                    .unwrap_or_else(|| "Unknown error".to_string())),
            }
        }
    });

    view! {
        <nav class="bg-white shadow-lg">
            <div class="max-w-7xl mx-auto px-4">
                <div class="flex justify-between h-16">
                    // Logo section
                    <div class="flex items-center">
                        <A href="/" class="text-xl font-bold text-gray-800">
                            "CremeCracker PaaS"
                        </A>
                    </div>

                    // User section
                    <div class="flex items-center space-x-4">
                        {move || user_resource.get().map(|result| match result {
                            Ok(user) => view! {
                                <div class="flex items-center space-x-4">
                                    <div class="flex items-center space-x-2">
                                        <span class="text-sm text-gray-700">
                                            {user.username}
                                        </span>
                                        <button
                                            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                                            on:click=move |_| logout.dispatch(())
                                        >
                                            <svg class="h-4 w-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                                            </svg>
                                            "Logout"
                                        </button>
                                    </div>
                                </div>
                            }.into_view(),
                            Err(_) => view! {
                                <A
                                    href="/login"
                                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                >
                                    <svg class="h-4 w-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1" />
                                    </svg>
                                    "Sign In"
                                </A>
                            }.into_view()
                        })}
                    </div>
                </div>
            </div>
        </nav>
    }
}

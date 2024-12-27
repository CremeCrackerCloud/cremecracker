use leptos::*;
use leptos_router::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="bg-white shadow-lg">
            <div class="max-w-7xl mx-auto px-4">
                <div class="flex justify-between h-16">
                    <div class="flex">
                        <div class="flex-shrink-0 flex items-center">
                            <A href="/" class="text-xl font-bold text-gray-800">
                                "PaaS"
                            </A>
                        </div>
                    </div>
                    <div class="flex items-center">
                        <A
                            href="/login"
                            class="ml-4 px-3 py-2 rounded-md text-sm font-medium text-gray-700 hover:text-gray-900 hover:bg-gray-50"
                        >
                            "Sign In"
                        </A>
                    </div>
                </div>
            </div>
        </nav>
    }
}

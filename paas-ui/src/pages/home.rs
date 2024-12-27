use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100">
            <div class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h1 class="text-4xl font-extrabold text-gray-900 sm:text-5xl sm:tracking-tight lg:text-6xl">
                        "Welcome to PaaS"
                    </h1>
                    <p class="mt-5 max-w-xl mx-auto text-xl text-gray-500">
                        "Your cloud-native platform for modern applications"
                    </p>
                </div>
            </div>
        </div>
    }
}

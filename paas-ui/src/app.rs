use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::nav::NavBar;
use crate::config::ConfigProvider;
use crate::pages::{Home, Login, OAuthCallback, Dashboard};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="PaaS - Platform as a Service"/>

        <ConfigProvider>
            <Router>
                <main>
                    <NavBar />
                    <Routes>
                        <Route path="" view=Home/>
                        <Route path="/login" view=Login/>
                        <Route path="/dashboard" view=Dashboard/>
                        <Route path="/auth/github/callback" view=OAuthCallback/>
                        <Route path="/auth/gitlab/callback" view=OAuthCallback/>
                        <Route path="/auth/bitbucket/callback" view=OAuthCallback/>
                    </Routes>
                </main>
            </Router>
        </ConfigProvider>
    }
}

use leptos::*;

#[component]
pub fn SplashScreen(cx: Scope) -> impl IntoView {
    view! { cx,
        <img src="public/splash.png"/>
    }
}
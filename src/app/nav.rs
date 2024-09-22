use crate::css::ClassName;
use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class=ClassName::NAV>
            <a href="/" class=ClassName::NAV_LINK>
                "Home"
            </a>

            <a href="/guestbook/" class=ClassName::NAV_LINK>
                "Guestbook"
            </a>
        </nav>
    }
}

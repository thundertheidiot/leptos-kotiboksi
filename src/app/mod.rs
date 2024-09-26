use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::css::*;

mod guestbook;
mod home;
mod nav;
mod radio;
use self::guestbook::Guestbook;
use self::home::HomePage;
use self::nav::Navbar;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Style>{STYLE_SHEET}</Style>

        <Title text="kotiboksi"/>

        <div class=ClassName::MAIN_CONTAINER>

            <Router>
                <Navbar/>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/guestbook" view=Guestbook/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </Router>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class=ClassName::TEXTPART style="text-align: center">
            <h1>"404 - Not Found"</h1>

            {move || {
                let breaks = vec![view! { <br/> }; 10];
                breaks.into_view()
            }}

            <p>"Keep looking"</p>
        </div>
    }
}

pub mod app;
pub mod css;
pub mod db;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;

    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}

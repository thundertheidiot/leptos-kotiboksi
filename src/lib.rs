pub mod app;
pub mod css;

cfg_if::cfg_if! {
    if #[cfg(feature = "csr")] {
    pub mod game;
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}

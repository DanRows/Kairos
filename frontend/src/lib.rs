// Declaración explícita de módulos
pub mod components;
pub mod pages;
pub mod hooks;
pub mod i18n;

use dioxus::prelude::*;
use dioxus_router::prelude::{Routable, Router}; // Importamos explícitamente lo que usamos
use wasm_bindgen::prelude::*;

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[route("/login")]
    LoginPage {},
    #[route("/dashboard")]
    DashboardPage {},
}

#[wasm_bindgen(start)]
pub fn start() {
    launch(app); // La llamamos directamente
}

// Wrapper público para el entrypoint
pub fn app() -> Element {
    rsx! {
        div {
            class: "app-container",
            // Añadimos el selector de idioma en la parte superior de la app
            components::common::language_selector::LanguageSelector {},
            hr {},
            // El Router manejará el resto de la página
            Router::<Route> {}
        }
    }
}

#[component]
pub fn LoginPage() -> Element {
    rsx! { components::LoginForm {} }
}

#[component]
pub fn DashboardPage() -> Element {
    rsx! { pages::Dashboard {} }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_app_renders() {
        let _app = app(); // Usamos el nombre en snake_case
        // Aquí irían las pruebas de renderizado
    }
}
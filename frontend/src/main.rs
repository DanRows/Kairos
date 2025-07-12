use dioxus::prelude::*;

fn main() {
    // Configurar logging para el navegador
    console_log::init_with_level(log::Level::Info).expect("error initializing log");
    console_error_panic_hook::set_once();

    // Iniciar la aplicación Dioxus
    dioxus::launch(App);
}

fn App() -> Element {
            rsx! {
                div {
            style: "padding: 20px; font-family: Arial, sans-serif;",
            h1 { "🌾 Kairos - Logística Predictiva para el Agro" }
            p { "Frontend en desarrollo..." }
            p { 
                style: "color: #666;",
                "Backend disponible en: http://localhost:8080"
            }
        }
    }
} 
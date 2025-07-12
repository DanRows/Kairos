use dioxus::prelude::*;
use web_sys::window;
use kairos_common::Language;
use std::str::FromStr;

// El hook ahora trabaja con el enum `Language`
pub fn use_i18n() -> Signal<Language> {
    let language = use_signal(get_default_language);
    
    // Efecto para guardar el idioma en localStorage cuando cambie
    use_effect(move || {
        if let Some(window) = window() {
            if let Some(local_storage) = window.local_storage().ok().flatten() {
                let _ = local_storage.set_item("language", language().to_str());
            }
        }
    });
    
    language
}

// La función de traducción ahora usa el enum
pub fn use_translation() -> impl Fn(String) -> String {
    let language = use_i18n();
    
    move |key: String| -> String {
        // Implementación simple de traducción
        match key.as_str() {
            "app.title" => "Kairos".to_string(), // Título es siempre el mismo
            "app.welcome" => match language() {
                Language::Spanish => "Bienvenido".to_string(),
                Language::English => "Welcome".to_string(),
                Language::Portuguese => "Bem-vindo".to_string(),
                // Caso por defecto para los otros idiomas
                _ => "Welcome".to_string(),
            },
            _ => key,
        }
    }
}

// La función para obtener el idioma ahora devuelve el enum
fn get_default_language() -> Language {
    // Intenta obtener de localStorage
    if let Some(window) = window() {
        if let Some(local_storage) = window.local_storage().ok().flatten() {
            if let Some(saved_language) = local_storage.get_item("language").ok().flatten() {
                if let Ok(lang) = Language::from_str(&saved_language) {
                    return lang;
                }
            }
        }
    }
    
    // Intenta detectar del navegador
    if let Some(window) = window() {
        let languages = window.navigator().languages();
        for i in 0..languages.length() {
            if let Some(lang_str) = languages.get(i).as_string() {
                let lang_code = lang_str.to_lowercase();
                if lang_code.starts_with("es") { return Language::Spanish; }
                if lang_code.starts_with("pt") { return Language::Portuguese; }
                if lang_code.starts_with("en") { return Language::English; }
            }
        }
    }
    
    // Idioma por defecto
    Language::default()
}

// Ya no necesitamos las implementaciones aquí, se movieron a kairos-common 
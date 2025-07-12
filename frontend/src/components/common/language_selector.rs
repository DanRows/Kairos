use dioxus::prelude::*;
use crate::hooks::use_i18n;
use kairos_common::Language;
use std::str::FromStr;

#[component]
pub fn LanguageSelector() -> Element {
    let mut language = use_i18n();
    
    rsx! {
        div {
            class: "language-selector",
            style: "margin: 10px 0;",
            
            span {
                style: "margin-right: 10px;",
                "Idioma: "
            }
            
            select {
                value: "{language()}",
                onchange: move |event| {
                    if let Ok(lang) = Language::from_str(&event.value()) {
                        language.set(lang);
                    }
                },
                style: "padding: 5px; border-radius: 3px;",
                
                option { value: "es", "Español" }
                option { value: "en", "English" }
                option { value: "pt", "Português" }
                option { value: "fr", "Français" }
                option { value: "de", "Deutsch" }
                option { value: "it", "Italiano" }
                option { value: "ru", "Русский" }
                option { value: "zh", "中文" }
                option { value: "ja", "日本語" }
                option { value: "ko", "한국어" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_language_selector() {
        let app = App::new(|cx| {
            rsx! {
                LanguageSelector {}
            }
        });

        // Aquí irían las pruebas de renderizado
    }
} 
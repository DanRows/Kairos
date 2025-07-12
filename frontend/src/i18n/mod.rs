use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use kairos_common::Language;

pub fn language_code(lang: &Language) -> &'static str {
    match lang {
        Language::Spanish => "es",
        Language::Portuguese => "pt",
        Language::English => "en",
        Language::French => "fr",
        Language::German => "de",
        Language::Italian => "it",
        Language::Russian => "ru",
        Language::Chinese => "zh",
        Language::Japanese => "ja",
        Language::Korean => "ko",
    }
}

pub fn language_name(lang: &Language) -> &'static str {
    match lang {
        Language::Spanish => "Español",
        Language::Portuguese => "Português",
        Language::English => "English",
        Language::French => "Français",
        Language::German => "Deutsch",
        Language::Italian => "Italiano",
        Language::Russian => "Русский",
        Language::Chinese => "中文",
        Language::Japanese => "日本語",
        Language::Korean => "한국어",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translations {
    pub language: Language,
    pub messages: HashMap<String, String>,
}

impl Translations {
    pub fn new(language: Language) -> Self {
        Self {
            language,
            messages: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.messages.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.messages.get(key).map(|s| s.as_str())
    }
}

pub mod translations {
    use super::*;
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    static TRANSLATIONS: Lazy<Mutex<HashMap<Language, Translations>>> = Lazy::new(|| {
        let mut map = HashMap::new();
        
        // Español
        let mut es = Translations::new(Language::Spanish);
        es.add("app.title", "Kairos - Trazabilidad Agrícola");
        es.add("auth.login", "Iniciar Sesión");
        es.add("auth.register", "Registrarse");
        es.add("auth.email", "Correo Electrónico");
        es.add("auth.password", "Contraseña");
        es.add("auth.full_name", "Nombre Completo");
        es.add("auth.farm_name", "Nombre de la Finca");
        es.add("auth.phone", "Teléfono");
        es.add("lots.create", "Crear Lote");
        es.add("lots.list", "Lista de Lotes");
        es.add("lots.product_name", "Nombre del Producto");
        es.add("lots.crop_type", "Tipo de Cultivo");
        es.add("lots.estimated_quantity", "Cantidad Estimada");
        es.add("lots.unit_of_measure", "Unidad de Medida");
        es.add("lots.estimated_harvest_date", "Fecha Estimada de Cosecha");
        map.insert(Language::Spanish, es);

        // Português
        let mut pt = Translations::new(Language::Portuguese);
        pt.add("app.title", "Kairos - Rastreabilidade Agrícola");
        pt.add("auth.login", "Entrar");
        pt.add("auth.register", "Registrar");
        pt.add("auth.email", "E-mail");
        pt.add("auth.password", "Senha");
        pt.add("auth.full_name", "Nome Completo");
        pt.add("auth.farm_name", "Nome da Fazenda");
        pt.add("auth.phone", "Telefone");
        pt.add("lots.create", "Criar Lote");
        pt.add("lots.list", "Lista de Lotes");
        pt.add("lots.product_name", "Nome do Produto");
        pt.add("lots.crop_type", "Tipo de Cultura");
        pt.add("lots.estimated_quantity", "Quantidade Estimada");
        pt.add("lots.unit_of_measure", "Unidade de Medida");
        pt.add("lots.estimated_harvest_date", "Data Estimada de Colheita");
        map.insert(Language::Portuguese, pt);

        // English
        let mut en = Translations::new(Language::English);
        en.add("app.title", "Kairos - Agricultural Traceability");
        en.add("auth.login", "Login");
        en.add("auth.register", "Register");
        en.add("auth.email", "Email");
        en.add("auth.password", "Password");
        en.add("auth.full_name", "Full Name");
        en.add("auth.farm_name", "Farm Name");
        en.add("auth.phone", "Phone");
        en.add("lots.create", "Create Lot");
        en.add("lots.list", "Lot List");
        en.add("lots.product_name", "Product Name");
        en.add("lots.crop_type", "Crop Type");
        en.add("lots.estimated_quantity", "Estimated Quantity");
        en.add("lots.unit_of_measure", "Unit of Measure");
        en.add("lots.estimated_harvest_date", "Estimated Harvest Date");
        map.insert(Language::English, en);

        Mutex::new(map)
    });

    pub fn get_translation(language: Language, key: &str) -> String {
        let translations = TRANSLATIONS.lock().unwrap();
        if let Some(lang_translations) = translations.get(&language) {
            if let Some(translation) = lang_translations.get(key) {
                return translation.to_string();
            }
        }
        // Fallback to English if translation not found
        if let Some(en_translations) = translations.get(&Language::English) {
            if let Some(translation) = en_translations.get(key) {
                return translation.to_string();
            }
        }
        key.to_string()
    }

    pub fn get_language_translations(language: Language) -> Option<Translations> {
        let translations = TRANSLATIONS.lock().unwrap();
        translations.get(&language).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_codes() {
        assert_eq!(Language::Spanish.code(), "es");
        assert_eq!(Language::Portuguese.code(), "pt");
        assert_eq!(Language::English.code(), "en");
    }

    #[test]
    fn test_translations() {
        assert_eq!(
            translations::get_translation(Language::Spanish, "app.title"),
            "Kairos - Trazabilidad Agrícola"
        );
        assert_eq!(
            translations::get_translation(Language::Portuguese, "app.title"),
            "Kairos - Rastreabilidade Agrícola"
        );
        assert_eq!(
            translations::get_translation(Language::English, "app.title"),
            "Kairos - Agricultural Traceability"
        );
    }
} 
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppLocale {
    English,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageConfigs {
    locale: AppLocale,
    translations: TranslationData,
}

impl LanguageConfigs {
    pub fn translations(&self) -> &TranslationData {
        &self.translations
    }

    pub fn current_locale(&self) -> AppLocale {
        self.locale
    }

    pub fn t(&self, key: &str) -> String {
        self.translations.get_translation(key)
    }
}

pub enum LanguageConfigsAction {
    ChangeLocale(AppLocale),
}

impl Reducible for LanguageConfigs {
    type Action = LanguageConfigsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            LanguageConfigsAction::ChangeLocale(locale) => Rc::new(LanguageConfigs {
                locale,
                translations: TranslationData::load_translation(locale),
            }),
        }
    }
}

pub type LanguageConfigsStore = UseReducerHandle<LanguageConfigs>;

#[function_component(LanguageConfigsProvider)]
pub fn language_config_provider(props: &yew::html::ChildrenProps) -> Html {
    let ctx = use_reducer(|| LanguageConfigs {
        locale: AppLocale::English,
        translations: TranslationData::default(),
    });

    html! {
        <ContextProvider<LanguageConfigsStore> context={ctx}>
            {props.children.clone()}
        </ContextProvider<LanguageConfigsStore>>
    }
}

#[derive(Deserialize, Debug, Serialize, Clone, PartialEq, Eq)]
pub struct TranslationData {
    #[serde(flatten)]
    pub translations: HashMap<String, Value>,
}

impl Default for TranslationData {
    fn default() -> Self {
        Self::load_translation(AppLocale::English)
    }
}

impl TranslationData {
    pub fn load_translation(locale: AppLocale) -> Self {
        match locale {
            AppLocale::English => serde_json::from_str(ENGLISH_TRANSLATIONS).unwrap(),
        }
    }

    // Get translation by flat key like "common_save" or "game_details_event"
    pub fn get_translation(&self, key: &str) -> String {
        // Direct key lookup
        if let Some(value) = self.translations.get(key) {
            return self.extract_string(value);
        }

        if key.contains('.') {
            let flat_key = key.replace('.', "_");
            if let Some(value) = self.translations.get(&flat_key) {
                return self.extract_string(value);
            }
        }

        // Return the key if no translation is found
        key.to_string()
    }

    fn extract_string(&self, value: &Value) -> String {
        if let Some(s) = value.as_str() {
            s.to_string()
        } else {
            value.to_string()
        }
    }
}

// Use the flattened JSON structure for translations
static ENGLISH_TRANSLATIONS: &str = include_str!("../../../static_resources/language/en.json");

// Helper function to use the language context
#[hook]
pub fn use_language_ctx() -> LanguageConfigsStore {
    use_context::<LanguageConfigsStore>().expect("LanguageConfigsStore context not set")
}

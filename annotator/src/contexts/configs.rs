use nostr_minions::browser_api::IdbStoreManager;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ExperienceLevel {
    #[default]
    Rookie,
    Expert,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    #[default]
    English,
    Spanish,
    Portuguese,
}
impl std::str::FromStr for ExperienceLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|_| ())
    }
}
impl std::str::FromStr for Language {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|_| ())
    }
}
impl std::fmt::Display for ExperienceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self)
            .map_err(|_| std::fmt::Error)?
            .fmt(f)
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self)
            .map_err(|_| std::fmt::Error)?
            .fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AnnotatorConfigurationEntry {
    id: String,
    pub language: Language,
    pub experience_level: ExperienceLevel,
}
impl Default for AnnotatorConfigurationEntry {
    fn default() -> Self {
        Self {
            id: "app_config".to_string(),
            language: Language::English,
            experience_level: ExperienceLevel::Rookie,
        }
    }
}
impl TryFrom<web_sys::wasm_bindgen::JsValue> for AnnotatorConfigurationEntry {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: web_sys::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}
impl From<AnnotatorConfigurationEntry> for web_sys::wasm_bindgen::JsValue {
    fn from(value: AnnotatorConfigurationEntry) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap_or_default()
    }
}

impl nostr_minions::browser_api::IdbStoreManager for AnnotatorConfigurationEntry {
    fn config() -> nostr_minions::browser_api::IdbStoreConfig {
        nostr_minions::browser_api::IdbStoreConfig {
            db_name: "annotator_config_db",
            store_name: "annotator_config_store",
            db_version: 1,
            document_key: "id",
        }
    }
    fn key(&self) -> web_sys::wasm_bindgen::JsValue {
        web_sys::wasm_bindgen::JsValue::from_str(&self.id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AnnotatorConfig {
    pub loaded: bool,
    pub language: Language,
    pub experience_level: ExperienceLevel,
}

pub enum AnnotatorConfigAction {
    Loaded,
    LoadConfig(AnnotatorConfigurationEntry),
    SetLanguage(Language),
    SetExperienceLevel(ExperienceLevel),
}

impl Reducible for AnnotatorConfig {
    type Action = AnnotatorConfigAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            AnnotatorConfigAction::SetLanguage(language) => {
                let new_entry = AnnotatorConfigurationEntry {
                    language,
                    experience_level: self.experience_level,
                    ..Default::default()
                };
                yew::platform::spawn_local(async move {
                    if let Err(e) = new_entry.save_to_store().await {
                        web_sys::console::error_1(&format!("Error saving config: {:?}", e).into());
                    }
                });

                std::rc::Rc::new(Self {
                    loaded: self.loaded,
                    language,
                    experience_level: self.experience_level,
                })
            }
            AnnotatorConfigAction::SetExperienceLevel(experience_level) => {
                let new_entry = AnnotatorConfigurationEntry {
                    language: self.language,
                    experience_level,
                    ..Default::default()
                };
                yew::platform::spawn_local(async move {
                    if let Err(e) = new_entry.save_to_store().await {
                        web_sys::console::error_1(&format!("Error saving config: {:?}", e).into());
                    }
                });
                std::rc::Rc::new(Self {
                    loaded: self.loaded,
                    language: self.language,
                    experience_level,
                })
            }
            AnnotatorConfigAction::LoadConfig(AnnotatorConfigurationEntry {
                id: _,
                language,
                experience_level,
            }) => std::rc::Rc::new(Self {
                loaded: true,
                language,
                experience_level,
            }),
            AnnotatorConfigAction::Loaded => std::rc::Rc::new(Self {
                loaded: true,
                language: self.language,
                experience_level: self.experience_level,
            }),
        }
    }
}

pub type AnnotatorConfigStore = UseReducerHandle<AnnotatorConfig>;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct AnnotatorConfigChildren {
    pub children: Children,
}

#[function_component(AnnotatorConfigProvider)]
pub fn key_handler(props: &AnnotatorConfigChildren) -> Html {
    let ctx = use_reducer(AnnotatorConfig::default);
    {
        use_memo((), |_| {
            let ctx = ctx.clone();
            yew::platform::spawn_local(async move {
                let entry = AnnotatorConfigurationEntry::retrieve_from_store::<
                    AnnotatorConfigurationEntry,
                >(&"app_config".into())
                .await;
                match entry {
                    Ok(entry) => {
                        ctx.dispatch(AnnotatorConfigAction::LoadConfig(entry));
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Error loading config: {:?}", e).into());
                    }
                }
            });
        });
    }

    html! {
        <ContextProvider<AnnotatorConfigStore> context={ctx}>
            {props.children.clone()}
        </ContextProvider<AnnotatorConfigStore>>
    }
}

#[hook]
pub fn use_annotator_config() -> UseReducerHandle<AnnotatorConfig> {
    use_context::<AnnotatorConfigStore>().expect("AnnotatorConfigStore context")
}

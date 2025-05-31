#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum GameOrigin {
    Annotated,
    Received,
    Public,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct RookyGameEntry {
    pub id: String,
    pub note: nostr_minions::nostro2::NostrNote,
    pub origin: GameOrigin,
}

impl From<&RookyGameEntry> for crate::RookyGame {
    fn from(entry: &RookyGameEntry) -> Self {
        entry.note.content.parse::<Self>().unwrap_or_default()
    }
}
impl From<RookyGameEntry> for crate::RookyGame {
    fn from(entry: RookyGameEntry) -> Self {
        entry.note.content.parse::<Self>().unwrap_or_default()
    }
}
impl From<RookyGameEntry> for web_sys::wasm_bindgen::JsValue {
    fn from(entry: RookyGameEntry) -> Self {
        serde_wasm_bindgen::to_value(&entry).unwrap_or(Self::NULL)
    }
}
impl TryFrom<web_sys::wasm_bindgen::JsValue> for RookyGameEntry {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: web_sys::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(value).map_err(|e| {
            web_sys::console::error_1(
                &format!("Failed to convert JsValue to RookyGameEntry: {e:?}").into(),
            );
            web_sys::wasm_bindgen::JsValue::from_str("Conversion error")
        })
    }
}
impl nostr_minions::browser_api::IdbStoreManager for RookyGameEntry {
    fn key(&self) -> web_sys::wasm_bindgen::JsValue {
        web_sys::wasm_bindgen::JsValue::from_str(self.note.id.clone().unwrap_or_default().as_str())
    }
    fn config() -> nostr_minions::browser_api::IdbStoreConfig {
        nostr_minions::browser_api::IdbStoreConfig {
            store_name: "rooky_games",
            db_name: "rooky_db",
            db_version: 3,
            document_key: "id",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum GameOrigin {
    Annotated,
    Received,
    Public,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct RookyGameEntry {
    #[serde(flatten)]
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
        serde_wasm_bindgen::to_value(&entry.note).unwrap_or(Self::NULL)
    }
}
impl TryFrom<nostr_minions::nostro2::NostrNote> for RookyGameEntry {
    type Error = Box<dyn std::error::Error>;
    fn try_from(note: nostr_minions::nostro2::NostrNote) -> Result<Self, Self::Error> {
        note.content.parse::<crate::RookyGame>()?;
        Ok(Self {
            note,
            origin: GameOrigin::Public,
        })
    }
}
impl TryFrom<web_sys::wasm_bindgen::JsValue> for RookyGameEntry {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: web_sys::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
        let note: nostr_minions::nostro2::NostrNote = serde_wasm_bindgen::from_value(value)?;
        note.try_into()
            .map_err(|e| web_sys::wasm_bindgen::JsValue::from_str(&format!("{e:?}")))
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
            db_version: 2,
            document_key: "id",
        }
    }
}

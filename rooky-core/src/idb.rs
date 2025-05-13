#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct RookyGameEntry(pub nostr_minions::nostro2::note::NostrNote);
impl From<&RookyGameEntry> for crate::RookyGame {
    fn from(entry: &RookyGameEntry) -> Self {
        entry.0.content.parse::<Self>().unwrap_or_default()
    }
}
impl From<RookyGameEntry> for crate::RookyGame {
    fn from(entry: RookyGameEntry) -> Self {
        entry.0.content.parse::<Self>().unwrap_or_default()
    }
}
impl From<RookyGameEntry> for web_sys::wasm_bindgen::JsValue {
    fn from(entry: RookyGameEntry) -> Self {
        serde_wasm_bindgen::to_value(&entry.0).unwrap_or(Self::NULL)
    }
}
impl TryFrom<nostr_minions::nostro2::note::NostrNote> for RookyGameEntry {
    type Error = Box<dyn std::error::Error>;
    fn try_from(note: nostr_minions::nostro2::note::NostrNote) -> Result<Self, Self::Error> {
        note.content.parse::<crate::RookyGame>()?;
        Ok(Self(note))
    }
}
impl TryFrom<web_sys::wasm_bindgen::JsValue> for RookyGameEntry {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: web_sys::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
        let note: nostr_minions::nostro2::note::NostrNote = serde_wasm_bindgen::from_value(value)?;
        note.try_into()
            .map_err(|e| web_sys::wasm_bindgen::JsValue::from_str(&format!("{e:?}")))
    }
}
impl nostr_minions::browser_api::IdbStoreManager for RookyGameEntry {
    fn key(&self) -> web_sys::wasm_bindgen::JsValue {
        web_sys::wasm_bindgen::JsValue::from_str(self.0.id.clone().unwrap_or_default().as_str())
    }
    fn config() -> nostr_minions::browser_api::IdbStoreConfig {
        nostr_minions::browser_api::IdbStoreConfig {
            store_name: "rooky_games",
            db_name: "rooky_db",
            db_version: 1,
            document_key: "id",
        }
    }
}

use nostr_minions::key_manager::UserIdentity;
use nostr_minions::nostro2::NostrNote;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use web_sys::wasm_bindgen::JsValue;

/// Represents user metadata as specified in NIP-01
/// Kind 0: user metadata
/// A stringified JSON object containing name, about, and picture
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NostrMetadata {
    pub name: String,
    pub about: Option<String>,
    pub picture: Option<String>,
}

impl Default for NostrMetadata {
    fn default() -> Self {
        Self {
            name: "Anon".to_string(),
            about: None,
            picture: None,
        }
    }
}

impl FromStr for NostrMetadata {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl TryFrom<NostrNote> for NostrMetadata {
    type Error = nostr_minions::nostro2::errors::NostrErrors;
    fn try_from(note: NostrNote) -> Result<Self, Self::Error> {
        if note.kind != 0 {
            return Err(nostr_minions::nostro2::errors::NostrErrors::from(
                "Wrong Kind - expected kind 0",
            ));
        }
        let metadata: Self = note.content.parse()?;
        Ok(metadata)
    }
}

impl NostrMetadata {
    pub const fn new(name: String, about: Option<String>, picture: Option<String>) -> Self {
        Self {
            name,
            about,
            picture,
        }
    }

    /// # Errors
    /// Returns a `serde_json::Error` if the struct cannot be serialized to JSON.
    /// This can happen if the struct contains invalid data that cannot be represented in JSON.
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserMetadataIdb {
    pub pubkey: String,
    #[serde(skip_serializing_if = "exclude_note", skip_deserializing)]
    pub note: NostrNote,
    pub metadata: NostrMetadata,
}

fn exclude_note(_: &NostrNote) -> bool {
    // Always exclude the note when serializing to avoid issues
    true
}

impl UserMetadataIdb {
    /// # Panics
    /// Panics if the `UserIdentity` doesn't provide a public key or if signing fails.
    #[allow(clippy::future_not_send)]
    pub async fn new(metadata: NostrMetadata, keys: &UserIdentity) -> Self {
        let pubkey = keys.get_pubkey().await.expect("no pubkey");
        let created_at = chrono::Utc::now().timestamp();
        let mut note = NostrNote {
            pubkey: pubkey.clone(),
            created_at,
            kind: 0,                  // NIP-01 kind 0 for metadata
            tags: Default::default(), // Use default for NoteTags
            content: metadata.to_json_string().unwrap_or_default(),
            ..Default::default()
        };

        if let Err(e) = note.serialize_id() {
            web_sys::console::error_1(&format!("Failed to serialize note ID: {:?}", e).into());
        }

        let mut note_to_sign = note.clone();
        keys.sign_nostr_note(&mut note_to_sign)
            .await
            .expect("Failed to sign nostr note");

        Self {
            pubkey,
            note: note_to_sign,
            metadata,
        }
    }

    /// Creates a placeholder instance without signing (for temporary use in UI)
    #[must_use]
    pub fn placeholder(metadata: NostrMetadata) -> Self {
        let note = NostrNote {
            kind: 0,
            content: metadata.to_json_string().unwrap_or_default(),
            ..Default::default()
        };

        Self {
            pubkey: "placeholder".to_string(),
            note,
            metadata,
        }
    }

    #[must_use]
    pub fn signed_note(&self) -> NostrNote {
        self.note.clone()
    }

    #[must_use]
    pub fn metadata(&self) -> NostrMetadata {
        self.metadata.clone()
    }

    #[must_use]
    pub fn pubkey(&self) -> String {
        self.pubkey.clone()
    }
}

impl From<UserMetadataIdb> for JsValue {
    fn from(val: UserMetadataIdb) -> Self {
        serde_wasm_bindgen::to_value(&val).unwrap_or_default()
    }
}

impl TryFrom<JsValue> for UserMetadataIdb {
    type Error = JsValue;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}

impl TryFrom<NostrNote> for UserMetadataIdb {
    type Error = nostr_minions::nostro2::errors::NostrErrors;
    fn try_from(note: NostrNote) -> Result<Self, Self::Error> {
        if note.kind != 0 {
            return Err(nostr_minions::nostro2::errors::NostrErrors::from(
                "Wrong Kind - expected kind 0",
            ));
        }
        let pubkey = note.pubkey.clone();
        let metadata: NostrMetadata = note.content.parse()?;
        Ok(Self {
            pubkey,
            note,
            metadata,
        })
    }
}

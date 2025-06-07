use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

impl std::str::FromStr for NostrMetadata {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl TryFrom<&nostr_minions::nostro2::NostrNote> for NostrMetadata {
    type Error = nostr_minions::nostro2::errors::NostrErrors;
    fn try_from(note: &nostr_minions::nostro2::NostrNote) -> Result<Self, Self::Error> {
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

/// A profile card component that uses the UserMetadataProvider context
#[function_component(UserProfileCard)]
pub fn user_profile_card() -> Html {
    // Get contexts needed
    let language_ctx = crate::contexts::language::use_language_ctx();
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("Relay context not found");
    let pubkey = nostr_minions::key_manager::use_nostr_key();
    let copied_hex = use_state(|| false);
    let copied_npub = use_state(|| false);

    let relay_clone = relay_ctx.clone();
    use_memo(pubkey.clone(), move |pubkey| {
        if let Some(pubkey) = pubkey.as_ref().map(|pk| pk.public_key()) {
            let filter = nostr_minions::nostro2::NostrSubscription {
                kinds: vec![0].into(), // Kind 0 for user metadata
                authors: vec![pubkey.clone()].into(),
                ..Default::default()
            };
            relay_clone.send(filter);
        }
    });
    let metadata = use_memo(relay_ctx.unique_notes.clone(), move |notes| {
        let note = notes.last()?;
        (note.kind == 0)
            .then(|| NostrMetadata::try_from(note).ok())
            .flatten()
    });

    html! {
        <div class="p-6">
            <div class="flex items-start gap-6">
                // <div class="flex-shrink-0">
                //     <img
                //         src={metadata.as_ref().as_ref().and_then(|p| p.picture.clone())
                //             .unwrap_or_else(|| "/public/img/default-avatar.png".to_string())}
                //         alt="Profile"
                //         class="size-20 object-cover rounded-full shadow-md border border-gray-300"
                //         />
                // </div>

                <div class="flex-1 space-y-4">
                    <div>
                        <h2 class="text-xl font-semibold mb-2">
                            {metadata.as_ref().as_ref().map(|p| p.name.clone()).unwrap_or_else(|| language_ctx.t("anonymous_user"))}
                        </h2>
                        <p class="text-sm text-muted leading-relaxed">
                            {metadata.as_ref().as_ref().and_then(|p| p.about.clone()).unwrap_or_else(|| language_ctx.t("no_bio"))}
                        </p>
                    </div>

                    <div class="grid grid-cols-2 gap-4">
                        <div class="max-w-xs">
                            <label class="text-sm font-medium text-muted">{ language_ctx.t("key_recovery_public_key") }</label>
                            <div onclick={
                                let pubkey_clone = pubkey.clone();
                                let copied = copied_hex.setter();
                                Callback::from(move |_| {
                                       if let Some(pubkey) = pubkey_clone.as_ref() {
                                           let text = pubkey.public_key();
                                           nostr_minions::browser_api::clipboard_copy(&text);
                                           copied.set(true);
                                           let copied = copied.clone();
                                           gloo::timers::callback::Timeout::new(2000, move || {
                                               copied.set(false);
                                           }).forget();
                                       }
                                   })
                                }
                                class="flex items-center gap-2 p-2 bg-muted rounded-md">
                                <code class="flex-1 text-xs font-mono text-muted-foreground truncate">
                                    {
                                            if *copied_hex {
                                                language_ctx.t("notification_copied_to_clipboard")
                                            } else {
                                                pubkey.as_ref().map_or_else(
                                                || language_ctx.t("profile_no_public_key"),
                                                |pk| pk.public_key())
                                            }
                                        }
                                </code>
                                <button
                                   class="size-8 p-0 flex-shrink-0">
                                    {if *copied_hex {
                                       html! { <lucide_yew::Check class="size-8 text-green-600" /> }
                                    } else {
                                       html! { <lucide_yew::Copy class="size-8" /> }
                                    }}
                                    <span class="sr-only">{ language_ctx.t("key_recovery_copy_public_key") }</span>
                                </button>
                            </div>
                        </div>
                        <div class="max-w-xs">
                            <label class="text-sm font-medium text-muted">{ language_ctx.t("profile_npub") }</label>
                            <div onclick={
                                    let pubkey_clone = pubkey.clone();
                                    let copied = copied_npub.setter();
                                    Callback::from(move |_| {
                                        if let Some(pubkey) = pubkey_clone.as_ref().and_then(|pk| pk.npub().ok()) {
                                            nostr_minions::browser_api::clipboard_copy(&pubkey);
                                            copied.set(true);
                                            let copied = copied.clone();
                                            gloo::timers::callback::Timeout::new(2000, move || {
                                                copied.set(false);
                                            }).forget();
                                        }
                                    })
                                }
                                class="flex items-center gap-2 p-2 bg-muted rounded-md">
                                <code class="flex-1 text-xs font-mono text-muted-foreground truncate">
                                    {
                                            if *copied_npub {
                                                language_ctx.t("notification_copied_to_clipboard")
                                            } else {
                                                pubkey.as_ref().and_then(|pk| pk.npub().ok()).unwrap_or(
                                                language_ctx.t("profile_no_public_key")
                                                )
                                            }
                                        }
                                </code>
                                <button
                                   class="size-8 p-0 flex-shrink-0">
                                    {if *copied_npub {
                                       html! { <lucide_yew::Check class="size-8 text-green-600" /> }
                                    } else {
                                       html! { <lucide_yew::Copy class="size-8" /> }
                                    }}
                                    <span class="sr-only">{ language_ctx.t("profile_copy_npub") }</span>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

          //   <Button
          //       // {onclick}
          //       variant={shady_minions::ui::ButtonVariant::Outline}
          //       class="w-full text-sm xs:text-base"
          //   >
          //       <lucide_yew::Pen class="size-4 mr-2" />
          //       { language_ctx.t("edit_profile") }
          //   </Button>
    }
}

use crate::models::{NostrMetadata, UserMetadataIdb};
use nostr_minions::nostro2::NostrNote;
use nostr_minions::widgets::toastify::ToastifyOptions;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub enum UserMetadataAction {
    SetMetadata(Box<UserMetadataIdb>),
    UpdateName(String),
    UpdateAbout(String),
    UpdatePicture(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserMetadataStore {
    metadata: Option<UserMetadataIdb>,
}

impl UserMetadataStore {
    pub fn new(metadata: Option<UserMetadataIdb>) -> Self {
        Self { metadata }
    }

    #[must_use]
    pub fn get_metadata(&self) -> Option<NostrMetadata> {
        self.metadata.as_ref().map(|m| m.metadata())
    }

    #[must_use]
    pub fn get_note(&self) -> Option<NostrNote> {
        self.metadata.as_ref().map(|m| m.signed_note())
    }
}

impl Reducible for UserMetadataStore {
    type Action = UserMetadataAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut cloned = (*self).clone();
        match action {
            UserMetadataAction::SetMetadata(metadata) => {
                cloned.metadata = Some(*metadata);
            }
            UserMetadataAction::UpdateName(name) => {
                if let Some(metadata) = &cloned.metadata {
                    let mut new_metadata = metadata.metadata();
                    new_metadata.name = name;
                    // We'll create a temporary placeholder and replace it later when saving
                    // This avoids the async/await in the reducer
                    cloned.metadata = Some(UserMetadataIdb::placeholder(new_metadata));
                }
            }
            UserMetadataAction::UpdateAbout(about) => {
                if let Some(metadata) = &cloned.metadata {
                    let mut new_metadata = metadata.metadata();
                    new_metadata.about = Some(about);
                    // Temporary placeholder and replace it later when saving
                    cloned.metadata = Some(UserMetadataIdb::placeholder(new_metadata));
                }
            }
            UserMetadataAction::UpdatePicture(picture) => {
                if let Some(metadata) = &cloned.metadata {
                    let mut new_metadata = metadata.metadata();
                    new_metadata.picture = Some(picture);
                    // We'll create a temporary placeholder and replace it later when saving
                    cloned.metadata = Some(UserMetadataIdb::placeholder(new_metadata));
                }
            }
        };
        Rc::new(cloned)
    }
}

#[derive(Properties, PartialEq)]
pub struct UserMetadataProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(UserMetadataProvider)]
pub fn user_metadata_provider(props: &UserMetadataProviderProps) -> Html {
    let user_metadata_store = use_reducer(|| UserMetadataStore::new(None));

    let key_ctx = nostr_minions::key_manager::use_nostr_id_ctx();
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("Relay context not found");

    // Just subscribe to metadata events once and update the store when new events arrive
    {
        let user_metadata_store = user_metadata_store.clone();
        let key_ctx = key_ctx.clone();
        let relay_ctx = relay_ctx.clone();

        // Subscribe to kind 0 metadata events matching our pubkey (once)
        {
            let relay_ctx = relay_ctx.clone();
            let key_ctx = key_ctx.clone();

            use_effect_with((), move |_| {
                let key_ctx = key_ctx.clone();
                if let Some(identity) = key_ctx.get_identity() {
                    let identity = identity.clone();
                    yew::platform::spawn_local(async move {
                        if let Some(pubkey) = identity.get_pubkey().await {
                            let metadata_filter = nostr_minions::nostro2::NostrSubscription {
                                kinds: Some(vec![0]),        // Kind 0 for metadata
                                authors: Some(vec![pubkey]), // Only look for our own events
                                limit: Some(1),              // Only get the most recent
                                ..Default::default()
                            };

                            // Subscribe to metadata events
                            relay_ctx.send(metadata_filter);

                            web_sys::console::log_1(&"Subscribed to metadata events".into());
                        }
                    });
                }
                || {}
            });
        }

        // Listen for relay events that might contain metadata
        {
            let user_store = user_metadata_store.clone();

            use_effect_with(relay_ctx.unique_notes.clone(), move |notes| {
                // Process any new notes that might contain metadata
                if let Some(note) = notes.last() {
                    if note.kind == 0 {
                        // Found a metadata note, trying to convert it
                        if let Ok(metadata_idb) = UserMetadataIdb::try_from(note.clone()) {
                            web_sys::console::log_1(
                                &format!(
                                    "Received metadata from relay: {:?}",
                                    metadata_idb.metadata
                                )
                                .into(),
                            );

                            let toast_message = format!(
                                "Received profile data for: {}",
                                metadata_idb.metadata.name
                            );
                            let toast = ToastifyOptions::new_event_received(&toast_message);
                            toast.show();

                            // Log when metadata is received from relay
                            web_sys::console::log_1(&"Metadata event received from relay".into());

                            user_store
                                .dispatch(UserMetadataAction::SetMetadata(Box::new(metadata_idb)));
                        }
                    }
                }
                || {}
            });
        }

        // Set initial placeholder until we receive real data
        {
            let user_store = user_metadata_store.clone();
            let key_ctx = key_ctx.clone();

            use_effect_with(key_ctx.get_pubkey(), move |pubkey| {
                if pubkey.is_some() {
                    // Create a placeholder with default metadata
                    let default_metadata = NostrMetadata::default();
                    let placeholder = UserMetadataIdb::placeholder(default_metadata);
                    user_store.dispatch(UserMetadataAction::SetMetadata(Box::new(placeholder)));
                }
                || {}
            });
        }
    }

    html! {
        <ContextProvider<UseReducerHandle<UserMetadataStore>> context={user_metadata_store}>
            { for props.children.iter() }
        </ContextProvider<UseReducerHandle<UserMetadataStore>>>
    }
}

#[hook]
pub fn use_user_metadata_ctx() -> UseReducerHandle<UserMetadataStore> {
    use_context::<UseReducerHandle<UserMetadataStore>>().expect("No UserMetadataStore found")
}

/// Hook to get a callback for publishing user metadata to Nostr relays
#[hook]
pub fn use_publish_metadata() -> Callback<NostrMetadata, ()> {
    let key_ctx = nostr_minions::key_manager::use_nostr_id_ctx();
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("Relay context not found");
    let user_metadata_store = use_user_metadata_ctx();

    Callback::from(move |metadata: NostrMetadata| {
        let key_ctx = key_ctx.clone();
        let relay_ctx = relay_ctx.clone();
        let user_metadata_store = user_metadata_store.clone();

        // Skip if no identity is available
        let Some(identity) = key_ctx.get_identity().cloned() else {
            return;
        };

        yew::platform::spawn_local(async move {
            // Creating a new UserMetadataIdb instance with the updated metadata
            let metadata_idb = UserMetadataIdb::new(metadata.clone(), &identity).await;

            // Get the signed note to publish
            let note = metadata_idb.signed_note();

            // Update our local store first for immediate UI feedback
            user_metadata_store.dispatch(UserMetadataAction::SetMetadata(Box::new(metadata_idb)));

            // Publish the note to relays
            relay_ctx.send(note);

            web_sys::console::log_1(&"Published metadata to relays".into());
        });
    })
}

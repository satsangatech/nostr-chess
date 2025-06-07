use nostr_minions::browser_api::IdbStoreManager;
use shady_minions::ui::{Button, ButtonVariant, Card, CardContent, CardHeader, CardTitle, Input};
use web_sys::MouseEvent;

use nostr_minions::key_manager::{NostrIdAction, NostrIdStore};
use yew::prelude::*;

#[function_component(KeyRecoveryPage)]
pub fn key_recovery_page() -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();

    html! {
        <Card class="max-w-md h-fit">
            <CardHeader>
                <CardTitle>
                    <div class="flex items-center space-x-3 pb-2">
                        <lucide_yew::Key class="text-primary size-6" />
                        <h3 class="text-2xl font-bold">{ language_ctx.t("key_recovery_title") }</h3>
                    </div>
                </CardTitle>
            </CardHeader>
            <CardContent class="space-y-6">
                <div class="flex gap-3">
                    <lucide_yew::TriangleAlert class="text-yellow-500 size-5 mt-1 flex-shrink-0" />
                    <p class="text-muted">
                        { language_ctx.t("key_recovery_use_key") }
                        <br />
                        { language_ctx.t("key_recovery_keep_safe") }
                    </p>
                </div>
                <KeyRecoverySection />
            </CardContent>
        </Card>
    }
}

#[function_component(KeyRecoverySection)]
fn key_recovery_section() -> Html {
    let key_ctx = use_context::<NostrIdStore>().expect("No NostrIdStore found");
    let language_ctx = crate::contexts::language::use_language_ctx();

    let priv_key_copied = use_state(|| false);
    let mnemonic_copied = use_state(|| false);
    let pubkey_copied = use_state(|| false);
    // let npub_copied = use_state(|| false);

    // State for showing/hiding sensitive data
    let show_sensitive = use_state(|| false);

    let pubkey = use_state(String::new);

    // State for private key and recovery phrase
    let priv_key = use_state(|| "".to_string());
    let recovery_phrase = use_state(Vec::<String>::new);
    let id_state = use_state(|| key_ctx.get_identity().cloned());

    // Check if user is using extension-based identity
    let is_extension = use_state(|| false);

    // Fetch key information and determine if user is using extension
    let priv_key_handle = priv_key.clone();
    let recovery_phrase_handle = recovery_phrase.clone();
    // let is_extension_handle = is_extension.clone();
    let id_handle = id_state.clone();
    let pubkey_setter = pubkey.setter();
    let notification_text = language_ctx.t("notification_copied_to_clipboard");
    use_effect_with(key_ctx.clone(), move |key_handle| {
        let priv_key_handle = priv_key_handle.clone();
        let recovery_phrase_handle = recovery_phrase_handle.clone();
        // let is_extension_handle = is_extension_handle.clone();
        let key_handle = key_handle.clone();

        let pubkey_setter = pubkey_setter.clone();
        yew::platform::spawn_local(async move {
            // Check if identity is extension-based by attempting to get the key
            id_handle.set(key_handle.get_identity().cloned());

            if let Some(mut key) = key_handle.get_nostr_key().await {
                // Set key as extractable to access sensitive data
                key.set_extractable(true);

                if let Ok(npub) = key.npub() {
                    pubkey_setter.set(npub.to_string());
                } else {
                    web_sys::console::log_1(&"Failed to retrieve public key".into());
                };

                // Get private key as hex
                let Ok(secret_key) = key.nsec() else {
                    web_sys::console::log_1(&"Failed to retrieve private key".into());
                    return;
                };
                priv_key_handle.set(secret_key);
                // Get the recovery phrase (mnemonic)
                // The key must be extractable to access the mnemonic
                if let Ok(mnemonic) = key.mnemonic(nostr_minions::nostro2_signer::Language::English)
                {
                    let words: Vec<String> =
                        mnemonic.split_whitespace().map(String::from).collect();
                    recovery_phrase_handle.set(words);
                }
            }
        });
        || {}
    });

    let onclick_copy_privkey = {
        let secret_key = priv_key.clone();
        let copied = priv_key_copied.clone();
        Callback::from(move |_| {
            nostr_minions::browser_api::clipboard_copy(&secret_key);
            copied.set(true);
            let copied = copied.clone();
            gloo::timers::callback::Timeout::new(2000, move || {
                copied.set(false);
            })
            .forget();
        })
    };

    let onclick_copy_phrase = {
        let phrase = recovery_phrase.clone();
        let copied = mnemonic_copied.clone();
        Callback::from(move |_| {
            nostr_minions::browser_api::clipboard_copy(&phrase.join(" "));
            copied.set(true);
            let copied = copied.clone();
            gloo::timers::callback::Timeout::new(2000, move || {
                copied.set(false);
            })
            .forget();
        })
    };

    let onclick_copy_pubkey = {
        let pubkey = pubkey.clone();
        let copied = pubkey_copied.clone();
        Callback::from(move |_| {
            nostr_minions::browser_api::clipboard_copy(&pubkey);
            copied.set(true);
            let copied = copied.clone();
            gloo::timers::callback::Timeout::new(2000, move || {
                copied.set(false);
            })
            .forget();
        })
    };

    let delete_key = {
        let key_handle = key_ctx.dispatcher();
        let id_state = id_state.clone();
        let lang_ctx = language_ctx.clone();
        Callback::from(move |_| {
            // if let Some(Ok(confirmed)) =
            if let Some(true) = web_sys::window().and_then(|win| {
                win.confirm_with_message(&lang_ctx.t("key_recovery_delete_confirm"))
                    .ok()
            }) {
                // First dispatch the DeleteIdentity action
                key_handle.dispatch(NostrIdAction::DeleteIdentity);
                let id = (*id_state).clone();
                yew::platform::spawn_local(async move {
                    let Some(id) = id else {
                        web_sys::console::log_1(&"No identity found to delete".into());
                        return;
                    };
                    if let Err(e) = id.delete_from_store().await {
                        web_sys::console::log_1(
                            &format!("Failed to delete identity: {:?}", e).into(),
                        );
                    }
                });
            }
        })
    };

    html! {
        <div class="space-y-6 overflow-y-auto pb-6">
            // Delete key button
            <div class="flex gap-3 w-full">
                // Show/Hide Toggle Button
                {
                    if !(*is_extension) {
                        html! {
                            <Button
                                onclick={
                                    let show_sensitive = show_sensitive.clone();
                                    Callback::from(move |_| show_sensitive.set(!*show_sensitive))
                                }
                                class="flex items-center gap-2 flex-1"
                            >
                                {
                                    if *show_sensitive {
                                        html! {
                                            <>
                                                <lucide_yew::EyeOff class="w-4 h-4" />
                                                <span>{ language_ctx.t("key_recovery_hide_data") }</span>
                                            </>
                                        }
                                    } else {
                                        html! {
                                            <>
                                                <lucide_yew::Eye class="w-4 h-4" />
                                                <span>{ language_ctx.t("key_recovery_show_data") }</span>
                                            </>
                                        }
                                    }
                                }
                            </Button>
                        }
                    } else {
                        html! {}
                    }
                }
                <Button
                    onclick={delete_key}
                    variant={ButtonVariant::Destructive}
                    class="flex items-center gap-2 flex-1"
                >
                    <span>{ language_ctx.t("key_recovery_delete") }</span>
                    <lucide_yew::Trash2 class="w-4 h-4" />
                </Button>
            </div>

            <div class="space-y-6">
                // Public Key Section
                <div class="space-y-2">
                    <h3 class="text-lg font-medium text-muted">{ language_ctx.t("key_recovery_public_key") }</h3>
                    <div class="bg-muted p-4 rounded-lg flex gap-3">
                        <pre class="max-w-x16 truncate text-sm text-muted-foreground select-all">
                            {if pubkey.is_empty() {
                                "Loading..."
                            } else {
                                // format!("{}...{}", &pubkey[0..8], &pubkey[pubkey.len() - 8..])
                                // &*pubkey
                                if *pubkey_copied {
                                    &notification_text
                                } else {
                                    &*pubkey
                                }
                            }}
                        </pre>
                        {if !*pubkey_copied {
                            html! {
                                <button
                                    onclick={onclick_copy_pubkey}
                                    class="hover:bg-muted hover:text-primary rounded-lg transition-colors"
                                    title={ language_ctx.t("key_recovery_copy_public_key") }
                                >
                                    <lucide_yew::Copy class="w-5 h-5 text-muted-foreground" />
                                </button>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </div>

                // Private Key Section with warning
                <div class="space-y-2">
                    <h3 class="text-lg font-medium text-muted">{ language_ctx.t("key_recovery_private_key") }</h3>

                    {
                        // if *is_extension {
                        //     html! {
                        //         <div class="bg-muted p-4 rounded-lg">
                        //             <div class="flex items-center text-gray-700 space-x-2">
                        //                 <lucide_yew::TriangleAlert class="text-amber-500 w-5 h-5 flex-shrink-0" />
                        //                 // <p>{ language_ctx.t("key_recovery_no_private_key") }</p>
                        //             </div>
                        //         </div>
                        //     }
                        // } else
                        if *show_sensitive {
                            html! {
                                <div class="bg-muted p-4 rounded-lg overflow-x-auto flex gap-3">
                                    // <pre class="text-sm text-gray-800 whitespace-pre-wrap break-all select-all">
                                    <pre class="text-sm text-muted-foreground truncate select-all ">
                                        // {&*priv_key}
                                        {if priv_key.is_empty() {
                                            "Loading..."
                                        } else if *priv_key_copied {
                                            &notification_text
                                        } else {
                                            &*priv_key
                                        }}
                                    </pre>
                                    {if !*priv_key_copied {
                                        html! {
                                            <button
                                                onclick={onclick_copy_privkey}
                                                class="hover:bg-muted hover:text-primary rounded-lg transition-colors"
                                                title={ language_ctx.t("key_recovery_copy_private_key") }
                                            >
                                                <lucide_yew::Copy class="w-5 h-5 text-muted-foreground" />
                                            </button>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                            }
                        } else {
                            html! {
                                <div class="bg-muted p-4 rounded-lg">
                                    <div class="text-muted-foreground italic">
                                        { language_ctx.t("key_recovery_hidden") }
                                    </div>
                                </div>
                            }
                        }
                    }
                </div>
                // Recovery Phrase Section
                <div class="space-y-2">
                    <h3 class="text-lg font-medium text-muted">{ language_ctx.t("key_recovery_recovery_phrase") }</h3>
                    {
                        // if *is_extension {
                        //     html! {
                        //         <div class="bg-muted p-4 rounded-lg">
                        //             <div class="flex items-center text-gray-700 space-x-2">
                        //                 <lucide_yew::TriangleAlert class="text-amber-500 w-5 h-5 flex-shrink-0" />
                        //     //            <p>{ language_ctx.t("key_recovery_extension_warning") }</p>
                        //             </div>
                        //         </div>
                        //     }
                        // } else
                        if *show_sensitive {
                            if recovery_phrase.is_empty() {
                                html! {
                                    <div class="bg-muted p-4 rounded-lg">
                                        <div class="flex items-center text-muted-foreground space-x-2">
                                            <lucide_yew::TriangleAlert class="text-amber-500 w-5 h-5 flex-shrink-0" />
                                            <p>{ language_ctx.t("key_recovery_no_phrase") }</p>
                                        </div>
                                    </div>
                                }
                            } else if *mnemonic_copied {
                                html! {
                                    <div class="bg-muted p-4 rounded-lg">
                                        <div class="text-muted-foreground italic">
                                            { language_ctx.t("notification_copied_to_clipboard") }
                                        </div>
                                    </div>
                                }
                            }
                            else {
                                html! {
                                    <div class="bg-muted p-4 rounded-lg relative text-xs">
                                        <div class="grid grid-cols-2 md:grid-cols-3 gap-2 pr-5 pb-2">
                                            {
                                                recovery_phrase.iter().enumerate().map(|(i, word)| {
                                                    html! {
                                                        <div class="flex text-muted-foreground items-center">
                                                            <span class="w-6 text-right mr-2">{format!("{}.", i + 1)}</span>
                                                            <span class="font-mono bg-white px-2 py-1 rounded flex-grow">{word}</span>
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                        <button
                                            onclick={onclick_copy_phrase}
                                            class="absolute top-2 right-2 p-2 hover:bg-muted hover:text-primary rounded-lg transition-colors"
                                            title={ language_ctx.t("key_recovery_copy_recovery_phrase") }
                                        >
                                            <lucide_yew::Copy class="w-5 h-5 text-muted-foreground" />
                                        </button>
                                    </div>
                                }
                            }
                        } else {
                            html! {
                                <div class="bg-muted p-4 rounded-lg">
                                    <div class="text-muted-foreground italic">
                                        { language_ctx.t("key_recovery_hidden") }
                                    </div>
                                </div>
                            }
                        }
                    }
                </div>


            </div>
        </div>
    }
}

#[function_component(RelayManagementPage)]
pub fn relay_management_page() -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    let new_relay_url = use_state(String::new);
    let relay_ctx = nostr_minions::relay_pool::use_nostr_relay_pool();

    // Loading relays from IndexedDB on component mount
    let relays = relay_ctx.relay_health();

    let add_relay = {
        let relays = relays.clone();
        let new_relay_url = new_relay_url.clone();
        let relay_ctx = relay_ctx.clone();

        Callback::from(move |_: MouseEvent| {
            let mut url = (*new_relay_url).clone();
            if url.trim().is_empty() {
                return;
            }
            if !url.starts_with("wss://") {
                url = format!("wss://{}", url.trim());
            }

            let relays = relays.clone();

            // Check if relay already exists
            if relays.contains_key(url.trim()) {
                nostr_minions::widgets::toastify::ToastifyOptions::new_failure(
                    "Relay already exists",
                )
                .show();
                return;
            }

            let new_relay = nostr_minions::relay_pool::UserRelay {
                url: url.trim().to_string(),
                read: true,
                write: true,
            };
            relay_ctx.dispatch(nostr_minions::relay_pool::NostrRelayPoolAction::AddRelay(
                new_relay.clone(),
            ));
            nostr_minions::widgets::toastify::ToastifyOptions::new_success(
                "Relay added successfully",
            )
            .show();
        })
    };

    let remove_relay = {
        let relay_ctx = relay_ctx.clone();

        Callback::from(move |url: String| {
            let relay_to_delete = nostr_minions::relay_pool::UserRelay {
                url: url.clone(),
                read: true,
                write: true,
            };
            relay_ctx.dispatch(
                nostr_minions::relay_pool::NostrRelayPoolAction::RemoveRelay(
                    relay_to_delete.clone(),
                ),
            );
            yew::platform::spawn_local(async move {
                if relay_to_delete.delete_from_store().await.is_err() {
                    web_sys::console::log_1(&format!("Failed to delete relay: {}", url).into());
                } else {
                    nostr_minions::widgets::toastify::ToastifyOptions::new_success(
                        "Relay removed successfully",
                    )
                    .show();
                }
            });
        })
    };

    let on_url_input = {
        let new_relay_url = new_relay_url.clone();
        Callback::from(move |value: String| {
            new_relay_url.set(value);
        })
    };

    html! {
            <Card class="max-w-md h-fit">
                <CardHeader>
                    <CardTitle>{ language_ctx.t("relay_add_new") }</CardTitle>
                </CardHeader>
                <CardContent>
                    <div class="flex gap-2">
                        <Input
                            r#type={shady_minions::ui::InputType::Text}
                            placeholder={language_ctx.t("relay_placeholder")}
                            value={(*new_relay_url).clone()}
                            oninput={on_url_input}
                            class="flex-1"
                        />
                        <Button
                            onclick={add_relay}
                            disabled={(*new_relay_url).trim().is_empty()}
                        >
                            <lucide_yew::Plus class="size-4" />
                        </Button>
                    </div>
                </CardContent>
                <CardHeader>
                    <CardTitle>{ language_ctx.t("relay_connected_relays") }</CardTitle>
                </CardHeader>
                <CardContent>
                    {if relays.is_empty() {
                        html! {
                            <div class="text-center py-8 text-muted-foreground">
                                <lucide_yew::Wifi class="w-12 h-12 mx-auto mb-2 opacity-50" />
                                <p>{ language_ctx.t("relay_no_configured") }</p>
                                <p class="text-sm">{ language_ctx.t("relay_add_hint") }</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="space-y-3">
                                {for relays.iter().map(|(url, relay)| {
                                    let url = url.clone();
                                    let remove_callback = {
                                        let remove_relay = remove_relay.clone();
                                        let url = url.clone();
                                        Callback::from(move |_| remove_relay.emit(url.clone()))
                                    };

                                    html! {
                                        <RelayItem
                                            url={url.clone()}
                                            relay={*relay}
                                            on_remove={remove_callback}
                                        />
                                    }
                                })}
                            </div>
                        }
                    }}
                </CardContent>
            </Card>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RelayItemProps {
    pub url: String,
    pub relay: nostr_minions::relay_pool::ReadyState,
    pub on_remove: Callback<MouseEvent>,
}

#[function_component(RelayItem)]
pub fn relay_item(props: &RelayItemProps) -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    let (status_color, status_text, status_icon) = match props.relay {
        nostr_minions::relay_pool::ReadyState::CONNECTING => (
            "text-yellow-500",
            language_ctx.t("relay_status_connecting"),
            "⏳",
        ),
        nostr_minions::relay_pool::ReadyState::OPEN => (
            "text-green-500",
            language_ctx.t("relay_status_connected"),
            "✅",
        ),
        nostr_minions::relay_pool::ReadyState::CLOSING => (
            "text-orange-500",
            language_ctx.t("relay_status_closing"),
            "⏳",
        ),
        nostr_minions::relay_pool::ReadyState::CLOSED => {
            ("text-red-500", language_ctx.t("relay_status_closed"), "❌")
        }
    };

    html! {
        <div class="flex items-center justify-between p-3 border border-border rounded-lg">
            <div class="flex items-center space-x-3 flex-1 min-w-0">
                <div class="flex items-center space-x-2">
                    <span class="text-lg">{status_icon}</span>
                    <div class="min-w-0 flex-1">
                        <p class="text-sm font-medium truncate">{&props.url}</p>
                        <div class="flex items-center space-x-4 text-xs text-muted">
                            <span class={classes!("font-medium", status_color)}>{status_text}</span>
                            // <span>{if props.relay.read { "📖 Read" } else { "" }}</span>
                            // <span>{if props.relay.write { "✏️ Write" } else { "" }}</span>
                        </div>
                    </div>
                </div>
            </div>
            <Button
                variant={ButtonVariant::Outline}
                onclick={props.on_remove.clone()}
                class="ml-2 px-3 py-1 text-red-600 border-red-200 hover:bg-red-50"
            >
                <lucide_yew::Trash2 class="w-4 h-4" />
            </Button>
        </div>
    }
}

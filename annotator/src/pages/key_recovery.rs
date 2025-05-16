use crate::contexts::language::use_language_ctx;
use lucide_yew::{ArrowLeft, Copy as CopyIcon, Eye, EyeOff, Key, Trash2, TriangleAlert};
use nostr_minions::key_manager::{NostrIdAction, NostrIdStore};
use shady_minions::ui::{Button, Card, CardContent, CardHeader, CardTitle};
use web_sys::window;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::router::AnnotatorRoute;

#[function_component(KeyRecoveryPage)]
pub fn key_recovery_page() -> Html {
    let navigator = use_navigator().unwrap();
    let language_ctx = use_language_ctx();

    let go_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&AnnotatorRoute::Home))
    };

    html! {
        <div class="container mx-auto p-6">
            <div class="mb-4">
                <Button
                    onclick={go_back}
                    class="flex items-center gap-2"
                >
                    <ArrowLeft class="w-4 h-4" />
                    <span>{ language_ctx.t("key_recovery_back") }</span>
                </Button>
            </div>
            <Card>
                <CardHeader>
                    <CardTitle>
                        <div class="flex items-center space-x-3 pb-2">
                            <Key class="text-primary w-6 h-6" />
                            <h3 class="text-2xl font-bold">{ language_ctx.t("key_recovery_title") }</h3>
                        </div>
                    </CardTitle>
                </CardHeader>
                <CardContent class="space-y-6">
                    <KeyRecoverySection />
                </CardContent>
            </Card>
        </div>
    }
}

#[function_component(KeyRecoverySection)]
fn key_recovery_section() -> Html {
    let key_ctx = use_context::<NostrIdStore>().expect("No NostrIdStore found");
    let language_ctx = use_language_ctx();

    // State for showing/hiding sensitive data
    let show_sensitive = use_state(|| false);

    // State for private key and recovery phrase
    let priv_key = use_state(|| "".to_string());
    let recovery_phrase = use_state(Vec::<String>::new);

    // Check if user is using extension-based identity
    let is_extension = use_state(|| false);

    // Fetch key information and determine if user is using extension
    let priv_key_handle = priv_key.clone();
    let recovery_phrase_handle = recovery_phrase.clone();
    let is_extension_handle = is_extension.clone();
    use_effect_with(key_ctx.clone(), move |key_handle| {
        let priv_key_handle = priv_key_handle.clone();
        let recovery_phrase_handle = recovery_phrase_handle.clone();
        let is_extension_handle = is_extension_handle.clone();
        let key_handle = key_handle.clone();

        spawn_local(async move {
            // Check if identity is extension-based by attempting to get the key
            if let Some(_identity) = key_handle.get_identity() {
                // Try to get the key - if it returns None or fails, it's likely an extension
                match key_handle.get_nostr_key().await {
                    Some(_) => {
                        is_extension_handle.set(false);
                    }
                    None => {
                        is_extension_handle.set(true);
                        return;
                    }
                }
            }

            if let Some(mut key) = key_handle.get_nostr_key().await {
                // Set key as extractable to access sensitive data
                key.set_extractable(true);

                // Get private key as hex
                let secret_key = key.secret_key();
                let mut hex_secret = String::with_capacity(secret_key.len() * 2);
                secret_key.iter().fold(&mut hex_secret, |acc, byte| {
                    use std::fmt::Write;
                    let _ = write!(acc, "{:02x}", byte);
                    acc
                });
                priv_key_handle.set(hex_secret);

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
        let lang_ctx = language_ctx.clone();
        Callback::from(move |_| {
            if let Some(window) = window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                let _ = clipboard.write_text(&secret_key);
                web_sys::console::log_1(&lang_ctx.t("key_recovery_copy_private_key").into());
            }
        })
    };

    let onclick_copy_phrase = {
        let phrase = recovery_phrase.clone();
        let lang_ctx = language_ctx.clone();
        Callback::from(move |_| {
            if let Some(window) = window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                let phrase_text = phrase.join(" ");
                let _ = clipboard.write_text(&phrase_text);
                web_sys::console::log_1(&lang_ctx.t("key_recovery_copy_recovery_phrase").into());
            }
        })
    };

    let pubkey = match key_ctx.get_pubkey() {
        Some(pk) => pk,
        None => "".to_string(),
    };

    let onclick_copy_pubkey = {
        let pubkey = pubkey.clone();
        let lang_ctx = language_ctx.clone();
        Callback::from(move |_| {
            if let Some(window) = window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                let _ = clipboard.write_text(&pubkey);
                web_sys::console::log_1(&lang_ctx.t("key_recovery_copy_public_key").into());
            }
        })
    };

    let delete_key = {
        let key_handle = key_ctx.dispatcher();
        let lang_ctx = language_ctx.clone();
        Callback::from(move |_| {
            if let Ok(confirmed) = window()
                .expect("No window found")
                .confirm_with_message(&lang_ctx.t("key_recovery_delete_confirm"))
            {
                if confirmed {
                    // First dispatch the DeleteIdentity action
                    key_handle.dispatch(NostrIdAction::DeleteIdentity);

                    // JavaScript interop to delete from IndexedDB directly
                    if let Some(window) = window() {
                        let document = window.document().expect("Document not found");
                        let script = document
                            .create_element("script")
                            .expect("Could not create script element");

                        // Script to delete from indexedDB
                        // Use raw string with fixed messages for simplicity
                        let deletion_script = r#"
                            (function() {
                                // Delete from the nostr_db database
                                let deleteRequest = indexedDB.deleteDatabase('nostr_db');
                                deleteRequest.onsuccess = function() {
                                    console.log("Successfully deleted nostr_db database");
                                    
                                    // Reload the page after deletion
                                    window.location.reload();
                                };
                                deleteRequest.onerror = function() {
                                    console.error("Error deleting nostr_db database");
                                };
                                
                                // Also clear localStorage for good measure
                                localStorage.clear();
                                sessionStorage.clear();
                            })();
                        "#;

                        script.set_text_content(Some(deletion_script));
                        let _ = document
                            .body()
                            .expect("Body not found")
                            .append_child(&script);
                    }
                }
            }
        })
    };

    html! {
        <div class="space-y-6 max-h-[82vh] overflow-y-auto pb-6">
            // Delete key button
            <div class="flex justify-end">
                <Button
                    onclick={delete_key}
                    class="flex items-center gap-2 bg-red-600 text-white hover:bg-red-700"
                >
                    <span>{ language_ctx.t("key_recovery_delete") }</span>
                    <Trash2 class="w-4 h-4" />
                </Button>
            </div>

            <div class="space-y-6">
                // Public Key Section
                <div class="space-y-2">
                    <h3 class="text-lg font-medium text-gray-700">{ language_ctx.t("key_recovery_public_key") }</h3>
                    <div class="bg-gray-100 p-4 rounded-lg overflow-x-auto relative">
                        <pre class="text-sm text-gray-800 whitespace-pre-wrap break-all select-all">
                            {&pubkey}
                        </pre>
                        <button
                            onclick={onclick_copy_pubkey}
                            class="absolute top-2 right-2 p-2 hover:bg-gray-200 hover:text-primary rounded-lg transition-colors"
                            title={ language_ctx.t("key_recovery_copy_public_key") }
                        >
                            <CopyIcon class="w-5 h-5 text-gray-600" />
                        </button>
                    </div>
                </div>

                // Recovery Phrase Section
                <div class="space-y-2">
                    <h3 class="text-lg font-medium text-gray-700">{ language_ctx.t("key_recovery_recovery_phrase") }</h3>
                    {
                        if *is_extension {
                            html! {
                                <div class="bg-gray-100 p-4 rounded-lg">
                                    <div class="flex items-center text-gray-700 space-x-2">
                                        <TriangleAlert class="text-amber-500 w-5 h-5 flex-shrink-0" />
                                        <p>{ language_ctx.t("key_recovery_extension_warning") }</p>
                                    </div>
                                </div>
                            }
                        } else if *show_sensitive {
                            if recovery_phrase.len() == 0 {
                                html! {
                                    <div class="bg-gray-100 p-4 rounded-lg">
                                        <div class="flex items-center text-gray-700 space-x-2">
                                            <TriangleAlert class="text-amber-500 w-5 h-5 flex-shrink-0" />
                                            <p>{ language_ctx.t("key_recovery_no_phrase") }</p>
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="bg-gray-100 p-4 rounded-lg relative">
                                        <div class="grid grid-cols-2 md:grid-cols-3 gap-2 pb-2">
                                            {
                                                recovery_phrase.iter().enumerate().map(|(i, word)| {
                                                    html! {
                                                        <div class="flex items-center">
                                                            <span class="w-6 text-gray-500 text-right mr-2">{format!("{}.", i + 1)}</span>
                                                            <span class="font-mono bg-white px-2 py-1 rounded flex-grow">{word}</span>
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                        <button
                                            onclick={onclick_copy_phrase}
                                            class="absolute top-2 right-2 p-2 hover:bg-gray-200 hover:text-primary rounded-lg transition-colors"
                                            title={ language_ctx.t("key_recovery_copy_recovery_phrase") }
                                        >
                                            <CopyIcon class="w-5 h-5 text-gray-600" />
                                        </button>
                                    </div>
                                }
                            }
                        } else {
                            html! {
                                <div class="bg-gray-100 p-4 rounded-lg">
                                    <div class="text-gray-700 italic">
                                        { language_ctx.t("key_recovery_hidden") }
                                    </div>
                                </div>
                            }
                        }
                    }
                </div>

                // Private Key Section with warning
                <div class="space-y-2">
                    <h3 class="text-lg font-medium text-gray-700">{ language_ctx.t("key_recovery_private_key") }</h3>

                    {
                        if *is_extension {
                            html! {
                                <div class="bg-gray-100 p-4 rounded-lg">
                                    <div class="flex items-center text-gray-700 space-x-2">
                                        <TriangleAlert class="text-amber-500 w-5 h-5 flex-shrink-0" />
                                        <p>{ language_ctx.t("key_recovery_no_private_key") }</p>
                                    </div>
                                </div>
                            }
                        } else if *show_sensitive {
                            html! {
                                <>
                                    <div class="flex items-start space-x-3">
                                        <TriangleAlert class="text-yellow-500 w-5 h-5 mt-1 flex-shrink-0" />
                                        <p class="text-gray-700">
                                            { language_ctx.t("key_recovery_use_key") }
                                        </p>
                                    </div>

                                    <div class="bg-gray-100 p-4 rounded-lg overflow-x-auto relative">
                                        <pre class="text-sm text-gray-800 whitespace-pre-wrap break-all select-all">
                                            {&*priv_key}
                                        </pre>
                                        <button
                                            onclick={onclick_copy_privkey}
                                            class="absolute top-2 right-2 p-2 hover:bg-gray-200 hover:text-primary rounded-lg transition-colors"
                                            title={ language_ctx.t("key_recovery_copy_private_key") }
                                        >
                                            <CopyIcon class="w-5 h-5 text-gray-600" />
                                        </button>
                                    </div>

                                    <div class="flex items-start space-x-3">
                                        <TriangleAlert class="text-yellow-500 w-5 h-5 mt-1 flex-shrink-0" />
                                        <p class="text-sm text-gray-600">
                                            { language_ctx.t("key_recovery_keep_safe") }
                                        </p>
                                    </div>
                                </>
                            }
                        } else {
                            html! {
                                <>
                                    <div class="flex items-start space-x-3">
                                        <TriangleAlert class="text-yellow-500 w-5 h-5 mt-1 flex-shrink-0" />
                                        <p class="text-gray-700">
                                            { language_ctx.t("key_recovery_use_key") }
                                        </p>
                                    </div>

                                    <div class="bg-gray-100 p-4 rounded-lg">
                                        <div class="text-gray-700 italic">
                                            { language_ctx.t("key_recovery_hidden") }
                                        </div>
                                    </div>
                                </>
                            }
                        }
                    }
                </div>

                // Show/Hide Toggle Button
                {
                    if !(*is_extension) {
                        html! {
                            <div class="mt-4">
                                <Button
                                    onclick={
                                        let show_sensitive = show_sensitive.clone();
                                        Callback::from(move |_| show_sensitive.set(!*show_sensitive))
                                    }
                                    class="flex items-center gap-2"
                                >
                                    {
                                        if *show_sensitive {
                                            html! {
                                                <>
                                                    <EyeOff class="w-4 h-4" />
                                                    <span>{ language_ctx.t("key_recovery_hide_data") }</span>
                                                </>
                                            }
                                        } else {
                                            html! {
                                                <>
                                                    <Eye class="w-4 h-4" />
                                                    <span>{ language_ctx.t("key_recovery_show_data") }</span>
                                                </>
                                            }
                                        }
                                    }
                                </Button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}

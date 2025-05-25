use crate::router::AnnotatorRoute;
use nostr_minions::browser_api::IdbStoreManager;
use shady_minions::ui::{Button, ButtonVariant, Card, CardContent, CardHeader, CardTitle, Input};
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(RelayManagementPage)]
pub fn relay_management_page() -> Html {
    let navigator = use_navigator().unwrap();
    let relays = use_state(Vec::<nostr_minions::relay_pool::UserRelay>::new);
    let new_relay_url = use_state(String::new);
    let is_loading = use_state(|| false);

    // Loading relays from IndexedDB on component mount
    {
        let relays = relays.clone();
        use_effect_with((), move |_| {
            let relays = relays.clone();
            yew::platform::spawn_local(async move {
                match nostr_minions::relay_pool::UserRelay::retrieve_all_from_store().await {
                    Ok(saved_relays) => {
                        if saved_relays.is_empty() {
                            // If no relays found in IDB, use default relays and save them
                            let default_relays = vec![
                                nostr_minions::relay_pool::UserRelay {
                                    url: "wss://purplepag.es".to_string(),
                                    read: true,
                                    write: true,
                                },
                                nostr_minions::relay_pool::UserRelay {
                                    url: "wss://relay.unkownk.com".to_string(),
                                    read: true,
                                    write: true,
                                },
                                nostr_minions::relay_pool::UserRelay {
                                    url: "wss://nos.lol".to_string(),
                                    read: true,
                                    write: true,
                                },
                                nostr_minions::relay_pool::UserRelay {
                                    url: "wss://relay.illuminodes.com".to_string(),
                                    read: true,
                                    write: true,
                                },
                            ];

                            // Save default relays to IDB
                            for relay in &default_relays {
                                if let Err(e) = relay.clone().save_to_store().await {
                                    web_sys::console::log_1(
                                        &format!(
                                            "Failed to save default relay {}: {:?}",
                                            relay.url, e
                                        )
                                        .into(),
                                    );
                                }
                            }

                            relays.set(default_relays);
                            web_sys::console::log_1(
                                &"Initialized with default relays and saved to IDB".into(),
                            );
                        } else {
                            relays.set(saved_relays);
                            web_sys::console::log_1(&"Loaded relays from IDB".into());
                        }
                    }
                    Err(e) => {
                        web_sys::console::log_1(
                            &format!("Failed to retrieve relays from IDB: {:?}", e).into(),
                        );
                        // Fall back to default relays if IDB fails
                        let default_relays = vec![
                            nostr_minions::relay_pool::UserRelay {
                                url: "wss://purplepag.es".to_string(),
                                read: true,
                                write: true,
                            },
                            nostr_minions::relay_pool::UserRelay {
                                url: "wss://relay.unkownk.com".to_string(),
                                read: true,
                                write: true,
                            },
                            nostr_minions::relay_pool::UserRelay {
                                url: "wss://nos.lol".to_string(),
                                read: true,
                                write: true,
                            },
                            nostr_minions::relay_pool::UserRelay {
                                url: "wss://relay.illuminodes.com".to_string(),
                                read: true,
                                write: true,
                            },
                        ];
                        relays.set(default_relays);
                    }
                }
            });
            || {}
        });
    }

    let add_relay = {
        let relays = relays.clone();
        let new_relay_url = new_relay_url.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |_: MouseEvent| {
            let url = (*new_relay_url).clone();
            if url.trim().is_empty() || !url.starts_with("wss://") {
                return;
            }

            let relays = relays.clone();
            let new_relay_url = new_relay_url.clone();
            let is_loading = is_loading.clone();

            is_loading.set(true);

            // Check if relay already exists
            let mut current_relays = (*relays).clone();
            if current_relays.iter().any(|r| r.url == url.trim()) {
                web_sys::console::log_1(&"Relay already exists".into());
                is_loading.set(false);
                return;
            }

            let new_relay = nostr_minions::relay_pool::UserRelay {
                url: url.trim().to_string(),
                read: true,
                write: true,
            };

            // Save to IndexedDB and update state
            yew::platform::spawn_local(async move {
                let new_relay_clone = new_relay.clone();
                match new_relay.save_to_store().await {
                    Ok(_) => {
                        web_sys::console::log_1(
                            &format!("Relay {} saved to IDB successfully", new_relay_clone.url)
                                .into(),
                        );

                        // Add to current list
                        current_relays.push(new_relay_clone);
                        relays.set(current_relays);
                        new_relay_url.set(String::new());
                        is_loading.set(false);

                        web_sys::console::log_1(&"Relay added successfully".into());
                    }
                    Err(e) => {
                        web_sys::console::log_1(
                            &format!("Failed to save relay to IDB: {:?}", e).into(),
                        );

                        // Still add to UI even if IDB save fails
                        current_relays.push(new_relay_clone);
                        relays.set(current_relays);
                        new_relay_url.set(String::new());
                        is_loading.set(false);

                        web_sys::console::log_1(&"Relay added to UI (IDB save failed)".into());
                    }
                }
            });
        })
    };

    let remove_relay = {
        let relays = relays.clone();

        Callback::from(move |url: String| {
            let relays = relays.clone();
            let url_clone = url.clone();

            // Remove from IndexedDB and update state
            yew::platform::spawn_local(async move {
                // Create a relay instance for deletion
                let relay_to_delete = nostr_minions::relay_pool::UserRelay {
                    url: url_clone.clone(),
                    read: true,
                    write: true,
                };

                match relay_to_delete.delete_from_store().await {
                    Ok(_) => {
                        web_sys::console::log_1(
                            &format!("Relay {} deleted from IDB successfully", url_clone).into(),
                        );
                    }
                    Err(e) => {
                        web_sys::console::log_1(
                            &format!("Failed to delete relay from IDB: {:?}", e).into(),
                        );
                    }
                }

                // Remove from current list regardless of IDB operation result
                let current_relays: Vec<_> = (*relays)
                    .iter()
                    .filter(|r| r.url != url_clone)
                    .cloned()
                    .collect();
                relays.set(current_relays);

                web_sys::console::log_1(&"Relay removed successfully".into());
            });
        })
    };

    let on_url_input = {
        let new_relay_url = new_relay_url.clone();
        Callback::from(move |value: String| {
            new_relay_url.set(value);
        })
    };

    let go_back = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&AnnotatorRoute::Home);
        })
    };

    html! {
        <div class="container mx-auto p-4 max-w-4xl">
            // Header with back button
            <div class="flex items-center mb-6">
                <Button
                    onclick={go_back}
                    variant={ButtonVariant::Outline}
                    class="mr-4 px-3 py-2"
                >
                    <lucide_yew::ArrowLeft class="w-4 h-4 mr-1" />
                    {"Back"}
                </Button>
                <div class="flex-1">
                    <h1 class="text-2xl font-bold mb-2">{"Relay Management"}</h1>
                    <p class="text-muted-foreground">
                        {"Manage your Nostr relay connections. Add, remove, and monitor relay status."}
                    </p>
                </div>
            </div>

            // Add new relay section
            <Card class="mb-6">
                <CardHeader>
                    <CardTitle>{"Add New Relay"}</CardTitle>
                </CardHeader>
                <CardContent>
                    <div class="flex gap-2">
                        <Input
                            r#type={shady_minions::ui::InputType::Text}
                            placeholder="wss://relay.example.com"
                            value={(*new_relay_url).clone()}
                            oninput={on_url_input}
                            class="flex-1"
                        />
                        <Button
                            onclick={add_relay}
                            disabled={(*new_relay_url).trim().is_empty() || !(*new_relay_url).starts_with("wss://") || *is_loading}
                        >
                            {if *is_loading { "Adding..." } else { "Add Relay" }}
                        </Button>
                    </div>
                    {if !(*new_relay_url).trim().is_empty() && !(*new_relay_url).starts_with("wss://") {
                        html! {
                            <p class="text-red-500 text-sm mt-2">{"Relay URL must start with wss://"}</p>
                        }
                    } else {
                        html! {}
                    }}
                </CardContent>
            </Card>

            // Relay list section
            <Card>
                <CardHeader>
                    <CardTitle>{"Connected Relays"}</CardTitle>
                </CardHeader>
                <CardContent>
                    {if relays.is_empty() {
                        html! {
                            <div class="text-center py-8 text-muted-foreground">
                                <lucide_yew::Wifi class="w-12 h-12 mx-auto mb-2 opacity-50" />
                                <p>{"No relays configured"}</p>
                                <p class="text-sm">{"Add a relay above to get started"}</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="space-y-3">
                                {for relays.iter().map(|relay| {
                                    let url = relay.url.clone();
                                    let remove_callback = {
                                        let remove_relay = remove_relay.clone();
                                        let url = url.clone();
                                        Callback::from(move |_| remove_relay.emit(url.clone()))
                                    };

                                    html! {
                                        <RelayItem
                                            relay={relay.clone()}
                                            on_remove={remove_callback}
                                        />
                                    }
                                })}
                            </div>
                        }
                    }}
                </CardContent>
            </Card>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RelayItemProps {
    pub relay: nostr_minions::relay_pool::UserRelay,
    pub on_remove: Callback<MouseEvent>,
}

#[function_component(RelayItem)]
pub fn relay_item(props: &RelayItemProps) -> Html {
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>();
    let connection_status = use_state(|| nostr_minions::relay_pool::ReadyState::CONNECTING);

    // Monitor relay status
    {
        let connection_status = connection_status.clone();
        let relay_url = props.relay.url.clone();

        use_effect_with(relay_ctx.clone(), move |relay_ctx| {
            if let Some(ctx) = relay_ctx.clone() {
                let connection_status = connection_status.clone();
                let relay_url = relay_url.clone();

                yew::platform::spawn_local(async move {
                    loop {
                        gloo::timers::future::sleep(std::time::Duration::from_secs(2)).await;
                        let health = ctx.relay_health();
                        if let Some(status) = health.get(&relay_url) {
                            connection_status.set(*status);
                        }
                    }
                });
            }
            || {}
        });
    }

    let (status_color, status_text, status_icon) = match *connection_status {
        nostr_minions::relay_pool::ReadyState::CONNECTING => {
            ("text-yellow-500", "Connecting", "⏳")
        }
        nostr_minions::relay_pool::ReadyState::OPEN => ("text-green-500", "Connected", "✅"),
        nostr_minions::relay_pool::ReadyState::CLOSING => {
            ("text-orange-500", "Disconnecting", "⏳")
        }
        nostr_minions::relay_pool::ReadyState::CLOSED => ("text-red-500", "Disconnected", "❌"),
    };

    html! {
        <div class="flex items-center justify-between p-3 border rounded-lg bg-white">
            <div class="flex items-center space-x-3 flex-1 min-w-0">
                <div class="flex items-center space-x-2">
                    <span class="text-lg">{status_icon}</span>
                    <div class="min-w-0 flex-1">
                        <p class="text-sm font-medium truncate">{&props.relay.url}</p>
                        <div class="flex items-center space-x-4 text-xs text-muted-foreground">
                            <span class={classes!("font-medium", status_color)}>{status_text}</span>
                            <span>{if props.relay.read { "📖 Read" } else { "" }}</span>
                            <span>{if props.relay.write { "✏️ Write" } else { "" }}</span>
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

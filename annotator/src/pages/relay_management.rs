use nostr_minions::browser_api::IdbStoreManager;
use shady_minions::ui::{Button, ButtonVariant, Card, CardContent, CardHeader, CardTitle, Input};
use web_sys::MouseEvent;
use yew::prelude::*;

#[function_component(RelayManagementPage)]
pub fn relay_management_page() -> Html {
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
        <>
            <yew_router::components::Link<crate::router::AnnotatorRoute>
                to={crate::router::AnnotatorRoute::Home}>
                <Button
                    class="fixed top-4 left-4 z-50"
                    variant={shady_minions::ui::ButtonVariant::Outline}
                    size={shady_minions::ui::ButtonSize::Small}
                    >
                    <lucide_yew::ArrowLeft class="size-4" />
                </Button>
            </yew_router::components::Link<crate::router::AnnotatorRoute>>
            <Card class="max-w-sm h-fit mx-auto mt-16">
                <CardHeader class="flex items-center gap-4">
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
                            disabled={(*new_relay_url).trim().is_empty()}
                        >
                            <lucide_yew::Plus class="size-4" />
                        </Button>
                    </div>
                </CardContent>
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
            </>
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
    let (status_color, status_text, status_icon) = match props.relay {
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

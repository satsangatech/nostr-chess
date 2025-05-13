use nostr_minions::browser_api::IdbStoreManager;
use yew::prelude::*;
fn main() {
    yew::Renderer::<App>::new().render();
}
#[function_component(App)]
fn app() -> Html {
    let relays = use_state(|| {
        vec![
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
            // nostr_minions::relay_pool::UserRelay {
            //     url: "wss://relay.arrakis.lat".to_string(),
            //     read: true,
            //     write: true,
            // },
        ]
    });
    {
        let relays = relays.clone();
        use_effect_with((), move |_| {
            nostr_minions::init_nostr_db().unwrap();
            yew::platform::spawn_local(async move {
                let Ok(saved_relays) =
                    nostr_minions::relay_pool::UserRelay::retrieve_all_from_store().await
                else {
                    web_sys::console::log_1(&"Failed to retrieve relays".into());
                    return;
                };
                relays.set(saved_relays);
            });
            || {}
        });
    }

    html! {
        <yew_router::BrowserRouter>
            <nostr_minions::key_manager::NostrIdProvider>
                <nostr_minions::relay_pool::NostrRelayPoolProvider relays={(*relays).clone()}>
                    <LoginCheck>
                        <bunker::live_game::AnnotatedGameHistoryProvider>

                        <div class={classes!("h-screen", "w-full", "flex")}>
                            <navbar class={classes!("min-w-fit", "bg-primary", "text-primary-foreground")}>
                                <div class={classes!("flex", "flex-col", "items-center", "gap-8", "p-4", "mx-4")}>
                                <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::Home}>
                                    <lucide_yew::House class={classes!("size-10")} />
                                </yew_router::components::Link<bunker::MainRoute>>
                                <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::NewGame}>
                                    <lucide_yew::Plus class={classes!("size-10")} />
                                </yew_router::components::Link<bunker::MainRoute>>
                                <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::MyGames}>
                                    <lucide_yew::BookOpen class={classes!("size-10")} />
                                </yew_router::components::Link<bunker::MainRoute>>
                                <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::Search}>
                                    <lucide_yew::Search class={classes!("size-10")} />
                                </yew_router::components::Link<bunker::MainRoute>>
                                <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::Settings}>
                                    <lucide_yew::Cog class={classes!("size-10")} />
                                </yew_router::components::Link<bunker::MainRoute>>
                                </div>
                            </navbar>
                            <div class={classes!("flex-1", "flex", "flex-col")}>
                                <main class={classes!("flex-1")}>
                                    <bunker::MainPages />
                                </main>
                                <footer class={classes!("min-h-12", "bg-secondary")}>
                                </footer>
                            </div>
                        </div>
                        </bunker::live_game::AnnotatedGameHistoryProvider>

                    </LoginCheck>
                </nostr_minions::relay_pool::NostrRelayPoolProvider>
            </nostr_minions::key_manager::NostrIdProvider>
        </yew_router::BrowserRouter>
    }
}

#[function_component(LoginCheck)]
fn login_check(props: &yew::html::ChildrenProps) -> Html {
    let nostr_key = nostr_minions::key_manager::use_nostr_key();
    match nostr_key {
        Some(_) => html! {
            {props.children.clone()}
        },
        None => {
            html! {
                <div class={"h-screen w-full flex flex-col gap-4 items-center justify-center"}>
                    <bunker::NostrLogin />
                </div>
            }
        }
    }
}

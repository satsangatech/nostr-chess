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
                                <Navbar />
                                <main class={classes!("flex-1")}>
                                    <bunker::MainPages />
                                </main>
                            </div>
                        </bunker::live_game::AnnotatedGameHistoryProvider>
                    </LoginCheck>
                </nostr_minions::relay_pool::NostrRelayPoolProvider>
            </nostr_minions::key_manager::NostrIdProvider>
        </yew_router::BrowserRouter>
    }
}

#[function_component(Navbar)]
fn navbar() -> Html {
    let navbar_button_class = classes!(
        "size-32",
        "flex",
        "items-center",
        "justify-center",
        "p-4",
        "bg-zinc-800",
        "rounded-r-[2vw]",
        "text-white",
        "flex-col",
        "gap-1"
    );
    html! {
        <navbar class={classes!("min-w-fit", "h-full", "flex", "flex-col", "justify-evenly")}>
            <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::Home}>
                <div class={navbar_button_class.clone()}>
                    <img src="/public/img/splashscreen.svg"
                         class={classes!("size-12", "rounded-full")} />
                    <span class={classes!("")}>{"Home"}</span>
                </div>
            </yew_router::components::Link<bunker::MainRoute>>
            <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::NewGame}>
                <div class={navbar_button_class.clone()}>
                    <lucide_yew::Plus class={classes!("size-12")} />
                    <span class={classes!("")}>{"Annotate"}</span>
                </div>
            </yew_router::components::Link<bunker::MainRoute>>
            <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::MyGames}>
                <div class={navbar_button_class.clone()}>
                    <lucide_yew::BookOpen class={classes!("size-12")} />
                    <span class={classes!("")}>{"Repertoire"}</span>
                </div>
            </yew_router::components::Link<bunker::MainRoute>>
            <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::Search}>
                <div class={navbar_button_class.clone()}>
                    <lucide_yew::Search class={classes!("size-12")} />
                    <span class={classes!("")}>{"Search"}</span>
                </div>
            </yew_router::components::Link<bunker::MainRoute>>
            <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::Settings}>
                <div class={navbar_button_class.clone()}>
                    <lucide_yew::Cog class={classes!("size-12")} />
                    <span class={classes!("")}>{"Settings"}</span>
                </div>
            </yew_router::components::Link<bunker::MainRoute>>
        </navbar>
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

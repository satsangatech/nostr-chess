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
        use_effect_with((), move |()| {
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
                <annotator::user_metadata::UserMetadataProvider>
                <annotator::language::LanguageConfigsProvider>
                <annotator::configs::AnnotatorConfigProvider>
                <div class={classes!("h-dvh", "w-dvw")}>
                <LoginCheck>
                        <annotator::live_game::AnnotatedGameProvider>
                            <annotator::AnnotatorRouter />
                        </annotator::live_game::AnnotatedGameProvider>
                </LoginCheck>
                </div>
                </annotator::configs::AnnotatorConfigProvider>
                </annotator::language::LanguageConfigsProvider>
                </annotator::user_metadata::UserMetadataProvider>
            </nostr_minions::relay_pool::NostrRelayPoolProvider>
        </nostr_minions::key_manager::NostrIdProvider>
        </yew_router::BrowserRouter>
    }
}

#[function_component(LoginCheck)]
fn login_check(props: &yew::html::ChildrenProps) -> Html {
    let key_ctx = nostr_minions::key_manager::use_nostr_id_ctx();
    let config_ctx = annotator::configs::use_annotator_config();

    let loaded = key_ctx.loaded() && config_ctx.loaded;
    let nostr_id = key_ctx.get_pubkey();

    if !loaded {
        return html! {
            <SplashScreen />
        };
    }

    let visible = classes!(
        "fixed",
        "inset-0",
        "transition-all",
        "duration-900",
        "z-20",
        "flex",
        "opacity-100",
        "py-8"
    );
    let hidden = classes!(
        "fixed",
        "inset-0",
        "transition-all",
        "duration-900",
        "pointer-events-none",
        "opacity-0"
    );
    // let loading_page_class = classes!(if loaded {
    //     hidden.clone()
    // } else {
    //     visible.clone()
    // },);
    let login_page_class = classes!(if loaded && nostr_id.is_none() {
        visible.clone()
    } else {
        hidden.clone()
    },);
    let children_class = classes!(if loaded && nostr_id.is_some() {
        visible
    } else {
        hidden
    },);
    // web_sys::console::log_1(
    //     &format!("Loaded: {}, Nostr ID: {:?}", loaded, nostr_id.is_some()).into(),
    // );
    html! {
        <>
            <div class={login_page_class}>
                <annotator::language::LanguageConfigsProvider>
                    <annotator::NostrLogin />
                </annotator::language::LanguageConfigsProvider>
            </div>
            <div class={children_class}>
                {props.children.clone()}
            </div>
        </>
    }
}

#[function_component(SplashScreen)]
pub fn splash_screen() -> Html {
    let class = classes!(
        "size-full",
        "flex",
        "flex-col",
        "gap-4",
        "justify-center",
        "items-center",
        "bg-[url(/public/assets/img/splashscreen_bg.png)]",
        "bg-cover",
        "bg-no-repeat",
        "bg-center"
    );
    html! {
        <div {class}>
            <img
                src="/public/assets/img/splashscreen.svg"
                alt="Rooky Logo"
                class={classes!("size-40", "object-contain")}
            />
            <LoadingBar />
        </div>
    }
}

#[function_component(LoadingBar)]
pub fn loading_bar() -> Html {
    html! {
        <div class="w-56 mx-auto h-2 bg-gray-200 rounded-full overflow-hidden">
            <div
                class="h-full w-20 rounded-full animate-loading-bar bg-[#1E06DD]"
            />
        </div>
    }
}

#[function_component(NostrActions)]
pub fn nostr_actions() -> Html {
    let game_ctx =
        use_context::<annotator::live_game::AnnotatedGameStore>().expect("missing game context");
    let game = game_ctx.pgn_game();

    html! {
        <div class={classes!("flex", "flex-col", "gap-2", "justify-between", "items-center")}>
            <annotator::ShareRookyGame game={game.clone()} />
            <annotator::DirectMessageRookyGame game={game.clone()} />
            <annotator::SaveTxtRookyGame game={game.clone()} />
        </div>
    }
}

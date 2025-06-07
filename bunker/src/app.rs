use nostr_minions::browser_api::IdbStoreManager;
use yew::prelude::*;
fn main() {
    yew::Renderer::<App>::new().render();
}
#[function_component(App)]
fn app() -> Html {
    html! {
        <Suspense fallback={html! { <SplashScreen /> }}>
            <AppContext />
        </Suspense>
    }
}

static DEFAULT_RELAYS: std::sync::LazyLock<[nostr_minions::relay_pool::UserRelay; 4]> =
    std::sync::LazyLock::new(|| {
        [
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
        ]
    });
#[function_component(AppContext)]
fn app() -> HtmlResult {
    let relays = yew::suspense::use_future(|| async move {
        nostr_minions::init_nostr_db().unwrap();
        match nostr_minions::relay_pool::UserRelay::retrieve_all_from_store().await {
            Ok(saved_relays) => {
                if saved_relays.is_empty() {
                    DEFAULT_RELAYS.to_vec()
                } else {
                    saved_relays
                }
            }
            Err(_) => {
                web_sys::console::log_1(&"Using default relays".into());
                DEFAULT_RELAYS.to_vec()
            }
        }
    })?;
    Ok(html! {
        <yew_router::BrowserRouter>
            <bunker::language::LanguageConfigsProvider>
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
            </bunker::language::LanguageConfigsProvider>
        </yew_router::BrowserRouter>
    })
}

#[function_component(Navbar)]
fn navbar() -> Html {
    let language_ctx = bunker::language::use_language_ctx();
    let navbar_button_class = classes!(
        "2xl:size-32",
        "xl:size-28",
        "lg:size-24",
        "size-20",
        "flex",
        "items-center",
        "justify-center",
        "p-4",
        "rounded-r-[2vw]",
        "text-white",
        "flex-col",
        "gap-1"
    );
    let current_route = yew_router::hooks::use_route::<bunker::MainRoute>();
    html! {
        <navbar class={classes!("min-w-fit", "h-full", "flex", "flex-col", "justify-evenly")}>
            <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::Home}>
                <div class={classes!(
                    navbar_button_class.clone(),
                    if matches!(current_route, Some(bunker::MainRoute::Home)) {
                        "bg-primary"
                    } else {
                        "bg-zinc-800"
                    }
                    )}>
                    <img src="/public/img/splashscreen.svg"
                         class={classes!("size-6", "lg:size-8", "xl:size-10", "2xl:size-12")} />
                    <span class={classes!("")}>{ language_ctx.t("navbar_home") }</span>
                </div>
            </yew_router::components::Link<bunker::MainRoute>>
            <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::NewGame}>
                <div class={classes!(
                    navbar_button_class.clone(),
                    if matches!(current_route, Some(bunker::MainRoute::NewGame)) {
                        "bg-primary"
                    } else {
                        "bg-zinc-800"
                    }
                    )}>
                    <lucide_yew::Plus class={classes!("size-6", "lg:size-8", "xl:size-10", "2xl:size-12")} />
                    <span class={classes!("")}>{ language_ctx.t("navbar_annotate") }</span>
                </div>
            </yew_router::components::Link<bunker::MainRoute>>
            <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::MyGames}>
                <div class={classes!(
                    navbar_button_class.clone(),
                    if matches!(current_route, Some(bunker::MainRoute::MyGames)) {
                        "bg-primary"
                    } else {
                        "bg-zinc-800"
                    }
                    )}>
                    <lucide_yew::BookOpen class={classes!("size-6", "lg:size-8", "xl:size-10", "2xl:size-12", )} />
                    <span class={classes!("")}>{ language_ctx.t("navbar_repertoire") }</span>
                </div>
            </yew_router::components::Link<bunker::MainRoute>>
            <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::Search}>
                <div class={classes!(
                    navbar_button_class.clone(),
                    if matches!(current_route, Some(bunker::MainRoute::Search)) {
                        "bg-primary"
                    } else {
                        "bg-zinc-800"
                    }
                    )}>
                    <lucide_yew::Search class={classes!("size-6", "lg:size-8", "xl:size-10", "2xl:size-12", )} />
                    <span class={classes!("")}>{ language_ctx.t("navbar_search") }</span>
                </div>
            </yew_router::components::Link<bunker::MainRoute>>
            <yew_router::components::Link<bunker::MainRoute> to={bunker::MainRoute::Settings}>
                <div class={classes!(
                    navbar_button_class.clone(),
                    if matches!(current_route, Some(bunker::MainRoute::Settings)) {
                        "bg-primary"
                    } else {
                        "bg-zinc-800"
                    }
                    )}>
                    <lucide_yew::Cog class={classes!("size-6", "lg:size-8", "xl:size-10", "2xl:size-12", )} />
                    <span class={classes!("")}>{ language_ctx.t("common_settings") }</span>
                </div>
            </yew_router::components::Link<bunker::MainRoute>>
        </navbar>
    }
}

#[function_component(LoginCheck)]
fn login_check(props: &yew::html::ChildrenProps) -> HtmlResult {
    let key_ctx = nostr_minions::key_manager::use_nostr_id_ctx();
    let nostr_id = yew::suspense::use_future_with(key_ctx, |_| async move {
        nostr_minions::key_manager::UserIdentity::find_identity().await
    })?;
    Ok(match *nostr_id {
        Ok(ref _id) => html! {
            {props.children.clone()}
        },
        Err(_) => {
            html! {
                <div class={"h-screen w-full flex flex-col gap-4 items-center justify-center"}>
                    <bunker::NostrLogin />
                </div>
            }
        }
    })
}

#[function_component(SplashScreen)]
pub fn splash_screen() -> Html {
    let class = classes!(
        "h-dvh",
        "w-dvw",
        "flex",
        "flex-col",
        "gap-4",
        "justify-center",
        "items-center",
        "bg-[url(/public/img/splashscreen_bg.png)]",
        "bg-cover",
        "bg-no-repeat",
        "bg-center"
    );
    html! {
        <div {class}>
            <img
                src="/public/img/splashscreen.svg"
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

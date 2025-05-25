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
        "opacity-100"
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
    html! {
        <div class={classes!("size-full", "flex", "flex-col", "gap-4", "justify-center", "items-center", "bg-[url(/public/assets/img/splashscreen_bg.png)]", "bg-cover", "bg-no-repeat", "bg-center")}>
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

#[function_component(NostrIdStatus)]
fn nostr_id_status() -> Html {
    let nostr_ctx =
        use_context::<nostr_minions::key_manager::NostrIdStore>().expect("missing nostr context");
    let nostr_id = nostr_ctx.get_pubkey();
    let has_pk = nostr_id.is_some();
    use_effect_with(nostr_ctx, move |nostr| {
        if !has_pk {
            let nostr = nostr.clone();
            yew::platform::spawn_local(async move {
                let mut new_key = "nsec1fnqevfdd7lqlp7whdensdtcjpv3hcct3cy5vepfgzc98qn83h2xqrj88ml"
                    .parse::<nostr_minions::nostro2_signer::keypair::NostrKeypair>()
                    .expect("Failed to create new key");
                new_key.set_extractable(true);
                let nostr_id = nostr_minions::key_manager::UserIdentity::from_new_keys(new_key)
                    .await
                    .expect("Failed to create new identity");
                nostr_id
                    .clone()
                    .save_to_store()
                    .await
                    .expect("Failed to save identity");
                nostr.dispatch(nostr_minions::key_manager::NostrIdAction::LoadIdentity(
                    nostr_id.get_pubkey().await.unwrap_or_default(),
                    nostr_id,
                ));
            });
        }
        || {}
    });
    html! {
        <div class={classes!("flex", "gap-2", "justify-between", "items-center")}>
            {nostr_id.map_or_else(|| html! {
                    <p>{"No Nostr ID found"}</p>
                }, |id| html! {
                    <p>{format!("Nostr ID: {}", id)}</p>
                })}
        </div>
    }
}

#[function_component(RelayPoolStatus)]
fn relay_pool_status() -> Html {
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("missing relay context");
    let relay_status_state = use_state(Vec::new);
    let relay_set = relay_status_state.setter();
    use_effect_with(relay_ctx, move |relay| {
        let relay = relay.clone();
        yew::platform::spawn_local(async move {
            loop {
                gloo::timers::future::sleep(std::time::Duration::from_secs(1)).await;
                let status = relay.relay_health();
                relay_set.set(status.values().copied().collect());
            }
        });
        || {}
    });
    let open_relays = relay_status_state
        .iter()
        .filter(|r| r == &&nostr_minions::relay_pool::ReadyState::OPEN)
        .count();
    html! {
        <div class={classes!("flex", "gap-2", "justify-between", "items-center")}>
            <p>{format!("Connected to {}/{} relays", open_relays,relay_status_state.len())}</p>
        </div>
    }
}

#[function_component(SettingsDrawer)]
fn settings_drawer() -> Html {
    let is_open = use_state(|| false);
    let open_onclick = { Callback::from(move |_| is_open.set(true)) };

    html! {
        <>
            <button onclick={open_onclick} class={classes!("fixed", "top-6", "right-6", "z-10")}>
                <lucide_yew::Settings class={classes!("size-8", "text-gray-400")} />
            </button>
            //<LeftDrawer {is_open}>
            //    <div class={classes!("")}>
            //        <h1>{"Settings"}</h1>
            //        <RelayPoolStatus />
            //        <NostrIdStatus />
            //    </div>
            //</LeftDrawer>
        </>
    }
}

//#[function_component(BoardPositionPreview)]
//fn board_position_preview() -> Html {
//    let move_san = "e2-e4";
//    let id = "board-preview";
//    html! {
//        <div class={classes!("size-10")}>
//        </div>
//    }
//}

#[derive(Clone, Debug, Properties, PartialEq)]
struct MoveSelectorProps {
    pub selected_move: UseStateHandle<Option<shakmaty::Move>>,
    pub selected_piece: UseStateHandle<Option<shakmaty::Role>>,
}

#[function_component(AvailableMovesSelector)]
fn available_moves_selector(props: &MoveSelectorProps) -> Html {
    let MoveSelectorProps {
        selected_move,
        selected_piece,
    } = props.clone();
    let game_ctx =
        use_context::<annotator::live_game::AnnotatedGameStore>().expect("missing game context");
    let label_classes = classes!(
        "absolute",
        "inset-0",
        "cursor-pointer",
        "px-2",
        "py-4",
        "border-2",
        "border-gray-300",
        "rounded-md",
        "transition-colors",
        "bg-gray-200",
        "peer-checked:bg-blue-500",
        "peer-checked:text-white"
    );

    html! {
        <>
        <div class={classes!("flex", "gap-1", "justify-between", "mb-4")}>
            {for shakmaty::Role::ALL.iter().map(|piece| {
                let onclick = {
                    let handler = selected_piece.setter();
                    let new_piece = *piece;
                    Callback::from(move |_: MouseEvent| {
                        handler.set(Some(new_piece));
                    })
                };
                let piece = piece.of(game_ctx.color_turn());
                let url = format!(
                    "/public/assets/img/{}{}.svg",
                    piece.color.char(),
                    piece.role.upper_char()
                );
                let name = format!("{}{}", piece.color.char(), piece.role.upper_char());
                html! {
                    <div {onclick} key={name.clone()} class="relative h-12 w-12">
                      <input
                        type="radio"
                        checked={*selected_piece == Some(piece.role)}
                        name={name.clone()}
                        class="peer absolute h-0 w-0 opacity-0" // Hidden but accessible
                      />
                      <label
                        for={name.clone()}
                        class={classes!(label_classes.clone(), "flex", "items-center", "justify-center", "h-full", "w-full")} >
                            <img src={url} alt={name} class="size-10 object-contain" />
                      </label>
                    </div>
                }})
            }
        </div>
        <div class={classes!("grid", "grid-cols-3", "gap-4", "mt-2", "flex-1")}>
            {for game_ctx.legal_moves().iter().filter_map(|mv| {
                let new_selected_piece = *selected_piece;
                (Some(mv.role()) == new_selected_piece).then_some({})?;
                let onclick = {
                    let selected_move = selected_move.clone();
                    let m = mv.clone();
                    Callback::from(move |_| {
                        selected_move.set(Some(m.clone()));
                    })
                };
                let move_input = format!("{mv}");
                Some(html! {
                    <div {onclick} class="relative h-12">
                      <input
                        type="radio"
                        id={move_input.clone()}
                        name="move"
                        value={move_input.clone()}
                        class="peer absolute h-0 w-0 opacity-0" // Hidden but accessible
                      />
                      <label
                        for={move_input.clone()}
                        class="block w-full h-full cursor-pointer p-2 border-2 border-gray-300 text-xs flex items-center justify-center
                            rounded-md transition-colors bg-gray-200 peer-checked:bg-blue-500 peer-checked:text-white" >
                        {move_input}
                      </label>
                    </div>
                })})
            }
        </div>
        </>
    }
}

#[function_component(NextMoveForm)]
fn next_move_form() -> Html {
    let game_ctx =
        use_context::<annotator::live_game::AnnotatedGameStore>().expect("missing game context");
    let selected_move = use_state(|| None);
    let selected_piece = use_state(|| None);
    let onsubmit = {
        let selected_move = selected_move.clone();
        let selected_piece = selected_piece.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(mv) = (*selected_move).clone() {
                game_ctx.dispatch(annotator::live_game::AnnotatedGameAction::PlayMove(mv));
                selected_move.set(None);
                selected_piece.set(None);
            }
        })
    };
    let button_classes = if (selected_move.is_some(), selected_piece.is_some()) == (true, true) {
        classes!("bg-blue-500", "text-white", "p-2", "rounded-md")
    } else {
        classes!("bg-gray-300", "text-gray-500", "p-2", "rounded-md")
    };
    html! {
        <form {onsubmit} class={classes!("flex", "flex-col", "gap-2","h-full")}>
            <AvailableMovesSelector
                selected_move={selected_move.clone()}
                selected_piece={selected_piece.clone()}
            />
            <button class={button_classes}>
                {"Play Move"}
            </button>
        </form>
    }
}

// #[function_component(EventDetailsSection)]
// fn event_details_section() -> Html {
//     let game_ctx =
//         use_context::<annotator::live_game::AnnotatedGameStore>().expect("missing game context");
//     let pgn_game = game_ctx.pgn_game();
//     let open_modal = use_state(|| false);
//     let display_el = match &pgn_game.event {
//         rooky_core::pgn_standards::PgnEvent::Casual => {
//             html! {<p>{"Casual"}</p>}
//         }
//         rooky_core::pgn_standards::PgnEvent::Named(_name) => {
//             html! {
//                 <>
//                 <p>{format!("Event: {}", pgn_game.event)}</p>
//                 <p>{format!("Site: {}", pgn_game.site)}</p>
//                 <p>{format!("Round: {}", pgn_game.round)}</p>
//                 </>
//             }
//         }
//         rooky_core::pgn_standards::PgnEvent::Unknown => {
//             html! {<p>{"Casual"}</p>}
//         }
//     };
//     let tournament_form = {
//         let onsubmit = {
//             let game_ctx = game_ctx.dispatcher();
//             Callback::from(move |e: SubmitEvent| {
//                 e.prevent_default();
//                 let form = e.target_unchecked_into::<web_sys::HtmlFormElement>();
//                 let event = form
//                     .get_with_name("event")
//                     .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
//                     .map(|input| input.value())
//                     .unwrap_or_default();
//                 let site = form
//                     .get_with_name("site")
//                     .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
//                     .map(|input| input.value())
//                     .unwrap_or_default();
//                 let round = form
//                     .get_with_name("round")
//                     .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
//                     .map(|input| input.value())
//                     .unwrap_or_default();
//                 game_ctx.dispatch(
//                     annotator::live_game::AnnotatedGameAction::UpdateEventDetails {
//                         event,
//                         site,
//                         round,
//                     },
//                 );
//             })
//         };
//         html! {
//             <form {onsubmit} class={classes!("flex", "flex-col", "gap-2", "bg-white", "p-4")}>
//                 <input
//                     type="text"
//                     name="event"
//                     placeholder="Event"
//                     class={classes!("w-24")}
//                 />
//                 <input
//                     type="text"
//                     name="site"
//                     placeholder="Site"
//                     class={classes!("w-24")}
//                 />
//                 <input
//                     type="text"
//                     name="round"
//                     placeholder="Round"
//                     class={classes!("w-24")}
//                 />
//                 <input
//                     type="submit"
//                     value="Save"
//                     class={classes!("bg-blue-500", "text-white", "p-2", "rounded-md")}
//                 />
//             </form>
//         }
//     };
//     html! {
//         <div class={classes!("flex", "flex-row", "gap-2", "items-center")}>
//             {display_el}
//             <button
//                 onclick={
//                     {
//                     let open_modal = open_modal.clone();
//                     Callback::from(move |_| open_modal.set(true))}}
//                 class={classes!("bg-blue-500", "text-white", "p-2", "rounded-md")}>
//                     <lucide_yew::Plus class={classes!("size-4", "text-white")} />
//                 </button>
//             <Modal
//                 is_open={open_modal}
//             >
//             {tournament_form}
//             </Modal>
//         </div>
//     }
// }

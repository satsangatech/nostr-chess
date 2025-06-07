mod notifications;
mod profile;
pub use notifications::*;
pub use profile::*;

use nostr_minions::browser_api::IdbStoreManager;
use shady_minions::ui::{
    Button, Card, CardContent, CardDescription, CardHeader, CardTitle, Form, Input, Popover,
    PopoverContent, PopoverTrigger,
};
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct JsChessGameProps {
    #[prop_or_default]
    pub game: rooky_core::RookyGame,
}

#[function_component(JsChessGame)]
pub fn js_chess_game(props: &JsChessGameProps) -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    let game_context = crate::live_game::use_game_history();
    let board_ref = use_node_ref();

    let route = yew_router::hooks::use_route::<crate::MainRoute>();
    let game_id = route
        .clone()
        .and_then(|route| match route {
            crate::MainRoute::GameDetail { id } => Some(id),
            _ => None,
        })
        .unwrap_or_default();
    let positions = props.game.game_positions();
    let game_board = use_mut_ref(|| None);
    let current_position = use_mut_ref(|| 0);
    let current_index = use_state(|| 0usize);

    {
        let board_setting = game_board.clone();
        let game_id = game_id.clone();
        use_effect_with(game_context.synced, move |synced| {
            if *synced {
                let board_options = chessboard_js::ChessboardConfig {
                    draggable: false,
                    piece_theme: "/public/img/pieces/{piece}.svg",
                    ..Default::default()
                };
                let board = chessboard_js::ChessBoardJs::new(&game_id, Some(board_options));
                *board_setting.borrow_mut() = Some(board);
            }
            || {}
        });
    }

    let next_move_onclick = {
        let positions = positions.clone();
        let current_position = current_position.clone();
        let game_board = game_board.clone();
        let current_index = current_index.clone();
        Callback::from(move |_| {
            let board_opt = game_board.borrow();
            if let Some(board) = board_opt.as_ref() {
                let mut pos = *current_position.borrow();
                pos += 1;
                let len = positions.len();
                if pos >= len {
                    return;
                }
                *current_position.borrow_mut() = pos;

                if let Some(position) = positions.get(pos).cloned() {
                    let fen =
                        shakmaty::fen::Fen::from_position(position, shakmaty::EnPassantMode::Legal)
                            .to_string();
                    board.set_position(&fen);
                    current_index.set(pos);
                }
            }
        })
    };

    let prev_move_onclick = {
        let positions = positions.clone();
        let current_position = current_position.clone();
        let game_board = game_board.clone();
        let current_index = current_index.clone();
        Callback::from(move |_| {
            let board_opt = game_board.borrow();
            if let Some(board) = board_opt.as_ref() {
                let mut pos = *current_position.borrow();
                if pos == 0 {
                    return;
                }
                pos -= 1;
                *current_position.borrow_mut() = pos;
                if let Some(position) = positions.get(pos).cloned() {
                    let fen =
                        shakmaty::fen::Fen::from_position(position, shakmaty::EnPassantMode::Legal)
                            .to_string();
                    board.set_position(&fen);
                    current_index.set(pos);
                }
            }
        })
    };
    {
        let next_cb = next_move_onclick.clone();
        let prev_cb = prev_move_onclick.clone();

        use_effect_with((), move |_| {
            let keydown_listener = gloo::events::EventListener::new(
                &web_sys::window().unwrap(),
                "keydown",
                move |event| {
                    let event = event.dyn_ref::<KeyboardEvent>().unwrap();
                    match event.key().as_str() {
                        "ArrowRight" => {
                            next_cb.emit(());
                        }
                        "ArrowLeft" => {
                            prev_cb.emit(());
                        }
                        _ => {}
                    }
                },
            );

            // Keep the listener alive
            move || drop(keydown_listener)
        });
    }

    html! {
        <>
        <Card class="h-fit w-fit">
            <CardHeader>
                <CardTitle >
                    <div ref={board_ref} id={game_id} class="h-[66vh] aspect-square" />
                </CardTitle>
            </CardHeader>
            <CardContent class="flex gap-4">
                <Button
                    class="flex-1"
                    onclick={Callback::from(move |_| {
                        prev_move_onclick.emit(());
                    })}>
                    {language_ctx.t("game_prev_move")}
                </Button>
                <Button
                    class="flex-1"
                    onclick={Callback::from(move |_| {
                        next_move_onclick.emit(());
                    })}>
                    {language_ctx.t("game_next_move")}
                </Button>
            </CardContent>
        </Card>
        <Card class="h-fit min-w-sm  max-h-[86vh] overflow-hidden">
            <CardHeader>
                <CardTitle class="mb-8">
                    <div class="flex justify-between items-top">
                        <h3 class="text-2xl font-bold text-white">
                            {language_ctx.t("common_game_details")}
                        </h3>
                    </div>
                </CardTitle>
            </CardHeader>
            <CardContent>
            <GameCard pgn_game={props.game.clone()}  />
            <div class="flex flex-col gap-2 p-6">
                <ShareRookyGame ..props.clone() />
                <DirectMessageRookyGame ..props.clone() />
                <SaveTxtRookyGame ..props.clone() />
            </div>
            </CardContent>
        </Card>
        </>
    }
}

#[function_component(ShareRookyGameCard)]
pub fn share_rooky_game_card(props: &JsChessGameProps) -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    html! {
        <Card class="size-fit">
            <CardHeader>
                <CardTitle>{language_ctx.t("share_rooky_game_title")}</CardTitle>
                <CardDescription class="max-w-64 text-wrap">
                    {language_ctx.t("share_rooky_game_desc")}
                </CardDescription>
            </CardHeader>
            <CardContent class="flex flex-col gap-2">
                <ShareRookyGame ..props.clone() />
                <DirectMessageRookyGame ..props.clone() />
                <SaveTxtRookyGame ..props.clone() />
            </CardContent>
        </Card>

    }
}

#[function_component(ShareRookyGame)]
pub fn share_rooky_game(props: &JsChessGameProps) -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("Relay context not found");
    let game_ctx = crate::live_game::use_game_history();
    let Some(keypair) = nostr_minions::key_manager::use_nostr_key() else {
        return html! {
            <lucide_yew::Share2 class={classes!("size-5", "bg-muted", "text-muted-foreground")} />
        };
    };
    let onclick = {
        let keypair = keypair.clone();
        let game = props.game.clone();
        let relay_ctx = relay_ctx.clone();
        let game_ctx = game_ctx.dispatcher();
        Callback::from(move |_| {
            let mut game_note: nostr_minions::nostro2::NostrNote = game.clone().into();
            if keypair.sign_note(&mut game_note).is_err() {
                web_sys::console::error_1(&"Failed to sign note".into());
                nostr_minions::widgets::toastify::ToastifyOptions::new_failure(
                    "Failed to sign note",
                )
                .show();
                return;
            }
            let game_entry = rooky_core::idb::RookyGameEntry {
                id: game_note.id.clone().unwrap_or_default(),
                note: game_note.clone(),
                origin: rooky_core::idb::GameOrigin::Annotated,
            };
            game_ctx.dispatch(crate::live_game::AnnotatedGameHistoryAction::AddGame(
                game_entry.clone(),
            ));
            yew::platform::spawn_local(async move {
                if game_entry.save_to_store().await.is_err() {
                    web_sys::console::error_1(&"Failed to save game".into());
                    nostr_minions::widgets::toastify::ToastifyOptions::new_failure(
                        "Failed to save game",
                    )
                    .show();
                }
            });
            relay_ctx.send(game_note);
            nostr_minions::widgets::toastify::ToastifyOptions::new_success(
                "Game shared successfully",
            )
            .show();
        })
    };

    html! {
        <Button {onclick}>
            <lucide_yew::Share2
                class={classes!("size-5")} />
            <span class="ml-2">{language_ctx.t("share_to_nostr")}</span>
        </Button>
    }
}
use nostr_minions::nostro2_signer::nostro2_nips::Nip17;
#[function_component(DirectMessageRookyGame)]
pub fn dm_rooky_game(props: &JsChessGameProps) -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("Relay context not found");
    let game_ctx = crate::live_game::use_game_history();
    let Some(keypair) = nostr_minions::key_manager::use_nostr_key() else {
        return html! {
            <lucide_yew::Share2 class={classes!("size-5", "bg-muted", "text-muted-foreground")} />
        };
    };
    let onsubmit = {
        let keypair = keypair.clone();
        let game = props.game.clone();
        let relay_ctx = relay_ctx.clone();
        let game_ctx = game_ctx.dispatcher();
        Callback::from(move |form: web_sys::HtmlFormElement| {
            let Some(recipient) = form
                .get_with_name("recipient")
                .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
                .map(|input| input.value())
            else {
                web_sys::console::log_1(&"Recipient not found".into());
                nostr_minions::widgets::toastify::ToastifyOptions::new_failure(
                    "Recipient not found",
                )
                .show();
                return;
            };
            let mut note = game.clone().into();
            if keypair.sign_note(&mut note).is_err() {
                web_sys::console::error_1(&"Failed to sign note".into());
                nostr_minions::widgets::toastify::ToastifyOptions::new_failure(
                    "Failed to sign note",
                )
                .show();
                return;
            }
            let note_entry = rooky_core::idb::RookyGameEntry {
                id: note.id.clone().unwrap_or_default(),
                note: note.clone(),
                origin: rooky_core::idb::GameOrigin::Annotated,
            };
            if let Ok(dm_game) = keypair.private_dm(&game.to_pgn(), &recipient) {
                relay_ctx.send(dm_game);
                game_ctx.dispatch(crate::live_game::AnnotatedGameHistoryAction::AddGame(
                    note_entry.clone(),
                ));
            } else {
                web_sys::console::error_1(&"Failed to send DM".into());
                nostr_minions::widgets::toastify::ToastifyOptions::new_failure("Failed to send DM")
                    .show();
                return;
            }
            nostr_minions::widgets::toastify::ToastifyOptions::new_success("DM sent successfully")
                .show();
        })
    };

    html! {
        <Button>
        <Popover>
            <PopoverTrigger>
                <div class="flex items-center gap-2">
                <lucide_yew::MessageSquareLock class={classes!("size-5")} />
                <span class="ml-2">{language_ctx.t("send_nostr_dm")}</span>
                </div>
            </PopoverTrigger>
            <PopoverContent>
                <Form {onsubmit} class="flex gap-2">
                    <Input
                        name="recipient"
                        r#type={shady_minions::ui::InputType::Text}
                        placeholder={language_ctx.t("enter_recipient_nostr_id")}
                        class={classes!("w-full", "mb-2", "min-w-32")} />
                    <Button r#type={shady_minions::ui::ButtonType::Submit}>
                        <lucide_yew::MessageSquareLock class={classes!("size-5")} />
                    </Button>
                </Form>
                </PopoverContent>
        </Popover>
        </Button>
    }
}

#[derive(Properties, PartialEq)]
pub struct GameCardProps {
    pub pgn_game: rooky_core::RookyGame,
}

#[function_component(GameCard)]
pub fn game_card(props: &GameCardProps) -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    let rooky_core::RookyGame {
        event,
        outcome,
        moves,
        white,
        black,
        date,
        site,
        round,
    } = &props.pgn_game;

    // Check if it's a casual game
    let is_casual_game = event == &rooky_core::pgn_standards::PgnEvent::Casual;
    let white_name = if white.is_empty() {
        language_ctx.t("game_details_white")
    } else {
        white.clone()
    };
    let black_name = if black.is_empty() {
        language_ctx.t("game_details_black")
    } else {
        black.clone()
    };

    html! {
            <>
                <div class="w-full space-y-2">
                    <h3 class="text-lg font-bold text-white">
                        { format!("{white_name} vs {black_name}") }
                    </h3>
                    <div class="flex justify-between text-white">
                        <span class="text-sm font-bold">{language_ctx.t("game_details_date")}</span>
                        <span class="text-sm">{ date.format("%Y-%m-%d").to_string() }</span>
                    </div>
                    <div class="flex justify-between text-white">
                        <span class="text-sm font-bold">{language_ctx.t("game_details_result")}</span>
                        <span class="text-sm">{ outcome.to_string() }</span>
                    </div>
                    <div class="flex justify-between text-white">
                        <span class="text-sm font-bold">{language_ctx.t("game_details_event")}</span>
                        <span class="text-sm">{ event.to_string() }</span>
                    </div>
                    {if !is_casual_game {
                        html! {
                            <>
                            <div class="flex justify-between text-white">
                                <span class="text-sm font-bold">{language_ctx.t("game_details_site")}</span>
                                <span class="text-sm">{ site.to_string() }</span>
                            </div>
                            <div class="flex justify-between text-white">
                                <span class="text-sm font-bold">{language_ctx.t("game_details_round")}</span>
                                <span class="text-sm">{ round.to_string() }</span>
                            </div>
                            </>
                        }
                    } else { html! {}}}
                </div>
                <div id="separator" class="h-[0.5px] bg-secondary my-4" />
                <div class="text-sm text-white flex flex-wrap gap-2 max-h-18 overflow-y-auto">
                    {
                        moves.iter().enumerate().map(|(i, move_text)| {
                            let turn = i / 2;
                            let is_white = i % 2 == 0;
                            let turn_number = turn + 1;

                            html! {
                                <span class={classes!("inline-flex", "items-center", "whitespace-nowrap")}>
                                    {
                                        if is_white {
                                            html! {
                                                <span class="mr-2 text-secondary text-xs">
                                                    { format!("{}.", turn_number) }
                                                </span>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    <span>{ move_text.to_string() }</span>
                                </span>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div id="separator" class="h-[0.5px] bg-secondary my-4" />
            </>
    }
}

#[function_component(SaveTxtRookyGame)]
pub fn save_txt_rooky_game(props: &JsChessGameProps) -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    use nostr_minions::browser_api::IdbStoreManager;
    let game = props.game.clone();
    let game_context = crate::live_game::use_game_history();
    let keypair = nostr_minions::key_manager::use_nostr_key();
    let onclick = {
        let game = game.clone();
        let keypair = keypair.clone();
        Callback::from(move |_| {
            let mut note = game.clone().into();
            if let Some(keypair) = keypair.as_ref() {
                if keypair.sign_note(&mut note).is_err() {
                    nostr_minions::widgets::toastify::ToastifyOptions::new_failure(
                        "Failed to sign note",
                    )
                    .show();
                    return;
                }
            } else {
                nostr_minions::widgets::toastify::ToastifyOptions::new_failure(
                    "No Nostr keypair found",
                )
                .show();
                return;
            }
            let id = note.id.clone().unwrap_or_default();
            let note_entry = rooky_core::idb::RookyGameEntry {
                id: id.clone(),
                note,
                origin: rooky_core::idb::GameOrigin::Annotated,
            };
            let note_entry_clone = note_entry.clone();
            yew::platform::spawn_local(async move {
                note_entry_clone
                    .save_to_store()
                    .await
                    .unwrap_or_else(|err| {
                        web_sys::console::error_1(&format!("Failed to save game: {err:#?}").into());

                        nostr_minions::widgets::toastify::ToastifyOptions::new_failure(
                            "Failed to save game",
                        )
                        .show();
                    });
            });
            game_context.dispatch(crate::live_game::AnnotatedGameHistoryAction::AddGame(
                note_entry,
            ));
            let blob_parts = web_sys::js_sys::Array::new();
            blob_parts.push(&web_sys::wasm_bindgen::JsValue::from_str(&game.to_pgn()));
            let blob = web_sys::Blob::new_with_str_sequence(&blob_parts).unwrap();

            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            let a = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("a")
                .unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", &format!("game-{id}.pgn"))
                .unwrap();
            a.dispatch_event(&web_sys::MouseEvent::new("click").unwrap())
                .unwrap();
            web_sys::Url::revoke_object_url(&url).unwrap();
            nostr_minions::widgets::toastify::ToastifyOptions::new_success(
                "Game saved successfully",
            )
            .show();
        })
    };

    html! {
        <Button {onclick}>
            <lucide_yew::Download class={classes!("size-5")} />
            <span class="ml-2">{language_ctx.t("share_save_pgn")}</span>
        </Button>
    }
}

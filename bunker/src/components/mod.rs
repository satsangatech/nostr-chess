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
    let board_ref = use_node_ref();
    web_sys::console::log_1(&"JsChessGame".into());

    let pgn_moves = props.game.moves.clone();
    let route = yew_router::hooks::use_route::<crate::MainRoute>();
    let game_id = route
        .clone()
        .and_then(|route| match route {
            crate::MainRoute::GameDetail { id } => Some(id),
            _ => None,
        })
        .unwrap_or_default();
    let date = props.game.date;
    let white_name = props.game.white.clone();
    let black_name = props.game.black.clone();
    let positions = props.game.game_positions();
    let game_board = use_mut_ref(|| None);
    let current_position = use_mut_ref(|| 0);
    let current_index = use_state(|| 0usize);

    {
        let board_setting = game_board.clone();
        let game_id = game_id.clone();
        use_effect_with((), move |()| {
            let board_options = chessboard_js::ChessboardConfig {
                draggable: false,
                ..Default::default()
            };
            let board = chessboard_js::ChessBoardJs::new(&game_id, Some(board_options));
            *board_setting.borrow_mut() = Some(board);
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
        <div class="flex gap-4 size-full">
            <div class="flex flex-col gap-2 size-fit">
                <div ref={board_ref} id={game_id.clone()} class="min-w-md flex-1" />
                <div class="flex">
                    <Button
                        class="flex-1"
                        onclick={Callback::from(move |_| {
                            prev_move_onclick.emit(());
                        })}>
                        {"Previous Move"}
                    </Button>
                    <Button
                        class="flex-1"
                        onclick={Callback::from(move |_| {
                            next_move_onclick.emit(());
                        })}>
                        {"Next Move"}
                    </Button>
                </div>
            </div>
            <div class="grid grid-cols-1 gap-4">
                <Card class="size-fit">
                <CardHeader>
                    <CardTitle>{ format!("{} vs {}", white_name, black_name) }</CardTitle>
                    <CardDescription class="max-w-xs truncate">
                        <br />
                        { format!("Date: {}", date.format("%Y-%m-%d")) }
                        <br />
                        { format!("Game ID: {}", game_id) }
                        <br />
                        { format!("Event: {}", props.game.event) }
                        <br />
                        { format!("Result: {}", props.game.outcome) }
                        <br />
                    </CardDescription>
                </CardHeader>
                <CardContent>
                <div class="text-sm text-gray-500 flex flex-wrap gap-1 max-w-sm">
                    {pgn_moves.iter().enumerate().map(|(i, move_text)| {
                        let turn = i / 2;
                        let is_white = i % 2 == 0;
                        let turn_number = turn + 1;
                        let is_current = *current_index > 0 && i == *current_index - 1;

                        html! {
                            <span class={classes!("inline-flex", "items-center", "whitespace-nowrap", "mr-1", if is_current { "font-bold" } else { "" })}>
                                {
                                    if is_white {
                                        html! { <span class="mr-0.5">{ format!("{turn_number}.", ) }</span> }
                                    } else {
                                        html! {}
                                    }
                                }
                                <span>{ move_text.to_string() }</span>
                            </span>
                        }
                    }).collect::<Html>()}
                </div>
                </CardContent>
                </Card>
                <ShareRookyGameCard ..props.clone() />
            </div>
        </div>
    }
}

#[function_component(ShareRookyGameCard)]
pub fn share_rooky_game_card(props: &JsChessGameProps) -> Html {
    html! {
        <Card class="size-fit">
            <CardHeader>
                <CardTitle>{ "Share Rooky Game" }</CardTitle>
                <CardDescription class="max-w-64 text-wrap">
                    { "Share this game with others. Any option you choose will also save the game locally on the app." }
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

use nostr_minions::browser_api::IdbStoreManager;
#[function_component(ShareRookyGame)]
pub fn share_rooky_game(props: &JsChessGameProps) -> Html {
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("Relay context not found");
    let Some(keypair) = nostr_minions::key_manager::use_nostr_key() else {
        return html! {
            <lucide_yew::Share2 class={classes!("size-5", "bg-muted", "text-muted-foreground")} />
        };
    };
    let onclick = {
        let keypair = keypair.clone();
        let game = props.game.clone();
        let relay_ctx = relay_ctx.clone();
        Callback::from(move |_| {
            let mut game_note: nostr_minions::nostro2::NostrNote = game.clone().into();
            keypair
                .sign_note(&mut game_note)
                .expect("Failed to sign note");
            let game_entry = rooky_core::idb::RookyGameEntry {
                note: game_note.clone(),
                origin: rooky_core::idb::GameOrigin::Annotated,
            };
            yew::platform::spawn_local(async move {
                game_entry
                    .save_to_store()
                    .await
                    .expect("Failed to save game");
            });
            relay_ctx.send(game_note);
        })
    };

    html! {
        <Button {onclick}>
            <lucide_yew::Share2
                class={classes!("size-5")} />
            <span class="ml-2">{"Share to Nostr Socials"}</span>
        </Button>
    }
}
use nostr_minions::nostro2_signer::nostro2_nips::Nip17;
#[function_component(DirectMessageRookyGame)]
pub fn dm_rooky_game(props: &JsChessGameProps) -> Html {
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("Relay context not found");
    let Some(keypair) = nostr_minions::key_manager::use_nostr_key() else {
        return html! {
            <lucide_yew::Share2 class={classes!("size-5", "bg-muted", "text-muted-foreground")} />
        };
    };
    let onsubmit = {
        let keypair = keypair.clone();
        let game = props.game.clone();
        let relay_ctx = relay_ctx.clone();
        Callback::from(move |form: web_sys::HtmlFormElement| {
            let Some(recipient) = form
                .get_with_name("recipient")
                .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
                .map(|input| input.value())
            else {
                web_sys::console::log_1(&"Recipient not found".into());
                return;
            };
            let mut note = game.clone().into();
            keypair.sign_note(&mut note).expect("Failed to sign note");
            let note_entry = rooky_core::idb::RookyGameEntry {
                note: note.clone(),
                origin: rooky_core::idb::GameOrigin::Annotated,
            };
            yew::platform::spawn_local(async move {
                note_entry
                    .save_to_store()
                    .await
                    .expect("Failed to save game");
            });
            let dm_game = keypair
                .private_dm(&game.to_pgn(), &recipient)
                .expect("Failed to sign note");
            relay_ctx.send(dm_game);
        })
    };

    html! {
        <Popover>
            <PopoverTrigger>
                <Button>
                    <lucide_yew::MessageSquareLock class={classes!("size-5")} />
                    <span class="ml-2">{"Send Nostr DM"}</span>
                </Button>
            </PopoverTrigger>
            <PopoverContent>
                <Form {onsubmit} class="flex gap-2">
                    <Input
                        name="recipient"
                        r#type={shady_minions::ui::InputType::Text}
                        placeholder="Enter recipient Nostr ID"
                        class={classes!("w-full", "mb-2", "min-w-32")} />
                    <Button r#type={shady_minions::ui::ButtonType::Submit}>
                        <lucide_yew::MessageSquareLock class={classes!("size-5")} />
                    </Button>
                </Form>
            </PopoverContent>
        </Popover>
    }
}

#[function_component(SaveTxtRookyGame)]
pub fn save_txt_rooky_game(props: &JsChessGameProps) -> Html {
    let game = props.game.clone();
    let mut note: nostr_minions::nostro2::NostrNote = game.clone().into();
    note.serialize_id().expect("Failed to serialize ID");
    let onclick = {
        let game = game.clone();
        let id = note.id.take().unwrap();
        Callback::from(move |_| {
            let note = game.clone().into();
            let note_entry = rooky_core::idb::RookyGameEntry {
                note,
                origin: rooky_core::idb::GameOrigin::Annotated,
            };
            yew::platform::spawn_local(async move {
                note_entry
                    .save_to_store()
                    .await
                    .expect("Failed to save game");
            });
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
        })
    };

    html! {
        <Button {onclick}>
            <lucide_yew::Download class={classes!("size-5")} />
            <span class="ml-2">{"Save as PGN file"}</span>
        </Button>
    }
}

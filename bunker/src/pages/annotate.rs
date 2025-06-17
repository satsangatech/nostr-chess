use std::str::FromStr;

use shady_minions::ui::{
    Button, Card, CardContent, CardHeader, CardTitle, Form, Input, Modal, Popover, PopoverContent,
    PopoverTrigger,
};
use shakmaty::Position;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::ShareRookyGameCard;

#[function_component(NewJsChessGame)]
pub fn js_chess_game() -> Html {
    let board_ref = use_node_ref();
    let game_ctx = use_context::<crate::contexts::live_game::AnnotatedGameHistoryStore>()
        .expect("ChessboardContext not found");

    let game_board = use_mut_ref(|| None::<chessboard_js::ChessBoardJs>);
    let game_position = use_mut_ref(shakmaty::Chess::new);
    let pgn_game = use_mut_ref(rooky_core::RookyGame::default);

    let force_update = use_state(|| 0);
    let force_update_cb = { Callback::from(move |()| force_update.set(*force_update + 1)) };

    let position = game_position.clone();
    let on_snap_start = Box::new(
        move |_source: web_sys::wasm_bindgen::JsValue,
              piece: web_sys::wasm_bindgen::JsValue,
              _position: web_sys::wasm_bindgen::JsValue,
              _color: web_sys::wasm_bindgen::JsValue| {
            if position.borrow().is_game_over() {
                return web_sys::wasm_bindgen::JsValue::from_bool(false);
            }

            let piece_str = piece.as_string().unwrap_or_default();
            let turn = position.borrow().turn();
            if (turn == shakmaty::Color::White && piece_str.starts_with('b'))
                || (turn == shakmaty::Color::Black && piece_str.starts_with('w'))
            {
                return web_sys::wasm_bindgen::JsValue::from_bool(false);
            }
            web_sys::wasm_bindgen::JsValue::from_bool(true)
        },
    )
        as Box<
            dyn Fn(
                web_sys::wasm_bindgen::JsValue,
                web_sys::wasm_bindgen::JsValue,
                web_sys::wasm_bindgen::JsValue,
                web_sys::wasm_bindgen::JsValue,
            ) -> web_sys::wasm_bindgen::JsValue,
        >;

    let position = game_position.clone();
    let update_ui = force_update_cb.clone();
    let pgn_game_clone = pgn_game.clone();
    let on_drop_cb = Box::new(
        move |source: web_sys::wasm_bindgen::JsValue, target: web_sys::wasm_bindgen::JsValue| {
            let Some(source) = source
                .as_string()
                .and_then(|s| shakmaty::Square::from_str(&s).ok())
            else {
                return web_sys::wasm_bindgen::JsValue::from_str("snapback");
            };
            let Some(target) = target
                .as_string()
                .and_then(|s| shakmaty::Square::from_str(&s).ok())
            else {
                return web_sys::wasm_bindgen::JsValue::from_str("snapback");
            };

            let Ok(shak_move) = shakmaty::uci::UciMove::Normal {
                from: source,
                to: target,
                promotion: None,
            }
            .to_move(&*position.borrow()) else {
                return web_sys::wasm_bindgen::JsValue::from_str("snapback");
            };
            let san = shakmaty::san::SanPlus::from_move(position.borrow().clone(), &shak_move);
            position.borrow_mut().play_unchecked(&shak_move);
            let new_pgn = pgn_game_clone.borrow().clone().new_move(san);
            pgn_game_clone.replace(new_pgn);
            update_ui.emit(());
            web_sys::wasm_bindgen::JsValue::undefined()
        },
    )
        as Box<
            dyn Fn(
                web_sys::wasm_bindgen::JsValue,
                web_sys::wasm_bindgen::JsValue,
            ) -> web_sys::wasm_bindgen::JsValue,
        >;

    let game_board_clone = game_board.clone();
    let pgn_game_clone = pgn_game.clone();
    let update_ui = force_update_cb.clone();
    let on_snap_end = Box::new(move || {
        if let Some(board) = game_board_clone.borrow().as_ref() {
            board.set_position(
                shakmaty::fen::Fen::from_position(
                    game_position.borrow().clone(),
                    shakmaty::EnPassantMode::Legal,
                )
                .to_string()
                .as_str(),
            );
            if game_position.borrow().is_game_over() {
                if let Some(outcome) = game_position.borrow().outcome() {
                    pgn_game_clone.borrow_mut().outcome = outcome;
                    update_ui.emit(());
                }
            }
        }
    }) as Box<dyn Fn()>;

    {
        let board_setting = game_board;
        use_effect_with(game_ctx.synced, move |synced| {
            if *synced {
                let board_options = chessboard_js::ChessboardConfig {
                    draggable: true,
                    piece_theme: "/public/img/pieces/{piece}.svg",
                    drop_off_board: chessboard_js::DropOffBoard::Snapback,
                    on_drop: Some(
                        web_sys::wasm_bindgen::closure::Closure::wrap(on_drop_cb)
                            .into_js_value()
                            .unchecked_into(),
                    ),
                    on_drag_start: Some(
                        web_sys::wasm_bindgen::closure::Closure::wrap(on_snap_start)
                            .into_js_value()
                            .unchecked_into(),
                    ),
                    on_snap_end: Some(
                        web_sys::wasm_bindgen::closure::Closure::wrap(on_snap_end)
                            .into_js_value()
                            .unchecked_into(),
                    ),
                    ..Default::default()
                };
                let board = chessboard_js::ChessBoardJs::new("game", Some(board_options));
                *board_setting.borrow_mut() = Some(board);
            }
            || {}
        });
    }

    let language_ctx = crate::contexts::language::use_language_ctx();
    html! {
        <div class="pl-12 h-full flex flex-col justify-evenly">
            <h2 class="text-4xl text-white font-black">{ language_ctx.t("home_start_new_game") }</h2>
            <div class="flex justify-evenly gap-6">
                <Card class="h-fit min-w-sm  max-h-[86vh] overflow-hidden">
                    <CardHeader>
                        <CardTitle class="mb-8">
                            <div class="flex justify-between items-top">
                                <h3 class="text-2xl font-bold text-white">
                                    { language_ctx.t("common_game_details") }
                                </h3>
                                <GameDetailsModal  pgn_game={pgn_game.clone()} on_update={force_update_cb.clone()} />
                            </div>
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <crate::components::GameCard pgn_game={pgn_game.borrow().clone()}  />
                        <ShareGameModal pgn_game={pgn_game.borrow().clone()} />
                    </CardContent>
                </Card>
                <Card class="h-fit w-fit">
                    <CardHeader>
                    <div ref={board_ref} id="game" class="h-[66vh] aspect-square" />
                    </CardHeader>
                    <CardContent></CardContent>
                </Card>

            </div>
        </div>
    }
}
#[derive(Properties, PartialEq, Clone)]
pub struct GameFormProps {
    pub pgn_game: std::rc::Rc<std::cell::RefCell<rooky_core::RookyGame>>,
    pub on_update: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_close: Callback<()>,
}

#[function_component(GameForm)]
pub fn game_form(props: &GameFormProps) -> Html {
    let pgn_game = props.pgn_game.clone();
    let on_update = props.on_update.clone();

    // Local state for form fields
    let date_value = use_state(|| pgn_game.borrow().date.format("%Y-%m-%d").to_string());
    let white_value = use_state(|| pgn_game.borrow().white.clone());
    let black_value = use_state(|| pgn_game.borrow().black.clone());

    // Event details state
    let event_value = use_state(|| match &pgn_game.borrow().event {
        rooky_core::pgn_standards::PgnEvent::Named(name) => name.clone(),
        rooky_core::pgn_standards::PgnEvent::Casual
        | rooky_core::pgn_standards::PgnEvent::Unknown => String::new(),
    });

    let site_value = use_state(|| match &pgn_game.borrow().site {
        rooky_core::pgn_standards::PgnSite::Named(name) => name.clone(),
        rooky_core::pgn_standards::PgnSite::Unknown => String::new(),
    });

    let round_value = use_state(|| match &pgn_game.borrow().round {
        rooky_core::pgn_standards::PgnRound::Named(name) => name.clone(),
        rooky_core::pgn_standards::PgnRound::Unknown => String::new(),
    });

    let language_ctx = crate::contexts::language::use_language_ctx();

    // Form submission handler
    let onsubmit_handler = {
        let pgn_game = pgn_game.clone();
        let on_update = on_update;
        let on_close = props.on_close.clone();
        let date_value = date_value.clone();
        let white_value = white_value.clone();
        let black_value = black_value.clone();
        let event_value = event_value.clone();
        let site_value = site_value.clone();
        let round_value = round_value.clone();

        Callback::from(move |_form: web_sys::HtmlFormElement| {
            // Update game with form values
            let mut game = pgn_game.borrow_mut();

            // Update basic fields
            game.date =
                chrono::NaiveDate::parse_from_str(&date_value, "%Y-%m-%d").unwrap_or_default();
            game.white.clone_from(&(*white_value));
            game.black.clone_from(&(*black_value));

            // Update event details if provided
            let event = (*event_value).clone();
            if !event.is_empty() {
                game.event = rooky_core::pgn_standards::PgnEvent::Named(event);
                game.site = rooky_core::pgn_standards::PgnSite::Named((*site_value).clone());
                game.round = rooky_core::pgn_standards::PgnRound::Named((*round_value).clone());
            }

            // Notify parent component of update
            on_update.emit(());

            // Add success notification
            nostr_minions::widgets::toastify::ToastifyOptions::new_success("Game details updated")
                .show();

            // Close modal after brief delay
            let on_close = on_close.clone();
            yew::platform::spawn_local(async move {
                gloo::timers::future::TimeoutFuture::new(1000).await;
                on_close.emit(());
            });
        })
    };

    html! {
        <Card class="size-fit">
            <CardHeader>
                <CardTitle>
                    { language_ctx.t("edit_game_info") }
                </CardTitle>
            </CardHeader>
            <CardContent class="flex flex-col gap-2">
                <Form onsubmit={onsubmit_handler}>
                    <div class="mb-4">
                        <label class="block text-sm font-medium mb-1">{ language_ctx.t("game_details_date") }</label>
                        <Input
                            name="date"
                            r#type={shady_minions::ui::InputType::Date}
                            value={(*date_value).clone()}
                            class="w-full"
                            onchange={{
                                let date_value = date_value.clone();
                                Callback::from(move |e: String| {
                                    date_value.set(e);
                                })
                            }}
                        />
                    </div>

                    <div class="mb-4">
                        <label class="block text-sm font-medium mb-1">{ language_ctx.t("common_white") }</label>
                        <Input
                            name="white"
                            r#type={shady_minions::ui::InputType::Text}
                            placeholder={ language_ctx.t("common_white") }
                            value={(*white_value).clone()}
                            class="w-full"
                            onchange={{
                                let white_value = white_value.clone();
                                Callback::from(move |e: String| {
                                    white_value.set(e);
                                })
                            }}
                        />
                    </div>

                    <div class="mb-4">
                        <label class="block text-sm font-medium mb-1">{ language_ctx.t("common_black") }</label>
                        <Input
                            name="black"
                            r#type={shady_minions::ui::InputType::Text}
                            placeholder={ language_ctx.t("common_black") }
                            value={(*black_value).clone()}
                            class="w-full"
                            onchange={{
                                let black_value = black_value.clone();
                                Callback::from(move |e: String| {
                                    black_value.set(e);
                                })
                            }}
                        />
                    </div>

                    <div class="mb-6">
                        <Popover>
                            <PopoverTrigger>
                                <Button
                                    r#type={shady_minions::ui::ButtonType::Button}
                                    class="w-full"
                                    variant={shady_minions::ui::ButtonVariant::Outline}>
                                    { language_ctx.t("game_details_event") }
                                </Button>
                            </PopoverTrigger>
                            <PopoverContent>
                                <div class="space-y-4">
                                    <div>
                                        <label class="block text-sm font-medium mb-1">{ language_ctx.t("game_details_event") }</label>
                                        <Input
                                            name="event_name"
                                            r#type={shady_minions::ui::InputType::Text}
                                            placeholder={ language_ctx.t("game_details_enter_event_name") }
                                            value={(*event_value).clone()}
                                            class="w-full"
                                            onchange={{
                                                let event_value = event_value.clone();
                                                Callback::from(move |e: String| {
                                                    event_value.set(e);
                                                })
                                            }}
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-sm font-medium mb-1">{ language_ctx.t("game_details_site") }</label>
                                        <Input
                                            name="site"
                                            r#type={shady_minions::ui::InputType::Text}
                                            placeholder={ language_ctx.t("game_details_enter_site") }
                                            value={(*site_value).clone()}
                                            class="w-full"
                                            onchange={{
                                                let site_value = site_value.clone();
                                                Callback::from(move |e: String| {
                                                    site_value.set(e);
                                                })
                                            }}
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-sm font-medium mb-1">{ language_ctx.t("game_details_round") }</label>
                                        <Input
                                            name="round"
                                            r#type={shady_minions::ui::InputType::Text}
                                            placeholder={ language_ctx.t("game_details_enter_round") }
                                            value={(*round_value).clone()}
                                            class="w-full"
                                            onchange={{
                                                let round_value = round_value.clone();
                                                Callback::from(move |e: String| {
                                                    round_value.set(e);
                                                })
                                            }}
                                        />
                                    </div>
                                </div>
                            </PopoverContent>
                        </Popover>
                    </div>

                    <Button
                        r#type={shady_minions::ui::ButtonType::Submit}
                        class="w-full">
                        { language_ctx.t("common_save") }
                    </Button>
                </Form>
            </CardContent>
        </Card>
    }
}

#[function_component(GameDetailsModal)]
pub fn game_details_modal(props: &GameFormProps) -> Html {
    let is_open = use_state(|| false);

    // Creating modal close callback
    let close_modal = {
        let is_open = is_open.clone();
        Callback::from(move |()| is_open.set(false))
    };

    html! {
        <>
            <Button
                class="p-4"
                size={shady_minions::ui::ButtonSize::Small}
                onclick={
                    let is_open = is_open.clone();
                    Callback::from(move |_| {
                    is_open.set(!&*is_open);
                })}>
                <lucide_yew::SquarePen class="size-6" />
            </Button>
            <Modal {is_open}>
                <GameForm
                    on_close={close_modal}
                    ..props.clone()
                />
            </Modal>
        </>
    }
}

#[function_component(ShareGameModal)]
pub fn share_game_modal(props: &crate::components::GameCardProps) -> Html {
    let is_open = use_state(|| false);
    let game = props.pgn_game.clone();
    let language_ctx = crate::contexts::language::use_language_ctx();
    html! {
        <>
            <Button
                class="w-full"
                r#type={shady_minions::ui::ButtonType::Button}
                onclick={
                    let is_open = is_open.clone();
                    Callback::from(move |_| {
                    is_open.set(!&*is_open);
                })}>
                    <span class="text-sm font-bold text-white">{ language_ctx.t("finish_game") }</span>
            </Button>
            <Modal {is_open}>
                <ShareRookyGameCard  {game} />
            </Modal>
        </>
    }
}

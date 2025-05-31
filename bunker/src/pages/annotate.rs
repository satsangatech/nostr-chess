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
    let force_update_cb = {
        let force_update = force_update.clone();
        Callback::from(move |_| force_update.set(*force_update + 1))
    };

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
            if (turn == shakmaty::Color::White && piece_str.starts_with("b"))
                || (turn == shakmaty::Color::Black && piece_str.starts_with("w"))
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
        let board_setting = game_board.clone();
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

    html! {
        <div class="pl-12 h-full flex flex-col justify-evenly">
            <h2 class="text-4xl text-white font-black">{"New Game"}</h2>
            <div class="flex justify-evenly gap-6">
                <Card class="h-full min-w-sm max-w-sm">
                    <CardHeader>
                        <CardTitle class="mb-8">
                            <div class="flex justify-between items-top">
                                <h3 class="text-2xl font-bold text-white">
                                    {"Game Info"}
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
                <div class="flex-1 flex justify-center items-start">
                    <div ref={board_ref} id="game" class="w-full max-w-[42rem] aspect-square" />
                </div>
            </div>
        </div>
    }
}
#[derive(Properties, PartialEq, Clone)]
pub struct GameFormProps {
    pub pgn_game: std::rc::Rc<std::cell::RefCell<rooky_core::RookyGame>>,
    pub on_update: Callback<()>,
}

#[function_component(GameForm)]
pub fn game_form(props: &GameFormProps) -> Html {
    let pgn_game = props.pgn_game.clone();
    let on_update = props.on_update.clone();

    html! {
        <Card class="size-fit">
            <CardHeader >
                <CardTitle>
                    { "Edit Game Info" }
                </CardTitle>
            </CardHeader>
            <CardContent class="flex flex-col gap-2">
                <Input
                    r#type={shady_minions::ui::InputType::Date}
                    value={pgn_game.borrow().date.format("%Y-%m-%d").to_string()}
                    class="w-full"
                    onchange={{
                        let pgn_game = pgn_game.clone();
                        let on_update = on_update.clone();
                        Callback::from(move |e: String| {
                            pgn_game.borrow_mut().date = chrono::NaiveDate::parse_from_str(&e, "%Y-%m-%d").unwrap_or_default();
                            on_update.emit(());
                        })
                    }}
                />
                <Input
                    r#type={shady_minions::ui::InputType::Text}
                    placeholder={"White"}
                    class="w-full"
                    onchange={{
                        let pgn_game = pgn_game.clone();
                        let on_update = on_update.clone();
                        Callback::from(move |e: String| {
                           pgn_game.borrow_mut().white = e;
                           on_update.emit(());
                        })
                    }}
                />
                <Input
                    r#type={shady_minions::ui::InputType::Text}
                    placeholder={"Black"}
                    class="w-full"
                    onchange={{
                        let pgn_game = pgn_game.clone();
                        let on_update = on_update.clone();
                        Callback::from(move |e: String| {
                            pgn_game.borrow_mut().black = e;
                            on_update.emit(());
                        })
                    }}
                />
                <Popover >
                <PopoverTrigger>
                    <Button
                        class="w-full"
                        variant={shady_minions::ui::ButtonVariant::Outline}>

                        { "Event" }
                    </Button>
                    </PopoverTrigger>
                    <PopoverContent>
                        <Form
                            onsubmit={{
                                let pgn_game = pgn_game.clone();
                                let on_update = on_update.clone();
                                Callback::from(move |e: web_sys::HtmlFormElement| {
                                    let event_name = e.get_with_name("event_name").and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok()).unwrap();
                                    let site = e.get_with_name("site").and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok()).unwrap();
                                    let round = e.get_with_name("round").and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok()).unwrap();
                                    let event_name = event_name.value();
                                    let site = site.value();
                                    let round = round.value();
                                    pgn_game.borrow_mut().event = rooky_core::pgn_standards::PgnEvent::Named(event_name);
                                    pgn_game.borrow_mut().site = rooky_core::pgn_standards::PgnSite::Named(site);
                                    pgn_game.borrow_mut().round = rooky_core::pgn_standards::PgnRound::Named(round);
                                    on_update.emit(());
                                })
                            }}
                        >
                            <Input
                                name="event_name"
                                r#type={shady_minions::ui::InputType::Text}
                                placeholder={"Event"}
                                required={true}
                                class="w-full"
                            />
                            <Input
                                name="site"
                                r#type={shady_minions::ui::InputType::Text}
                                placeholder={"Site"}
                                required={true}
                                class="w-full"
                            />
                            <Input
                                name="round"
                                r#type={shady_minions::ui::InputType::Text}
                                placeholder={"Round"}
                                required={true}
                                class="w-full"
                            />
                            <Button
                                r#type={shady_minions::ui::ButtonType::Submit}
                                class="w-full"
                                variant={shady_minions::ui::ButtonVariant::Outline}>
                                { "Save" }
                            </Button>
                        </Form>
                    </PopoverContent>
                </Popover>
            </CardContent>
        </Card>
    }
}

#[function_component(GameDetailsModal)]
pub fn game_details_modal(props: &GameFormProps) -> Html {
    let is_open = use_state(|| false);
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
                <GameForm ..props.clone() />
            </Modal>
        </>
    }
}

#[function_component(ShareGameModal)]
pub fn share_game_modal(props: &crate::components::GameCardProps) -> Html {
    let is_open = use_state(|| false);
    let game = props.pgn_game.clone();
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
                    <span class="text-sm font-bold text-white">{ "Finish Game" }</span>
            </Button>
            <Modal {is_open}>
                <ShareRookyGameCard  {game} />
            </Modal>
        </>
    }
}

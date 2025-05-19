use crate::router::AnnotatorRoute;
use shady_minions::ui::{
    Button, Card, CardContent, CardHeader, CardTitle, Input, InputType, LeftDrawer, Modal, Switch,
    Tabs, TabsContent, TabsList, TabsTrigger,
};
use web_sys::wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let config_ctx = crate::configs::use_annotator_config();
    html! {
        <>
            <Tabs default_value={config_ctx.experience_level.to_string()}
                class="flex flex-col size-full bg-muted">
                <HomeHeader />
                <div class="container flex-1 p-3 pb-6">
                    <TabsContent
                        class="size-full"
                        value={crate::configs::ExperienceLevel::Rookie.to_string()}>
                        <RookieAnnotation />
                    </TabsContent>
                    <TabsContent
                        class="size-full flex"
                        value={crate::configs::ExperienceLevel::Expert.to_string()}>
                        <ExpertAnnotation />
                    </TabsContent>
                </div>
            </Tabs>
        </>
    }
}

#[function_component(HomeHeader)]
pub fn home_header() -> Html {
    html! {
        <header class="flex justify-between items-center p-3">
            <SettingsDrawer />
            <ExperienceSelector />
            <GameDetailsModal />
        </header>
    }
}

#[function_component(SettingsDrawer)]
pub fn settings_drawer() -> Html {
    let is_open = use_state(|| false);
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(!*is_open);
        })
    };
    let navigator = use_navigator().unwrap();
    let config_ctx = crate::contexts::configs::use_annotator_config();

    let go_to_key_recovery = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| navigator.push(&AnnotatorRoute::KeySettings))
    };

    let set_experience_level = {
        let config_ctx = config_ctx.clone();
        move |level: crate::contexts::configs::ExperienceLevel| {
            config_ctx.dispatch(
                crate::contexts::configs::AnnotatorConfigAction::SetExperienceLevel(level),
            );
        }
    };

    let is_expert =
        config_ctx.experience_level == crate::contexts::configs::ExperienceLevel::Expert;

    let language_ctx = crate::contexts::language::use_language_ctx();

    html! {
        <>
            <Button {onclick}>
                <lucide_yew::Menu class="size-4" />
            </Button>
            <LeftDrawer {is_open} >
                <h3 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4">{language_ctx.t("common_settings")}</h3>
                <div class="space-y-4 w-full">
                    <div class="rounded-lg bg-slate-50 p-3 sm:p-4 shadow-sm">
                        <div class="mb-1.5 sm:mb-2">
                            <label class="text-sm sm:text-base font-medium block mb-0.5 sm:mb-1">
                                { language_ctx.t("settings_default_level") }
                            </label>
                            <p class="text-xs sm:text-sm text-muted-foreground">
                                { language_ctx.t("settings_default_level_description") }
                            </p>
                        </div>
                        <div class="mt-2">
                            <div class="flex items-center justify-between px-1">
                                <div class="flex items-center">
                                    <span class="text-sm font-medium mr-2 sm:mr-3">{ language_ctx.t("common_rookie") }</span>
                                    <Switch
                                        checked={is_expert}
                                        onchange={
                                            let set_experience_level = set_experience_level.clone();
                                            Callback::from(move |checked: bool| {
                                                if checked {
                                                    set_experience_level(crate::contexts::configs::ExperienceLevel::Expert);
                                                } else {
                                                    set_experience_level(crate::contexts::configs::ExperienceLevel::Rookie);
                                                }
                                            })
                                        }
                                    />
                                </div>
                                <span class="text-sm font-medium ml-2 sm:ml-3">{ language_ctx.t("common_expert") }</span>
                            </div>
                        </div>
                    </div>

                    <Button
                        onclick={go_to_key_recovery}
                        class="w-full flex items-center justify-start py-2 px-3 sm:py-3 sm:px-4 h-auto bg-slate-100 border border-slate-200 text-sm sm:text-base shadow-sm break-words"
                        variant={shady_minions::ui::ButtonVariant::Outline}
                    >
                        <lucide_yew::Key class="w-4 h-4 sm:w-5 sm:h-5 mr-1.5 sm:mr-2 flex-shrink-0 text-primary" />
                        <span class="font-medium truncate">{ language_ctx.t("settings_key_recovery") }</span>
                    </Button>
                </div>
            </LeftDrawer>
        </>
    }
}

#[function_component(ExperienceSelector)]
pub fn experience_selector() -> Html {
    // Get language context
    let language_ctx = crate::contexts::language::use_language_ctx();

    html! {
            <TabsList class="flex flex-1">
                <TabsTrigger value={crate::configs::ExperienceLevel::Rookie.to_string()}>
                    { language_ctx.t("common_rookie") }
                </TabsTrigger>
                <TabsTrigger value={crate::configs::ExperienceLevel::Expert.to_string()}>
                    { language_ctx.t("common_expert") }
                </TabsTrigger>
            </TabsList>

    }
}

#[function_component(GameDetailsModal)]
pub fn game_details_modal() -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let game = game_ctx.pgn_game();
    let is_open = use_state(|| true);
    let language_ctx = crate::contexts::language::use_language_ctx();
    html! {
        <>
            <Button
                onclick={
                    let is_open = is_open.clone();
                    Callback::from(move |_| {
                    is_open.set(true);
                })}>
                <lucide_yew::Cog class="size-4" />
            </Button>
        <Modal {is_open}>
            <Card>
                <CardHeader>
                    <CardTitle>
                        { language_ctx.t("common_game_details") }
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    <div class="flex flex-col">
                        <p>{ format!("{}: {}", language_ctx.t("game_details_event"), game.event) }</p>
                        <p>{ format!("{}: {}", language_ctx.t("game_details_site"), game.site) }</p>
                        <p>{ format!("{}: {}", language_ctx.t("game_details_date"), game.date) }</p>
                        <p>{ format!("{}: {}", language_ctx.t("game_details_white"), game.white) }</p>
                        <p>{ format!("{}: {}", language_ctx.t("game_details_black"), game.black) }</p>
                    </div>
                </CardContent>
            </Card>
        </Modal>
        </>
    }
}

#[function_component(RookieAnnotation)]
pub fn rookie_annotation() -> Html {
    let next_role = use_state(|| None::<shakmaty::Role>);
    let language_ctx = crate::contexts::language::use_language_ctx();
    let clear_role = {
        let next_role = next_role.clone();
        Callback::from(move |_| {
            next_role.set(None);
        })
    };
    html! {
        <>
            {match next_role.as_ref() {
            Some(role) => {
                html! {
                    <div class="flex flex-col p-3 items-center size-full max-w-md mx-auto">
                        <Card>
                            <CardContent>
                                <h3 class={"text-2xl"}>{format!("{role:#?}")}</h3>
                            </CardContent>
                        </Card>
                        <h3 class="mb-4">{ language_ctx.t("annotation_select_move") }</h3>
                        <Button
                            class="float-left text-white"
                            onclick={clear_role}>
                            <lucide_yew::ArrowLeft class="size-4" />
                        </Button>
                        <MoveSelection {next_role} />
                        <MoveList />
                    </div>
                }
            },
            None => {
                html! {
                    <>
                    <PieceSelection {next_role} />
                    <MoveList />
                    </>
                }
            }}}
        </>
    }
}
#[derive(Properties, PartialEq)]
pub struct PieceSelectionProps {
    pub next_role: UseStateHandle<Option<shakmaty::Role>>,
}

#[function_component(PieceSelection)]
pub fn piece_selection(props: &PieceSelectionProps) -> Html {
    let set_role = {
        let next_role = props.next_role.clone();
        Callback::from(move |role: shakmaty::Role| {
            next_role.set(Some(role));
        })
    };

    let language_ctx = crate::contexts::language::use_language_ctx();
    html! {
        <div class="flex flex-col p-3 items-center w-full max-w-md mx-auto">
            <h3 class="mb-4">{ language_ctx.t("annotation_select_piece") }</h3>
            <div class="grid grid-cols-2 gap-4 w-full">
                <PieceSelector
                    piece={shakmaty::Role::Pawn}
                    onclick={set_role.clone()} />
                <PieceSelector
                    piece={shakmaty::Role::Knight}
                    onclick={set_role.clone()} />
                <PieceSelector
                    piece={shakmaty::Role::Bishop}
                    onclick={set_role.clone()} />
                <PieceSelector
                    piece={shakmaty::Role::Rook}
                    onclick={set_role.clone()} />
                <PieceSelector
                    piece={shakmaty::Role::Queen}
                    onclick={set_role.clone()} />
                <PieceSelector
                    piece={shakmaty::Role::King}
                    onclick={set_role.clone()} />
            </div>
        </div>
    }
}
#[derive(Properties, PartialEq)]
pub struct PieceSelectorProps {
    pub piece: shakmaty::Role,
    pub onclick: Callback<shakmaty::Role>,
}

#[function_component(PieceSelector)]
pub fn piece_selector(props: &PieceSelectorProps) -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let language_ctx = crate::contexts::language::use_language_ctx();
    let PieceSelectorProps { piece, onclick } = props;
    let can_be_moved = game_ctx.legal_moves().iter().any(|m| &m.role() == piece);
    let image_url = format!(
        "/public/assets/img/{}{}.svg",
        game_ctx.color_turn().char(),
        piece.upper_char()
    );

    html! {
        <Button
            class={classes!(
                if can_be_moved {
                    match game_ctx.color_turn() {
                        shakmaty::Color::White => "bg-white",
                        shakmaty::Color::Black => "bg-zinc-900",
                    }
                } else {
                    "bg-zinc-200"
                },
                if can_be_moved {
                    match game_ctx.color_turn() {
                        shakmaty::Color::White => "text-zinc-900",
                        shakmaty::Color::Black => "text-white",
                    }
                } else {
                    "text-zinc-400"
                },
                "p-2",
                "rounded",
                "aspect-square",
                "w-full",
                "h-full",
                "flex",
                "items-center",
                "justify-center",
                "flex-col",
                if can_be_moved {
                    "cursor-pointer"
                } else {
                    "cursor-not-allowed"
                }
            )}
            onclick={
                let onclick = onclick.clone();
                let piece_clone = *piece;
                Callback::from(move |_| {
                    if can_be_moved {
                        onclick.emit(piece_clone);
                    }
                })
            } >
            <img
                src={image_url}
                alt={format!("{:?}", piece)}
                class={classes!(
                    if can_be_moved {
                        "opacity-100"
                    } else {
                        "opacity-30"
                    },
                    "p-2",
                    "rounded-full",
                    "bg-white/20",
                    "size-16",
                    "object-cover")}
            />
            <span class="text-sm text-center mt-2">
            { match piece {
                shakmaty::Role::Pawn => language_ctx.t("pieces_pawn"),
                shakmaty::Role::Knight => language_ctx.t("pieces_knight"),
                shakmaty::Role::Bishop => language_ctx.t("pieces_bishop"),
                shakmaty::Role::Rook => language_ctx.t("pieces_rook"),
                shakmaty::Role::Queen => language_ctx.t("pieces_queen"),
                shakmaty::Role::King => language_ctx.t("pieces_king"),
            } }
            </span>
        </Button>
    }
}

#[function_component(MoveSelection)]
pub fn move_selection(props: &PieceSelectionProps) -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let language_ctx = crate::contexts::language::use_language_ctx();
    let next_move = use_state(|| None::<shakmaty::Move>);
    let legal_moves = game_ctx.legal_moves();
    let legal_moves = legal_moves
        .iter()
        .filter(|m| m.role() == props.next_role.unwrap())
        .cloned()
        .collect::<Vec<_>>();

    // Store the moves in a UseState hook for immutability
    html! {
        <div class="flex-1 flex flex-col justify-between">
        <div class="flex w-full h-full gap-2 overflow-x-auto max-w-md">
        {
            legal_moves.iter().cloned().map(|m| {
                let next_move_clone = next_move.clone();
                let is_selected = next_move.as_ref() == Some(&m);
                let onclick = {
                    let m = m.clone();
                    Callback::from(move |_| {
                        if next_move_clone.as_ref() == Some(&m) {
                            next_move_clone.set(None);
                        } else {
                            next_move_clone.set(Some(m.clone()));
                        };
                    })
                };
                html! {
                    <Button
                        class={classes!(
                            if is_selected {
                                "bg-green-500"
                            } else {
                                "bg-white"
                            },
                            "p-2",
                            "flex-1",
                            "rounded",
                            "aspect-square",
                            "h-full",
                            "max-w-xs",
                            "max-h-96",
                            "m-4",
                            "flex",
                            "items-center",
                            "justify-center",
                            "flex-col",
                            if is_selected {
                                "cursor-not-allowed"
                            } else {
                                "cursor-pointer"
                            }
                        )}
                        onclick={onclick}>
                        // <img
                        //     src={format!("/public/img/moves/{m}.png", )}
                        //     alt={format!("{:?}", m)}
                        //     class="size-16 object-cover"
                        // />
                        <span class="text-2xl text-center mt-2 text-black">
                            { m.to_string() }
                        </span>
                        <SquaresPreview next_move={m.clone()} />
                    </Button>
                }
            }).collect::<Html>()
        }
        </div>
        <Button
            class={classes!(
                if next_move.is_some() {
                    "bg-green-500"
                } else {
                    "bg-zinc-400"
                },
                if next_move.is_some() {
                    "cursor-pointer"
                } else {
                    "cursor-not-allowed"
                },
                if next_move.is_none() {
                    "pointer-events-none"
                } else {
                    "pointer-events-auto"
                },
                "p-2",
                "rounded",
                "flex",
                "items-center",
                "justify-center",
                "flex-col",
            )}
            onclick={
                let next_move = next_move.clone();
                let next_role = props.next_role.clone();
                let game_ctx = game_ctx.clone();
                Callback::from(move |_| {
                    if let Some(m) = next_move.as_ref() {
                        game_ctx.dispatch(crate::live_game::AnnotatedGameAction::PlayMove(m.clone()));
                    }
                    next_move.set(None);
                    next_role.set(None);
                })
            }>
            <span class="text-sm text-center mt-2 text-white">
                { language_ctx.t("annotation_play_move") }
            </span>
        </Button>
        </div>
    }
}

#[function_component(MoveList)]
pub fn move_list() -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let moves = game_ctx.pgn_game().moves.clone();

    // Create a reference to the container for scrolling
    let container_ref = use_node_ref();

    // Use effect to scroll to the end when moves update
    {
        let container_ref = container_ref.clone();
        let moves_len = moves.len();
        use_effect_with(moves_len, move |_| {
            if let Some(container) = container_ref.cast::<web_sys::HtmlElement>() {
                let scroll_width = container.scroll_width();
                container.set_scroll_left(scroll_width);
            }
            || ()
        });
    }

    html! {
        <div
            ref={container_ref}
            class="flex flex-row p-3 items-center w-full mx-auto overflow-x-auto whitespace-nowrap gap-3 pb-2 max-w-sm"
        >
            { for moves.chunks(2).enumerate().map(|(index, m)| {
                let white_move = m.first().expect("White move");
                let black_move = m.get(1);
                let is_last_chunk = index == moves.chunks(2).count() - 1;

                let move_number = index + 1;

                let white_class = if is_last_chunk && black_move.is_none() {
                    "text-lg font-semibold"
                } else {
                    "text-sm"
                };

                let black_class = if is_last_chunk && black_move.is_some() {
                    "text-lg font-semibold"
                } else {
                    "text-sm"
                };

                html! {
                    <div class="inline-flex items-center">
                        <span class="text-gray-500 mr-1 text-xs">{ format!("{}.", move_number) }</span>
                        <span class={classes!(white_class, "mr-1")}>{ white_move.to_string() }</span>
                        { if let Some(black_move) = black_move {
                            html! { <span class={black_class}>{ black_move.to_string() }</span> }
                        } else {
                            html! {}
                        }}
                    </div>
                }
            })}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SquaresPreviewProps {
    pub next_move: shakmaty::Move,
}

#[function_component(SquaresPreview)]
pub fn squares_preview(props: &SquaresPreviewProps) -> Html {
    let board_ref = use_node_ref();
    let next_move = props.next_move.clone();
    let game_ctx = crate::live_game::use_annotated_game();
    let role = next_move.role();
    let color = game_ctx.color_turn();
    let Some(from_square) = next_move.from() else {
        return html! {};
    };
    let to_square = next_move.to();
    let board_id = format!("game-{next_move}");
    let piece_image_url = format!(
        "/public/assets/img/{}{}.svg",
        color.char(),
        role.upper_char()
    );
    let game_board = use_mut_ref(|| None::<chessboard_js::ChessBoardJs>);

    {
        let board_setting = game_board.clone();
        let board_id = board_id.clone();
        use_effect_with((), move |()| {
            let board_options = chessboard_js::ChessboardConfig {
                draggable: false,
                position: chessboard_js::ChessboardPosition::Empty,
                ..Default::default()
            };
            let board = chessboard_js::ChessBoardJs::new(&board_id, Some(board_options));
            *board_setting.borrow_mut() = Some(board);

            || {}
        });
    }
    {
        let piece_image_url = piece_image_url.clone();
        use_effect_with(
            (board_ref.clone(), next_move.clone()),
            move |(board_ref, _next_move)| {
                if let Some(_board) = game_board.borrow_mut().as_mut() {
                    let root_ele = board_ref.cast::<web_sys::HtmlElement>().unwrap();
                    let from_ele = root_ele
                        .clone()
                        .get_elements_by_class_name(&format!("square-{}", from_square))
                        .unchecked_into::<web_sys::js_sys::Array>()
                        .to_vec();
                    let from_ele = from_ele
                        .first()
                        .unwrap()
                        .unchecked_ref::<web_sys::HtmlElement>();
                    let to_ele = root_ele
                        .clone()
                        .get_elements_by_class_name(&format!("square-{}", to_square))
                        .unchecked_into::<web_sys::js_sys::Array>()
                        .to_vec();
                    let to_ele = to_ele
                        .first()
                        .unwrap()
                        .unchecked_ref::<web_sys::HtmlElement>();
                    let to_classes =
                        classes!(to_ele.class_name(), "border", "border-4", "border-blue-500");
                    to_ele.set_class_name(&to_classes.to_string());

                    let document = web_sys::window().unwrap().document().unwrap();
                    // Clear the inner HTML of the square.
                    from_ele.set_inner_html("");
                    // A div to contain the piece image with proper styling
                    let container = document.create_element("div").unwrap();
                    let container_div = container.dyn_into::<web_sys::HtmlElement>().unwrap();
                    container_div
                        .set_attribute("style", "width: 100%; height: 100%; position: relative;")
                        .unwrap();
                    let img = document.create_element("img").unwrap();
                    let img_element = img.dyn_into::<web_sys::HtmlElement>().unwrap();
                    img_element.set_attribute("src", &piece_image_url).unwrap();
                    img_element.set_attribute(
                        "style",
                        "position: absolute; width: 80%; height: 80%; top: 10%; left: 10%; z-index: 2;"
                    ).unwrap();
                    container_div.append_child(&img_element).unwrap();
                    from_ele.append_child(&container_div).unwrap();
                }
                || {}
            },
        );
    }

    html! {
        <div ref={board_ref} id={board_id} class="size-full aspect-square" />
    }
}

#[function_component(ExpertAnnotation)]
pub fn expert_annotation() -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let next_move = use_state(|| None::<String>);
    let legal_moves = use_state(|| game_ctx.legal_moves());
    let last_position = game_ctx.last_game_position();
    let ready_move = use_state(|| None::<shakmaty::Move>);

    {
        let legal_moves = legal_moves.clone();
        let last_position = last_position.clone();
        use_effect_with(next_move.clone(), move |next| {
            if let Some(next) = next.as_ref() {
                let mut new_moves = (*legal_moves).clone();
                new_moves.retain(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().strip_prefix(next).is_some()
                });
                legal_moves.set(new_moves);
            } else {
                legal_moves.set(game_ctx.legal_moves());
            }
            || {}
        });
    }

    {
        let legal_moves = legal_moves.clone();
        let ready_move = ready_move.clone();
        use_effect_with(next_move.clone(), move |next| {
            if let Some(next) = next.as_ref() {
                let matching_move = legal_moves
                    .iter()
                    .filter_map(|m| {
                        let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                        san.to_string().strip_prefix(next)?;
                        Some(m.clone())
                        //san.to_string().contains(next).then_some(Some(m))
                    })
                    .collect::<Vec<_>>();
                if matching_move.len() == 1 {
                    let m = matching_move.first().cloned();
                    ready_move.set(m);
                }
            }
            || {}
        });
    }

    let props = ExpertAnnotationProps {
        next_move,
        ready_move: ready_move.clone(),
        legal_moves: legal_moves.clone(),
    };

    html! {
        <div class="flex flex-col flex-1 justify-between">
            <div class="space-y-2">
                <MoveList />
                <AnnotationDisplay ..props.clone() />
            </div>
            <AnnotationCalculator ..props />
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct ExpertAnnotationProps {
    pub next_move: UseStateHandle<Option<String>>,
    pub legal_moves: UseStateHandle<Vec<shakmaty::Move>>,
    pub ready_move: UseStateHandle<Option<shakmaty::Move>>,
}

#[function_component(AnnotationDisplay)]
pub fn annotation_display(props: &ExpertAnnotationProps) -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let language_ctx = crate::contexts::language::use_language_ctx();
    let last_position = game_ctx.last_game_position();
    let ExpertAnnotationProps {
        next_move,
        legal_moves,
        ready_move: _,
    } = props;
    let is_selecting = next_move.is_some();

    // Format legal moves as a string
    let legal_moves_text = if is_selecting {
        legal_moves
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, m)| {
                let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                if i == 0 {
                    format!("{acc}: {san}",)
                } else {
                    format!("{acc}, {san}")
                }
            })
    } else {
        language_ctx.t("annotation_select_piece").to_string()
    };

    html! {
        <div class="relative w-full">
            <Input
                class={classes!(
                    "w-full",
                    "pr-4", // Add padding on the right to prevent text overlap
                    "pl-4", // Add padding on the left
                )}
                disabled={true}
                value={""} // Keep the actual input value empty
            />

            <div class="absolute inset-0 flex items-center px-4">
                // Left side - next move
                <div class="flex-shrink-0 mr-2">
                    <span class="text-xl font-medium">
                        { next_move.as_ref().cloned().unwrap_or_default() }
                    </span>
                </div>

                // Right side - legal moves (with truncation)
                <div class="flex-grow flex justify-end">
                    <span class="text-xl text-gray-500 truncate max-w-xs">
                        { legal_moves_text }
                    </span>
                </div>
            </div>
        </div>
    }
}

#[function_component(SanMoveBlocks)]
pub fn san_move_blocks(props: &ExpertAnnotationProps) -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let last_position = game_ctx.last_game_position();
    let ExpertAnnotationProps {
        next_move: _,
        legal_moves,
        ready_move: _,
    } = props;

    html! {
        <div class="text-xl font-bold grid grid-cols-3 gap-4">
            {legal_moves.iter().map(|m| {
                let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                html! {
                    <Button
                        class={classes!(
                            "p-2",
                            "rounded",
                            "aspect-square",
                            "w-full",
                            "h-full",
                            "flex",
                            "items-center",
                            "justify-center",
                            "flex-col"
                        )}>
                        <span>{san.to_string()}</span>
                    </Button>
                }
            }).collect::<Html>()}
        </div>
    }
}

#[function_component(AnnotationCalculator)]
pub fn annotation_calculator(props: &ExpertAnnotationProps) -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let last_position = game_ctx.last_game_position();
    let ExpertAnnotationProps {
        next_move,
        legal_moves,
        ready_move,
    } = props;
    let onclick = {
        let next_move = next_move.clone();
        let legal_moves = legal_moves.clone();
        let last_position = last_position.clone();
        Callback::from(move |input_event: String| {
            if legal_moves.iter().any(|m| {
                let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                san.to_string().contains(&input_event)
            }) {
                let mut new_move = (*next_move).clone().unwrap_or_default();
                new_move.push_str(&input_event);
                next_move.set(Some(new_move));
            }
        })
    };
    let play_move = {
        let ready_move = ready_move.clone();
        let next_move = next_move.clone();
        Callback::from(move |_| {
            if let Some(m) = ready_move.as_ref() {
                game_ctx.dispatch(crate::live_game::AnnotatedGameAction::PlayMove(m.clone()));
            }
            next_move.set(None);
            ready_move.set(None);
        })
    };
    let clear = {
        let next_move = next_move.clone();
        Callback::from(move |_| {
            next_move.set(None);
        })
    };
    let input_class = classes!("h-fit", "text-6xl", "font-bold",);
    html! {
        <div class="">
            <div class="grid grid-cols-5">
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("N")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"N"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("B")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"B"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("R")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"R"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("Q")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"Q"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("K")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"K"} />
            </div>
            <div class="grid grid-cols-4">
            <Input
                class={input_class.clone()}
                onclick={onclick.clone()}
                r#type={InputType::Button}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("a")})
                }
                value={"a"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("b")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"b"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("c")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"c"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("d")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"d"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("e")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"e"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("f")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"f"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("g")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"g"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("h")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"h"} />
            </div>
            <div class="grid grid-cols-4">
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("1")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"1"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("2")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"2"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("3")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"3"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("4")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"4"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("5")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"5"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("6")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"6"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("7")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"7"} />
            <Input
                class={input_class.clone()}
                disabled={
                    legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    !san.to_string().contains("8")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"8"} />
            </div>
            <div class="grid grid-cols-5">
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("x")})
                }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"x"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("+")
                }) }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"+"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("#")
                }) }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"#"} />
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter().any(|m| {
                    let san = shakmaty::san::SanPlus::from_move(last_position.clone(), m);
                    san.to_string().contains("O")
                }) }
                onclick={onclick.clone()}
                r#type={InputType::Button}
                value={"O"} />
            <Button
                class={classes!(
                    "w-full",
                    "h-full",
                )}
                variant={shady_minions::ui::ButtonVariant::Destructive}
                onclick={clear} >
                <lucide_yew::Delete class="size-12" />
            </Button>
            <Button
                class={classes!(
                    "w-full",
                    "h-full",
                    if ready_move.is_some() {
                        ""
                    } else {
                        "bg-zinc-400"
                    },
                    if ready_move.is_some() {
                        "cursor-pointer"
                    } else {
                        "cursor-not-allowed"
                    },
                    if ready_move.is_none() {
                        "pointer-events-none"
                    } else {
                        "pointer-events-auto"
                    },
                )}
                onclick={play_move} >
                <lucide_yew::Send class="size-12" />
            </Button>
            </div>
        </div>
    }
}

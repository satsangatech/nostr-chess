use yew::prelude::*;

#[function_component(RookieAnnotation)]
pub fn rookie_annotation() -> Html {
    let next_move = use_state(|| None::<shakmaty::Move>);
    let next_role = use_state(|| None::<shakmaty::Role>);
    let next_from_square = use_state(|| None::<shakmaty::Square>);
    let language_ctx = crate::contexts::language::use_language_ctx();
    let game_ctx = crate::live_game::use_annotated_game();

    let clear_role = {
        let next_role = next_role.clone();
        let next_from_square = next_from_square.clone();
        Callback::from(move |_| {
            next_role.set(None);
            next_from_square.set(None);
        })
    };

    let clear_from_square = {
        let next_from_square = next_from_square.clone();
        let next_move = next_move.clone();
        Callback::from(move |_| {
            next_from_square.set(None);
            next_move.set(None);
        })
    };

    let set_next_from_square = {
        let next_from_square = next_from_square.clone();
        Callback::from(move |square: shakmaty::Square| {
            next_from_square.set(Some(square));
        })
    };

    let back_option = match (next_role.as_ref(), next_from_square.as_ref()) {
        (Some(_), Some(_)) => html! {
            <shady_minions::ui::Button
                size={shady_minions::ui::ButtonSize::Icon}
                onclick={clear_from_square}>
                <lucide_yew::ArrowLeft
                    class="size-6" />
            </shady_minions::ui::Button>
        },
        (Some(_), None) => html! {
            <shady_minions::ui::Button
                size={shady_minions::ui::ButtonSize::Icon}
                onclick={clear_role}>
                <lucide_yew::ArrowLeft
                    class="size-6" />
            </shady_minions::ui::Button>
        },
        _ => html! {
                <div class="w-2"></div> // Spacer for centering
        },
    };
    let inner_html = match (next_role.as_ref(), next_from_square.as_ref()) {
        (Some(role), None) => html! {
            <FromSquareSelection piece={*role} onclick={set_next_from_square} />
        },
        (Some(_), Some(_)) => html! {
            <MoveSelection
                next_move={next_move.clone()}
                next_role={next_role.clone()}
                next_from_square={next_from_square.clone()} />
        },
        _ => html! {
            <PieceSelection next_role={next_role.clone()} />
        },
    };

    let role_html = if let Some(role) = next_role.as_ref() {
        let src = format!(
            "/public/assets/img/{}{}.svg",
            game_ctx.color_turn().char(),
            role.upper_char()
        );
        html! {
            <img
                src={src}
                alt={format!("{:?}", role)}
                class="p-2 rounded-full size-8 object-cover" />
        }
    } else {
        html! {}
    };
    let next_from_square_html = if let Some(square) = next_from_square.as_ref() {
        html! {
            <h3 class="text-center size-6 p-1 font-bold text-muted">
                { format!("{:?}", square) }
            </h3>
        }
    } else {
        html! {}
    };

    let move_html = if let Some(mv) = next_move.as_ref() {
        let san_move = shakmaty::san::SanPlus::from_move(game_ctx.last_game_position(), mv);
        html! {
            <h3 class="text-center size-6 p-1 font-bold text-muted">
                { format!("{san_move}") }
            </h3>
        }
    } else {
        html! {}
    };
    let header_html = match next_role.as_ref() {
        Some(_) => html! {
            <div class="w-full flex gap-3 items-center max-w-sm">
                // Back button
                {back_option}
                // Preview Information Card
                <div class="flex-1 bg-background rounded-lg shadow-sm">
                    <div class="p-1">
                        <div class="grid grid-cols-3 gap-1">
                            <div class="text-center">
                                <label class="text-xs font-medium text-foreground block">{"Role"}</label>
                                {role_html}
                            </div>

                            <div class="text-center items-center flex flex-col">
                                <label class="text-xs font-medium text-foreground block">{"From Square"}</label>
                                {next_from_square_html}
                            </div>

                            <div class="text-center items-center flex flex-col">
                                <label class="text-xs font-medium text-foreground block">{"Move"}</label>
                                {move_html}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        },
        None => html! {
            <div class="w-full flex items-center justify-center text-center">
                <h2 class="text-lg font-bold text-white">
                    { language_ctx.t("annotation_select_piece") }
                </h2>
            </div>
        },
    };

    html! {
        <div class="flex flex-col items-center flex-1 gap-3 max-w-2xl">
            // Header with Back Control
            {header_html}

            // Main Content Area
            <shady_minions::ui::Card class="p-3 min-w-xs w-full max-w-sm">
                {inner_html}
            </shady_minions::ui::Card>
            // Play Move Button
            {if next_role.is_some() {
                html! {
                    <PlayMoveButton
                        next_move={next_move.clone()}
                        next_role={next_role.clone()}
                        next_from_square={next_from_square.clone()} />
                }
            } else {
                html! {
                }
            }}
        </div>
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

    html! {
        <div class="grid grid-cols-2 gap-3 w-full justify-center align-center items-center place-items-center">
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
        <button
            class={classes!(
                if can_be_moved {
                    match game_ctx.color_turn() {
                        shakmaty::Color::White => "bg-white hover:bg-secondary",
                        shakmaty::Color::Black => "bg-black hover:bg-secondary",
                    }
                } else {
                    "bg-muted"
                },
                if can_be_moved {
                    match game_ctx.color_turn() {
                        shakmaty::Color::White => "text-black/100",
                        shakmaty::Color::Black => "text-white",
                    }
                } else {
                    "text-muted"
                },
                "p-4",
                "rounded",
                "aspect-square",
                "min-h-28",
                "min-w-28",
                "flex",
                "items-center",
                "justify-center",
                "font-semibold",
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
                    "rounded-full",
                    "size-8",
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
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct FromSquareSelectionProps {
    pub piece: shakmaty::Role,
    pub onclick: Callback<shakmaty::Square>,
}

#[function_component(FromSquareSelection)]
pub fn from_square_selection(props: &FromSquareSelectionProps) -> Html {
    let configs = use_context::<crate::contexts::configs::AnnotatorConfigStore>()
        .expect("AnnotatorConfigStore context not found");
    let orientation = match configs.playing_as {
        crate::configs::BoardPlayingSide::White => chessboard_js::ChessboardOrientation::White,
        crate::configs::BoardPlayingSide::Black => chessboard_js::ChessboardOrientation::Black,
    };
    let board_ref = use_node_ref();
    let game_ctx = crate::live_game::use_annotated_game();
    let legal_moves = game_ctx.legal_moves();
    let from_squares = legal_moves
        .iter()
        .filter_map(|m| (m.role() == props.piece).then(|| m.from()).flatten())
        .collect::<std::collections::HashSet<_>>();
    let board_id = "from-selector";
    let game_board = use_mut_ref(|| None::<chessboard_js::ChessBoardJs>);
    let color = game_ctx.color_turn();
    let mut empty_board = shakmaty::Board::empty();
    for square in from_squares.iter().copied() {
        empty_board.set_piece_at(
            square,
            shakmaty::Piece {
                color,
                role: props.piece,
            },
        );
    }

    // Setting up the board
    {
        let board_setting = game_board.clone();
        use_effect_with((), move |()| {
            let fen_board = empty_board.board_fen(shakmaty::Bitboard::default());
            let board_options = chessboard_js::ChessboardConfig {
                draggable: false,
                position: chessboard_js::ChessboardPosition::Fen(fen_board.to_string()),
                orientation,
                ..Default::default()
            };
            let board = chessboard_js::ChessBoardJs::new(board_id, Some(board_options));
            *board_setting.borrow_mut() = Some(board);
            || {}
        });
    }
    // Highlighting squares and handle clicks
    {
        let onclick = props.onclick.clone();
        use_effect_with(
            (board_ref.clone(), from_squares.clone()),
            move |(board_ref, squares)| {
                if let Some(root_ele) = board_ref.cast::<web_sys::HtmlElement>() {
                    for from_sq in squares.iter().copied() {
                        let class_name = format!("square-{}", from_sq);
                        let squares = root_ele.get_elements_by_class_name(&class_name);
                        for i in 0..squares.length() {
                            let Some(html_ele) = squares.item(i).and_then(|el| {
                                web_sys::wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(el)
                                    .ok()
                            }) else {
                                continue;
                            };
                            let sq = from_sq;
                            let onclick = onclick.clone();
                            let closure = web_sys::wasm_bindgen::closure::Closure::wrap(Box::new(
                                move |_event: web_sys::MouseEvent| {
                                    onclick.emit(sq);
                                },
                            )
                                as Box<dyn FnMut(_)>)
                            .into_js_value();
                            html_ele.set_onclick(Some(
                                web_sys::wasm_bindgen::JsCast::unchecked_ref(&closure),
                            ));
                        }
                    }
                }
                || {}
            },
        );
    }
    html! {
        <div ref={board_ref} id={board_id} class="h-[35vh] sm:h-[45vh] aspect-square mx-auto" />
    }
}

#[derive(Properties, PartialEq)]
pub struct MoveSelectionProps {
    pub next_role: UseStateHandle<Option<shakmaty::Role>>,
    pub next_from_square: UseStateHandle<Option<shakmaty::Square>>,
    pub next_move: UseStateHandle<Option<shakmaty::Move>>,
}

#[function_component(MoveSelection)]
pub fn move_selection(props: &MoveSelectionProps) -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let next_move = props.next_move.clone();
    let legal_moves = game_ctx.legal_moves();
    let legal_moves = legal_moves
        .iter()
        .filter(|m| {
            Some(&m.role()) == props.next_role.as_ref()
                && m.from().as_ref() == props.next_from_square.as_ref()
        })
        .cloned()
        .collect::<Vec<_>>();

    // Storing the moves in a UseState hook for immutability
    let on_select = {
        let next_move = next_move.clone();
        Callback::from(move |m: shakmaty::Move| {
            next_move.set(Some(m));
        })
    };
    html! {
        <MultiSquaresPreview  next_move={(*next_move).clone()}
            moves={legal_moves.clone()} on_select={on_select.clone()} />
    }
}

#[derive(Properties, PartialEq)]
pub struct MultiSquaresPreviewProps {
    pub moves: Vec<shakmaty::Move>,
    pub next_move: Option<shakmaty::Move>,
    pub on_select: Callback<shakmaty::Move>,
}

#[function_component(MultiSquaresPreview)]
pub fn multi_squares_preview(props: &MultiSquaresPreviewProps) -> Html {
    let configs = use_context::<crate::contexts::configs::AnnotatorConfigStore>()
        .expect("AnnotatorConfigStore context not found");
    let orientation = match configs.playing_as {
        crate::configs::BoardPlayingSide::White => chessboard_js::ChessboardOrientation::White,
        crate::configs::BoardPlayingSide::Black => chessboard_js::ChessboardOrientation::Black,
    };
    use std::collections::HashSet;
    let board_ref = use_node_ref();
    let game_ctx = crate::live_game::use_annotated_game();
    let board_id = "multi-preview";
    let game_board = use_mut_ref(|| None::<chessboard_js::ChessBoardJs>);
    let to_squares: HashSet<shakmaty::Square> = props.moves.iter().map(|m| m.to()).collect();
    let on_select = props.on_select.clone();
    let moves = props.moves.clone();
    let role = props.moves.first().map(|m| m.role());
    let from_sq = props.moves.first().and_then(|m| m.from());
    let color = game_ctx.color_turn();

    let mut empty_board = shakmaty::Board::empty();
    if let (Some(role), Some(square)) = (role, from_sq) {
        empty_board.set_piece_at(square, shakmaty::Piece { color, role });
    }

    // Setting up the board
    {
        let board_setting = game_board.clone();
        use_effect_with((), move |()| {
            let board_fen = empty_board.board_fen(shakmaty::Bitboard::default());
            let board_options = chessboard_js::ChessboardConfig {
                draggable: false,
                position: chessboard_js::ChessboardPosition::Fen(board_fen.to_string()),
                orientation,
                ..Default::default()
            };
            let board = chessboard_js::ChessBoardJs::new(board_id, Some(board_options));
            *board_setting.borrow_mut() = Some(board);
            || {}
        });
    }
    // Highlighting squares and handle clicks
    {
        let board_ref = board_ref.clone();
        let to_squares = to_squares.clone();
        let on_select = on_select.clone();
        let moves = moves.clone();
        use_effect_with(
            (board_ref.clone(), moves.clone(), props.next_move.clone()),
            move |(board_ref, _, next_move)| {
                if let Some(root_ele) = board_ref.cast::<web_sys::HtmlElement>() {
                    for to_sq in &to_squares {
                        let class_name = format!("square-{}", to_sq);
                        let squares = root_ele.get_elements_by_class_name(&class_name);
                        for i in 0..squares.length() {
                            let Some(square_ele) = squares.item(i).and_then(|el| {
                                web_sys::wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(el)
                                    .ok()
                            }) else {
                                continue;
                            };
                            let style = square_ele.style();
                            if let Some(next_move) = next_move.as_ref() {
                                if next_move.to() == *to_sq {
                                    let _ =
                                        style.set_property("background-color", "hsl(74, 88%, 48%)");
                                } else {
                                    let _ = style
                                        .set_property("background-color", "hsl(247, 95%, 45%)");
                                }
                            } else {
                                let _ =
                                    style.set_property("background-color", "hsl(247, 95%, 45%)");
                            }
                            if let Some(m) = moves.iter().find(|m| m.to() == *to_sq) {
                                let on_select = on_select.clone();
                                let mv = m.clone();
                                let closure = web_sys::wasm_bindgen::closure::Closure::wrap(
                                    Box::new(move |_event: web_sys::MouseEvent| {
                                        on_select.emit(mv.clone());
                                    }) as Box<dyn FnMut(_)>,
                                )
                                .into_js_value();
                                square_ele.set_onclick(Some(
                                    web_sys::wasm_bindgen::JsCast::unchecked_ref(&closure),
                                ));
                            }
                        }
                    }
                }
                || {}
            },
        );
    }
    html! {
        <div ref={board_ref} id={board_id} class="h-[35vh] sm:h-[45vh] aspect-square mx-auto" />
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct PlayMoveButtonProps {
    pub next_move: UseStateHandle<Option<shakmaty::Move>>,
    pub next_role: UseStateHandle<Option<shakmaty::Role>>,
    pub next_from_square: UseStateHandle<Option<shakmaty::Square>>,
}

#[function_component(PlayMoveButton)]
pub fn play_move_button(props: &PlayMoveButtonProps) -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let PlayMoveButtonProps {
        next_move,
        next_role,
        next_from_square,
    } = props.clone();
    let language_ctx = crate::contexts::language::use_language_ctx();
    let onclick = {
        let game_ctx = game_ctx.clone();
        let next_move = next_move.clone();
        Callback::from(move |_| {
            if let Some(m) = next_move.as_ref() {
                game_ctx.dispatch(crate::live_game::AnnotatedGameAction::PlayMove(m.clone()));
            }
            next_move.set(None);
            next_role.set(None);
            next_from_square.set(None);
        })
    };
    html! {
        <shady_minions::ui::Button
                class="w-full max-w-sm mx-auto"
                variant={if next_move.is_some() {
                    shady_minions::ui::ButtonVariant::Normal
                } else {
                    shady_minions::ui::ButtonVariant::Disabled
                }}
                {onclick} >
                <span class="text-sm text-center mt-2 text-white">
                    { language_ctx.t("annotation_play_move") }
                </span>
            </shady_minions::ui::Button>
    }
}

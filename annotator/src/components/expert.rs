use shady_minions::ui::{Button, Input, InputType};
use yew::prelude::*;

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
        <>
            <AnnotationDisplay ..props.clone() />
            <AnnotationCalculator ..props />
        </>
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
        language_ctx.t("annotation_type_move").to_string()
    };

    html! {
        <div class="relative w-full">
            <Input
                class={classes!(
                    "w-full",
                    "min-h-18",
                    "pr-4", // Add padding on the right to prevent text overlap
                    "pl-4", // Add padding on the left
                    "py-12",
                    "rounded-lg",
                    "bg-muted-foreground"
                )}
                disabled={true}
                value={""} // Keep the actual input value empty
            />

            <div class="absolute inset-0 flex items-center px-8">
                // Left side - next move
                <div class="flex-shrink-0 mr-2">
                    <span class="text-2xl font-medium text-muted">
                        { next_move.as_ref().cloned().unwrap_or_default() }
                    </span>
                </div>

                // Right side - legal moves (with truncation)
                <div class="flex-grow flex justify-end">
                    <span class="text-2xl text-foreground truncate max-w-64">
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
    let input_class = classes!("h-fit", "text-2xl", "font-bold",);
    html! {
        <div class="">
            <div class="flex space-y-1 space-x-1">
            <Input
                class={input_class.clone()}
                disabled={
                    !legal_moves.iter()
                    .any(|m| {
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
            <div class="flex space-y-1 space-x-1">
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
            <div class="flex space-y-1 space-x-1">
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
            <div class="flex space-y-1 space-x-1">
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
                <lucide_yew::Delete class="size-8" />
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
                <lucide_yew::Send class="size-8" />
            </Button>
            </div>
        </div>
    }
}

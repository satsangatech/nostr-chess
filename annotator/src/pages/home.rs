use crate::components::UserProfileCard;
use crate::router::AnnotatorRoute;
use shady_minions::ui::{
    Button, Card, CardContent, CardHeader, CardTitle, Input, InputType, LeftDrawer, Modal, Switch,
    Tabs, TabsContent, TabsList, TabsTrigger,
};
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
                    <MoveList />
                    <TabsContent
                        class="flex flex-col h-full justify-between"
                        value={crate::configs::ExperienceLevel::Rookie.to_string()}>
                        <crate::components::RookieAnnotation />
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
                    // User profile card section
                    <UserProfileCard />

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
            class="flex flex-row p-3 items-center w-full mx-auto overflow-x-auto whitespace-nowrap gap-3 pb-2 max-w-sm min-h-12 bg-zinc-800 rounded-lg"
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

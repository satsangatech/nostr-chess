use crate::components::UserProfileCard;
use crate::router::AnnotatorRoute;
use shady_minions::ui::{
    Button, Card, CardContent, CardHeader, CardTitle, Input, InputType, LeftDrawer, Switch, Tabs,
    TabsContent, TabsList, TabsTrigger,
};
use wasm_bindgen::JsCast;
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

    let go_to_relay_management = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| navigator.push(&AnnotatorRoute::RelaySettings))
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
                    // Relay Status Section
                    <div class="rounded-lg bg-slate-50 p-3 sm:p-4 shadow-sm">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center">
                                <lucide_yew::Wifi class="w-4 h-4 sm:w-5 sm:h-5 mr-2 text-primary" />
                                <span class="text-sm sm:text-base font-medium">{"Relay Status"}</span>
                            </div>
                            <RelayStatusIcon />
                        </div>
                    </div>

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

                    <Button
                        onclick={go_to_relay_management}
                        class="w-full flex items-center justify-start py-2 px-3 sm:py-3 sm:px-4 h-auto bg-slate-100 border border-slate-200 text-sm sm:text-base shadow-sm break-words"
                        variant={shady_minions::ui::ButtonVariant::Outline}
                    >
                        <lucide_yew::Wifi class="w-4 h-4 sm:w-5 sm:h-5 mr-1.5 sm:mr-2 flex-shrink-0 text-primary" />
                        <span class="font-medium truncate">{"Relay Management"}</span>
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

    // Auto-open modal if this is a new game (both players are unnamed)
    let should_auto_open = game.white.is_empty() && game.black.is_empty();
    let is_open = use_state(|| should_auto_open);

    // Auto-close modal when both player names are filled in
    {
        let is_open = is_open.clone();
        let game = game.clone();
        use_effect_with(
            (game.white.clone(), game.black.clone()),
            move |(white, black)| {
                if !white.is_empty() && !black.is_empty() && *is_open {
                    // Small delay to let user see the form was submitted
                    yew::platform::spawn_local(async move {
                        gloo::timers::future::TimeoutFuture::new(1000).await;
                        is_open.set(false);
                    });
                }
                || ()
            },
        );
    }

    html! {
        <>
            <Button
                onclick={
                    let is_open = is_open.clone();
                    Callback::from(move |_| {
                        is_open.set(!*is_open);
                    })}>
                <lucide_yew::Cog class="size-4" />
            </Button>
            {
                if *is_open {
                    html! {
                        <div class="fixed inset-0 z-50 flex items-center justify-center">
                            <div class="fixed inset-0 bg-black/50" onclick={{
                                let is_open = is_open.clone();
                                Callback::from(move |_| is_open.set(false))
                            }}></div>
                            <div class="relative z-10 w-full max-w-md mx-4">
                                <GameDetailsForm game_ctx={game_ctx} />
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct GameDetailsFormProps {
    pub game_ctx: crate::live_game::AnnotatedGameStore,
}

#[function_component(GameDetailsForm)]
pub fn game_details_form(props: &GameDetailsFormProps) -> Html {
    let game_ctx = props.game_ctx.clone();
    let game = game_ctx.pgn_game();
    let language_ctx = crate::contexts::language::use_language_ctx();

    // State for event selection
    let event_options = ["Casual", "Tournament", "Match", "Simul", "Other"];

    let current_event = match &game.event {
        rooky_core::pgn_standards::PgnEvent::Casual => "Casual".to_string(),
        rooky_core::pgn_standards::PgnEvent::Named(name) => name.clone(),
        rooky_core::pgn_standards::PgnEvent::Unknown => "Casual".to_string(),
    };

    let selected_event = use_state(|| current_event.clone());
    let is_tournament = *selected_event != "Casual";

    html! {
        <Card class="w-full max-w-md">
            <CardHeader>
                <CardTitle>
                    { language_ctx.t("common_game_details") }
                </CardTitle>
            </CardHeader>
            <CardContent>
                <shady_minions::ui::Form
                    class="space-y-4"
                    onsubmit={{
                        let game_ctx = game_ctx.clone();
                        Callback::from(move |form: web_sys::HtmlFormElement| {
                            let white_input = form.get_with_name("white")
                                .and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok());
                            let black_input = form.get_with_name("black")
                                .and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok());
                            let date_input = form.get_with_name("date")
                                .and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok());
                            let event_input = form.get_with_name("event")
                                .and_then(|n| n.dyn_into::<web_sys::HtmlSelectElement>().ok());
                            let site_input = form.get_with_name("site")
                                .and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok());
                            let round_input = form.get_with_name("round")
                                .and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok());

                            if let Some(white) = white_input {
                                let white_value = white.value();
                                if !white_value.is_empty() {
                                    game_ctx.dispatch(crate::live_game::AnnotatedGameAction::AddWhiteName(white_value));
                                }
                            }
                            if let Some(black) = black_input {
                                let black_value = black.value();
                                if !black_value.is_empty() {
                                    game_ctx.dispatch(crate::live_game::AnnotatedGameAction::AddBlackName(black_value));
                                }
                            }
                            if let Some(date) = date_input {
                                if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(&date.value(), "%Y-%m-%d") {
                                    game_ctx.dispatch(crate::live_game::AnnotatedGameAction::ChangeDate(parsed_date));
                                }
                            }
                            if let Some(event) = event_input {
                                let event_value = event.value();
                                let site_value = site_input.map(|s| s.value()).unwrap_or_default();
                                let round_value = round_input.map(|r| r.value()).unwrap_or_default();

                                if event_value == "Casual" {
                                    game_ctx.dispatch(crate::live_game::AnnotatedGameAction::UpdateEventDetails {
                                        event: "Casual".to_string(),
                                        site: String::new(),
                                        round: String::new(),
                                    });
                                } else {
                                    game_ctx.dispatch(crate::live_game::AnnotatedGameAction::UpdateEventDetails {
                                        event: event_value,
                                        site: site_value,
                                        round: round_value,
                                    });
                                }
                            }
                        })
                    }}
                >
                    // White player name
                    <div class="space-y-2">
                        <label class="text-sm font-medium text-foreground">{ language_ctx.t("game_details_white") }</label>
                        <Input
                            name="white"
                            r#type={shady_minions::ui::InputType::Text}
                            placeholder={ language_ctx.t("game_details_enter_white_player") }
                            value={game.white.clone()}
                            class="w-full"
                        />
                    </div>

                    // Black player name
                    <div class="space-y-2">
                        <label class="text-sm font-medium text-foreground">{ language_ctx.t("game_details_black") }</label>
                        <Input
                            name="black"
                            r#type={shady_minions::ui::InputType::Text}
                            placeholder={ language_ctx.t("game_details_enter_black_player") }
                            value={game.black.clone()}
                            class="w-full"
                        />
                    </div>

                    // Date
                    <div class="space-y-2">
                        <label class="text-sm font-medium text-foreground">{ language_ctx.t("game_details_date") }</label>
                        <Input
                            name="date"
                            r#type={shady_minions::ui::InputType::Date}
                            value={game.date.format("%Y-%m-%d").to_string()}
                            class="w-full"
                        />
                    </div>

                    // Event dropdown
                    <div class="space-y-2">
                        <label class="text-sm font-medium text-foreground">{ language_ctx.t("game_details_event") }</label>
                        <select
                            name="event"
                            class="w-full px-3 py-2 border border-input rounded-md text-sm bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-ring"
                            onchange={{
                                let selected_event = selected_event.clone();
                                Callback::from(move |e: Event| {
                                    let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                    selected_event.set(select.value());
                                })
                            }}
                        >
                            { for event_options.iter().map(|option| {
                                html! {
                                    <option
                                        value={*option}
                                        selected={*option == current_event}
                                    >
                                        { *option }
                                    </option>
                                }
                            })}
                        </select>
                    </div>

                    // Conditional Site and Round fields (only show if not Casual)
                    if is_tournament {
                        <>
                            <div class="space-y-2">
                                <label class="text-sm font-medium text-foreground">{ language_ctx.t("game_details_site") }</label>
                                <Input
                                    name="site"
                                    r#type={shady_minions::ui::InputType::Text}
                                    placeholder={ language_ctx.t("game_details_enter_site") }
                                    value={match &game.site {
                                        rooky_core::pgn_standards::PgnSite::Named(name) => name.clone(),
                                        _ => String::new(),
                                    }}
                                    class="w-full"
                                />
                            </div>

                            <div class="space-y-2">
                                <label class="text-sm font-medium text-foreground">{ language_ctx.t("game_details_round") }</label>
                                <Input
                                    name="round"
                                    r#type={shady_minions::ui::InputType::Text}
                                    placeholder={ language_ctx.t("game_details_enter_round") }
                                    value={match &game.round {
                                        rooky_core::pgn_standards::PgnRound::Named(name) => name.clone(),
                                        _ => String::new(),
                                    }}
                                    class="w-full"
                                />
                            </div>
                        </>
                    }

                    // Submit button
                    <shady_minions::ui::Button
                        r#type={shady_minions::ui::ButtonType::Submit}
                        class="w-full mt-4"
                    >
                        { language_ctx.t("common_save") }
                    </shady_minions::ui::Button>
                </shady_minions::ui::Form>
            </CardContent>
        </Card>
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

#[function_component(RelayStatusIcon)]
pub fn relay_status_icon() -> Html {
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("missing relay context");
    let relay_status_state = use_state(Vec::new);
    let relay_set = relay_status_state.setter();

    use_effect_with(relay_ctx.clone(), move |relay| {
        let relay = relay.clone();
        yew::platform::spawn_local(async move {
            loop {
                gloo::timers::future::sleep(std::time::Duration::from_secs(2)).await;
                let status = relay.relay_health();
                relay_set.set(status.values().cloned().collect());
            }
        });
        || {}
    });

    let open_relays = relay_status_state
        .iter()
        .filter(|r| r == &&nostr_minions::relay_pool::ReadyState::OPEN)
        .count();

    let total_relays = relay_status_state.len();
    let is_connected = open_relays > 0;
    let connection_quality = if total_relays == 0 {
        "unknown"
    } else if open_relays == 0 {
        "disconnected"
    } else if open_relays < total_relays / 2 {
        "poor"
    } else if open_relays < total_relays {
        "good"
    } else {
        "excellent"
    };

    let (icon_color, status_text) = match connection_quality {
        "excellent" => (
            "text-green-500",
            format!("{}/{}", open_relays, total_relays),
        ),
        "good" => (
            "text-yellow-500",
            format!("{}/{}", open_relays, total_relays),
        ),
        "poor" => (
            "text-orange-500",
            format!("{}/{}", open_relays, total_relays),
        ),
        "disconnected" => ("text-red-500", "Offline".to_string()),
        _ => ("text-gray-400", "...".to_string()),
    };

    html! {
        <div class="flex items-center">
            <div class={classes!("w-2", "h-2", "rounded-full", "mr-2", if is_connected { "bg-green-500" } else { "bg-red-500" })}></div>
            <span class={classes!("text-xs", "sm:text-sm", icon_color)}>{status_text}</span>
        </div>
    }
}

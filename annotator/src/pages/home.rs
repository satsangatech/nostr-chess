use crate::components::UserProfileCard;
use crate::router::AnnotatorRoute;
use shady_minions::ui::{
    Button, Card, CardContent, CardHeader, CardTitle, Input, LeftDrawer, Modal, Select,
    SelectContent, SelectItem, SelectTrigger, Switch, Tabs, TabsContent, TabsList, TabsTrigger,
};

use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let config_ctx = crate::configs::use_annotator_config();
    let outcome_open = use_state(|| false);
    html! {
        <>
            <Tabs default_value={config_ctx.experience_level.as_ref().to_string()}
                class="flex flex-col w-full items-center">
                <HomeHeader />
                <div class="flex-1 flex flex-col w-full px-6">
                    <div class="flex gap-1 h-fit w-full items-center">
                        <TakeBackButton />
                        <MoveList />
                        <Button
                            onclick={
                                let outcome_open = outcome_open.clone();
                                Callback::from(move |_: MouseEvent| {
                                    outcome_open.set(!*outcome_open);
                                })
                            }
                            size={shady_minions::ui::ButtonSize::Icon}
                            class="bg-transparent">
                            <lucide_yew::Handshake class="size-7" />
                        </Button>
                    </div>
                    <div class="h-[0.5px] bg-muted my-3 w-full px-3 sm:px-6 rounded-lg" />
                    <TabsContent
                        class="flex flex-col justify-between"
                        value={crate::configs::ExperienceLevel::Rookie.as_ref()}>
                        <crate::components::RookieAnnotation />
                    </TabsContent>
                    <TabsContent
                        class="flex-1 flex flex-col justify-end gap-6"
                        value={crate::configs::ExperienceLevel::Expert.as_ref()}>
                        <crate::components::ExpertAnnotation />
                    </TabsContent>
                </div>
            </Tabs>
            <Modal is_open={outcome_open} >
                <OutcomeForm />
            </Modal>
        </>
    }
}

#[function_component(TakeBackButton)]
pub fn take_back_button() -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let onclick = {
        let game_ctx = game_ctx.clone();
        Callback::from(move |_| {
            game_ctx.dispatch(crate::live_game::AnnotatedGameAction::TakeBack);
        })
    };
    html! {
        <shady_minions::ui::Button
            size={shady_minions::ui::ButtonSize::Icon}
            class="bg-transparent"
            // variant={shady_minions::ui::ButtonVariant::Normal}
            {onclick}>
            <lucide_yew::Undo2
                class="size-7 text-destructive" />
            // <span class="text-sm text-center">
            //     { language_ctx.t("annotation_take_back") }
            // </span>
        </shady_minions::ui::Button>
    }
}

const OUTCOMES: [shakmaty::Outcome; 3] = [
    shakmaty::Outcome::Draw,
    shakmaty::Outcome::Decisive {
        winner: shakmaty::Color::White,
    },
    shakmaty::Outcome::Decisive {
        winner: shakmaty::Color::Black,
    },
];
#[function_component(OutcomeForm)]
pub fn outcome_form() -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let language_ctx = crate::contexts::language::use_language_ctx();
    let outcome_state = use_state(|| shakmaty::Outcome::Draw);
    let navigator = use_navigator().unwrap();

    html! {
        <Card class="w-full max-w-sm">
            <CardHeader>
                <CardTitle>
                    { language_ctx.t("common_outcome") }
                </CardTitle>
            </CardHeader>
            <CardContent>
                <shady_minions::ui::Form
                    class="space-y-4"
                    onsubmit={{
                        let game_ctx = game_ctx.clone();
                        let outcome_state = outcome_state.clone();
                        let navigator = navigator.clone();
                        Callback::from(move |_: web_sys::HtmlFormElement| {
                            game_ctx.dispatch(crate::live_game::AnnotatedGameAction::AddOutcome(
                                *outcome_state,
                            ));
                            navigator.push(&AnnotatorRoute::Review);

                        })
                    }}
                >
                    <Select::<shakmaty::Outcome>
                        name="outcome"
                        onchange={{
                            let outcome_state = outcome_state.setter();
                            Callback::from(move |value: Option<shakmaty::Outcome>| {
                                if let Some(outcome) = value {
                                    outcome_state.set(outcome);
                                }
                            })
                        }}
                        >
                        <SelectTrigger::<shakmaty::Outcome> class="w-full" />
                        <SelectContent::<shakmaty::Outcome>>
                            { for OUTCOMES.iter().map(|outcome| {
                                html! {
                                    <SelectItem::<shakmaty::Outcome> value={*outcome}
                                        label={match *outcome {
                                            shakmaty::Outcome::Draw => language_ctx.t("common_draw"),
                                            shakmaty::Outcome::Decisive { winner } => {
                                                if winner == shakmaty::Color::White {
                                                    language_ctx.t("common_white_wins")
                                                } else {
                                                    language_ctx.t("common_black_wins")
                                                }
                                            }
                                        }}
                                    />
                                }
                            }) }
                        </SelectContent::<shakmaty::Outcome>>
                    </Select::<shakmaty::Outcome>>

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

#[function_component(HomeHeader)]
pub fn home_header() -> Html {
    html! {
        <header class="flex justify-between items-center px-6 gap-2 w-full mb-6">
            <SettingsDrawer />
            <ExperienceSelector />
            <GameDetailsModal />
        </header>
    }
}

#[function_component(SettingsDrawer)]
pub fn settings_drawer() -> Html {
    let is_open = use_state(|| false);
    let pubkey = nostr_minions::key_manager::use_nostr_pubkey();
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

    let set_playing_side = {
        let config_ctx = config_ctx.clone();
        move |playing_as: crate::contexts::configs::BoardPlayingSide| {
            config_ctx.dispatch(
                crate::contexts::configs::AnnotatorConfigAction::SetPlayingAs(playing_as),
            );
        }
    };

    let is_expert =
        config_ctx.experience_level == crate::contexts::configs::ExperienceLevel::Expert;

    let language_ctx = crate::contexts::language::use_language_ctx();

    html! {
        <>
            <Button {onclick}
                variant={shady_minions::ui::ButtonVariant::Outline}
                size={shady_minions::ui::ButtonSize::Icon}>
                <lucide_yew::Menu class="size-7" />
            </Button>
            <LeftDrawer {is_open}  >
                <h3 class="text-lg sm:text-xl font-semibold text-foreground my-6 ml-6">{language_ctx.t("common_settings")}</h3>
                <div class="space-y-4 w-full ml-6 max-w-4/5">
                    // User profile card section
                    <div class="flex flex-col gap-1">
                        <Button
                            onclick={go_to_key_recovery}
                            size={shady_minions::ui::ButtonSize::Small}
                            variant={shady_minions::ui::ButtonVariant::Outline}
                        >
                            <lucide_yew::Key class="w-4 h-4 sm:w-5 sm:h-5 mr-1.5 sm:mr-2 flex-shrink-0 text-secondary" />
                            <span class="font-medium truncate text-sm">{ language_ctx.t("settings_key_recovery") }</span>
                        </Button>

                        <Button
                            onclick={go_to_relay_management}
                            size={shady_minions::ui::ButtonSize::Small}
                            variant={shady_minions::ui::ButtonVariant::Outline}
                        >
                            <lucide_yew::Wifi class="w-4 h-4 sm:w-5 sm:h-5 mr-1.5 sm:mr-2 flex-shrink-0 text-secondary" />
                            <span class="font-medium truncate text-sm">{"Relay Management"}</span>
                        </Button>
                    </div>
                    <div class="border border-secondary w-full max-w-sm mx-auto my-6" />
                    <UserProfileCard />
                    <div class="rounded-lg shadow-sm">
                        <div class="flex items-center justify-between gap-2">
                            <div class="flex items-center overflow-hidden">
                                <lucide_yew::KeyRound class="size-4 min-w-4 min-h-4 mr-2 text-secondary" />
                                <span class="text-sm sm:text-base font-medium text-muted truncate">{pubkey.unwrap_or_default()}</span>
                            </div>
                            <lucide_yew::Copy class="size-4 min-w-4 min-h-4 mr-2 text-secondary" />
                        </div>
                    </div>
                    // Relay Status Section
                    <div class="rounded-lg shadow-sm">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center">
                                <lucide_yew::Wifi class="w-4 h-4 sm:w-5 sm:h-5 mr-2 text-secondary" />
                                <span class="text-sm sm:text-base font-medium text-muted ">{"Relay Status"}</span>
                            </div>
                            <RelayStatusIcon />
                        </div>
                    </div>
                    <div class="border border-secondary w-full max-w-sm mx-auto my-6" />

                    <div class="rounded-lg shadow-sm">
                        <div class="mb-1.5 sm:mb-2">
                            <label class="text-sm sm:text-base font-medium block mb-0.5 sm:mb-1 text-muted">
                                { language_ctx.t("settings_default_level") }
                            </label>
                            <p class="text-xs sm:text-sm text-muted-foreground">
                                { language_ctx.t("settings_default_level_description") }
                            </p>
                        </div>
                        <div class="mt-2">
                            <div class="flex items-center justify-between px-1">
                                <div class="flex items-center">
                                    <span class="text-sm font-medium mr-2 sm:mr-3 text-muted">{ language_ctx.t("common_rookie") }</span>
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
                                <span class="text-sm font-medium ml-2 sm:ml-3 text-muted">{ language_ctx.t("common_expert") }</span>
                            </div>
                        </div>
                    </div>

                    <div class="rounded-lg shadow-sm">
                        <div class="mb-1.5 sm:mb-2">
                            <label class="text-sm sm:text-base font-medium block mb-0.5 sm:mb-1 text-muted">
                                { language_ctx.t("settings_default_orientation") }
                            </label>
                            <p class="text-xs sm:text-sm text-muted-foreground">
                                { language_ctx.t("settings_default_orientation_description") }
                            </p>
                        </div>
                        <div class="mt-2">
                            <div class="flex items-center justify-between px-1">
                                <div class="flex items-center">
                                    <span class="text-sm font-medium mr-2 sm:mr-3 text-muted">{ language_ctx.t("common_white") }</span>
                                    <Switch
                                        checked={config_ctx.playing_as == crate::contexts::configs::BoardPlayingSide::Black}
                                        onchange={
                                            let set_playing_side = set_playing_side.clone();
                                            Callback::from(move |checked: bool| {
                                                if checked {
                                                    set_playing_side(crate::contexts::configs::BoardPlayingSide::Black);
                                                } else {
                                                    set_playing_side(crate::contexts::configs::BoardPlayingSide::White);
                                                }
                                            })
                                        }
                                    />
                                </div>
                                <span class="text-sm font-medium ml-2 sm:ml-3 text-muted">{ language_ctx.t("common_black") }</span>
                            </div>
                        </div>
                    </div>
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
                <TabsTrigger value={crate::configs::ExperienceLevel::Rookie.as_ref()}>
                    { language_ctx.t("common_rookie") }
                </TabsTrigger>
                <TabsTrigger value={crate::configs::ExperienceLevel::Expert.as_ref()}>
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
    let should_auto_open = game.white.is_empty() && game.black.is_empty() && game.moves.is_empty();
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
    let close_on_click = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(!*is_open);
        })
    };

    html! {
        <>
            <Button onclick={close_on_click}
                variant={shady_minions::ui::ButtonVariant::Outline}
                size={shady_minions::ui::ButtonSize::Icon}>
                <lucide_yew::SquarePen class="size-7" />
            </Button>
            <Modal {is_open}>
                <GameDetailsForm game_ctx={game_ctx} />
            </Modal>
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

    let selected_event = use_state(|| game.event.clone());
    let is_tournament = matches!(
        *selected_event,
        rooky_core::pgn_standards::PgnEvent::Named(_)
    );

    html! {
        <Card class="w-full max-w-md max-h-[76vh] overflow-y-auto">
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
                            id="form-details-white"
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
                            id="form-details-black"
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
                            id="form-details-date"
                            name="date"
                            r#type={shady_minions::ui::InputType::Date}
                            value={game.date.format("%Y-%m-%d").to_string()}
                            class="w-full"
                        />
                    </div>

                    <div class="space-y-2">
                        <label class="text-sm font-medium text-foreground">{ language_ctx.t("game_details_event") }</label>
                        <Input
                            id="form-details-event"
                            name="event"
                            value={match &*selected_event {
                                rooky_core::pgn_standards::PgnEvent::Named(name) => name.clone(),
                                _ => String::new(),
                            }}
                            oninput={{
                                let selected_event = selected_event.clone();
                                Callback::from(move |e: String| {
                                    if e.is_empty() {
                                        selected_event.set(rooky_core::pgn_standards::PgnEvent::Casual);
                                        return;
                                    }
                                    selected_event.set(rooky_core::pgn_standards::PgnEvent::Named(e));
                                })
                            }}
                        />
                    </div>

                    // Conditional Site and Round fields (only show if not Casual)
                    if is_tournament {
                        <>
                            <div class="space-y-2">
                                <label class="text-sm font-medium text-foreground">{ language_ctx.t("game_details_site") }</label>
                                <Input
                                    id="form-details-site"
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
                                    id="form-details-round"
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
            class="flex flex-row p-3 items-center w-full overflow-x-auto whitespace-nowrap gap-3 pb-2 max-w-sm min-h-12 bg-background rounded-lg"
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
                    "text-lg font-semibold text-muted"
                } else {
                    "text-sm, text-muted"
                };

                html! {
                    <div class="inline-flex items-center">
                        <span class="text-secondary-foreground mr-1 text-sm">{ format!("{}.", move_number) }</span>
                        <span class={classes!(white_class, "mr-1", "text-muted")}>{ white_move.to_string() }</span>
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
